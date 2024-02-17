#include "engine/context.hpp"
#include "engine/imgui/imgui.h"
#include <GL/glew.h>
#include <GLFW/glfw3.h>
#include <algorithm>
#include <glm/common.hpp>
#include <glm/ext/matrix_transform.hpp>
#include <glm/ext/quaternion_common.hpp>
#include <glm/geometric.hpp>
#include <iostream>
#include <glm/glm.hpp>
#include <glm/gtc/type_ptr.hpp>
#include "engine/camera.hpp"
#include <GLFW/glfw3.h>
#include <memory>
#include <vector>
#include "geometry.hpp"
#include "object.hpp"

GLuint create_shader_program();

const glm::vec3 GRAVITY(0, -9.80665, 0);

int main() {
  Context::init(720, 1280, "Rigid body szimuláció");
  glEnable(GL_CULL_FACE);

  auto shader_program = create_shader_program();

  auto model_uniform = glGetUniformLocation(shader_program, "model");
  auto model_inv_uniform = glGetUniformLocation(shader_program, "model_inv");
  auto view_uniform = glGetUniformLocation(shader_program, "view");
  auto projection_uniform = glGetUniformLocation(shader_program, "projection");

  auto cubeGeometry = std::make_shared<CubeGeometry>();
  std::vector<Object> cubes;
  for (int x = -10; x < 10; ++x) {
    for (int y = -10; y < 10; ++y) {
      for (int z = -10; z < 10; ++z) {
        cubes.emplace_back(cubeGeometry, glm::vec3(x * 2, y * 2, z * 2));
      }
    }
  }
  Object ground(cubeGeometry, glm::vec3(0, -21, 0), glm::vec3(100, 1, 100));

  bool mousegrab = false;
  bool paused = true;

  auto camera = FirstPersonCamera({0, 1.5, 0}, 0, 0);

  Context::loop([&]() {
    glClearColor(0.3, 0.6, 0.9, 1.0);
    glClear(GL_COLOR_BUFFER_BIT);
    ImGui::Begin("Settings");
    ImGui::Checkbox("Pause", &paused);
    ImGui::DragFloat3("camera position", glm::value_ptr(camera.pos));
    ImGui::DragFloat("camera yaw", &camera.yaw);
    ImGui::DragFloat("camera pitch", &camera.pitch);
    if (mousegrab) {
      ImGui::Text("Press Escape to release mouse");
    } else {
      ImGui::Text("Press Escape to grab mouse");
    }
    ImGui::Text("FPS: %2.2f", Context::fps());
    ImGui::End();

    for (auto& cube : cubes) {
      cube.force += GRAVITY * cube.mass;
    }

    if (!paused) {
      for (auto& cube : cubes) {
        cube.update(Context::delta());
      }
    }

    for (auto& cube : cubes) {
      cube.force = glm::vec3(0, 0, 0);
    }

    double speed = 3;
    if (Context::key_pressed[GLFW_KEY_LEFT_CONTROL]) {
      speed *= 2;
    }

    if (Context::key_just_pressed[GLFW_KEY_CAPS_LOCK]) {
      if (mousegrab) {
        Context::release_mouse();
        mousegrab = false;
      } else {
        Context::grab_mouse();
        mousegrab = true;
      }
    }

    if (mousegrab) {
      camera.yaw -= Context::mouse_position_change().x / 10;
      camera.pitch -= Context::mouse_position_change().y / 10;
      camera.pitch = std::clamp(camera.pitch, -89.999f, 89.999f);
    }

    glm::vec<3, double> dir(0, 0, 0);
    if (Context::key_pressed[GLFW_KEY_W])          dir.x += 1;
    if (Context::key_pressed[GLFW_KEY_S])          dir.x -= 1;
    if (Context::key_pressed[GLFW_KEY_A])          dir.z -= 1;
    if (Context::key_pressed[GLFW_KEY_D])          dir.z += 1;
    if (Context::key_pressed[GLFW_KEY_LEFT_SHIFT]) dir.y -= 1;
    if (Context::key_pressed[GLFW_KEY_SPACE])      dir.y += 1;
    if (glm::length(dir) > 0) {
      camera.move_facing(glm::normalize(dir) * speed * Context::delta());
    }

    glProgramUniformMatrix4fv(shader_program, view_uniform, 1, GL_FALSE, glm::value_ptr(camera.view()));
    glProgramUniformMatrix4fv(shader_program, projection_uniform, 1, GL_FALSE, glm::value_ptr(camera.projection()));

    glUseProgram(shader_program);
    for (auto& cube : cubes) {
      cube.draw(model_uniform, model_inv_uniform);
    }
    ground.draw(model_uniform, model_inv_uniform);
  });

  Context::uninit();
}

GLuint create_shader_program() {
  auto shader_program = glCreateProgram();

  auto vertex_shader = glCreateShader(GL_VERTEX_SHADER);
  auto vertex_source = R"(
    #version 400

    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 projection;
    uniform mat4 model_inv;

    layout(location = 0) in vec4 vertexPosition;
    layout(location = 1) in vec3 vertexNormal;

    out vec3 normal;
    out vec3 position;

    void main() {
      gl_Position = projection * view * model * vertexPosition;
      position = vec3(model * vertexPosition);
      normal = normalize(vec3(vec4(vertexNormal, 0) * model_inv));
    }
  )";
  glShaderSource(vertex_shader, 1, &vertex_source, NULL);
  glCompileShader(vertex_shader);
  glAttachShader(shader_program, vertex_shader);

  auto fragment_shader = glCreateShader(GL_FRAGMENT_SHADER);
  auto fragment_source = R"(
    #version 400

    in vec3 normal;
    in vec3 position;

    out vec4 frag_color;

    void main() {
        vec3 light_dir = vec3(-0.7, 1, 0.5);
        float ambient_lightness = 0.3;
        float bright_lightness = 0.7 * max(0, dot(light_dir, normalize(normal)));
        float lightness = ambient_lightness + bright_lightness;
        vec3 color = vec3(1, 1, 1) * lightness;
        frag_color = vec4(color, 1);
    }
  )";
  glShaderSource(fragment_shader, 1, &fragment_source, NULL);
  glCompileShader(fragment_shader);
  glAttachShader(shader_program, fragment_shader);

  glLinkProgram(shader_program);
  return shader_program;
}
/*
rigidbodies:
  draw (player perspective + shadows)
  simulate

draw loop:
  clear frame buffer and shadow buffer
  reset vertex buffer
  for each body:
    add to vertex buffer
    if vertex buffer is full:
      draw to frame buffer and shadown buffer !!!! this is wrong (except if
shadows are drawn in post processing) reset vertex buffer post processing: apply
shadows to picture (broken if transparency is used)

alternate draw loop:
  clear shadow buffer
  reset vertex buffer
  for each body:
    add to vertex buffer
    if vertex buffer is full:
      draw to shadow buffer
      reset vertex buffer
  clear frame buffer
  reset vertex buffer
  for each body:
    add to vertex buffer
    if vertex buffer is full:
      draw to frame buffer (use shadow buffer for lighting)
      reset vertex buffer
*/
