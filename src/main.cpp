#include "engine/context.hpp"
#include "engine/imgui/imgui.h"
#include <GL/glew.h>
#include <GLFW/glfw3.h>
#include <algorithm>
#include <glm/common.hpp>
#include <iostream>
#include <glm/glm.hpp>
#include <glm/gtc/type_ptr.hpp>
#include "engine/camera.hpp"
#include <GLFW/glfw3.h>

GLuint create_shader_program();

int main() {
  Context::init(720, 1280, "Rigid body szimuláció");

  auto shader_program = create_shader_program();

  int counter = 0;
  auto model = glm::mat4();
  glProgramUniformMatrix4fv(shader_program, glGetUniformLocation(shader_program, "model"), 1, GL_FALSE, glm::value_ptr(model));
  auto view_uniform = glGetUniformLocation(shader_program, "view");
  auto projection_uniform = glGetUniformLocation(shader_program, "projection");

  GLuint vao;
  GLuint vbo;
  glCreateVertexArrays(1, &vao);
  glCreateBuffers(1, &vbo);
  glBindVertexArray(vao);
  glBindBuffer(GL_ARRAY_BUFFER, vbo);

  glEnableVertexAttribArray(0);
  glVertexAttribPointer(0, 3, GL_FLOAT, GL_TRUE, 6*sizeof(float), NULL);
  glEnableVertexAttribArray(1);
  glVertexAttribPointer(1, 3, GL_FLOAT, GL_TRUE, 6*sizeof(float), (void*)(3*sizeof(float)));

  glm::vec3 points[][2] = {
    {{100, 0, 100}, {1, 1, 1}},
    {{100, 0, -100}, {1, 1, 1}},
    {{-100, 0, 100}, {1, 1, 1}},
    {{-100, 0, -100}, {1, 1, 0}},
    {{-100, 0, 100}, {1, 1, 0}},
    {{100, 0, -100}, {1, 1, 0}},
    {{1, 1, -1}, {1, 0, 0}},
    {{1, -1, 1}, {1, 0, 0}},
    {{-1, 1, 1}, {1, 0, 0}},
  };
  glNamedBufferData(vbo, sizeof(points), points, GL_STATIC_DRAW);
  bool mousegrab = false;

  auto camera = FirstPersonCamera({0, 1.5, 0}, 0, 0);

  Context::loop([&]() {
    glClearColor(0.3, 0.6, 0.9, 1.0);
    glClear(GL_COLOR_BUFFER_BIT);
    ImGui::Begin("Settings");
    if (ImGui::Button("Hello World!"))
      ++counter;
    ImGui::Text("%d", counter);
    ImGui::DragFloat3("camera position", glm::value_ptr(camera.pos));
    ImGui::DragFloat("camera yaw", &camera.yaw);
    ImGui::DragFloat("camera pitch", &camera.pitch);
    if (mousegrab) {
      ImGui::Text("Press Escape to release mouse");
    } else {
      ImGui::Text("Press Escape to grab mouse");
    }
    ImGui::Text("Mouse x: %f", Context::mouse_x);
    ImGui::Text("Mouse y: %f", Context::mouse_y);
    ImGui::Text("Mouse dx: %f", Context::mouse_dx);
    ImGui::Text("Mouse dy: %f", Context::mouse_dy);
    ImGui::End();

    float speed = 1.0/60.0 * 3; // 3 m/s
    if (Context::key_pressed[GLFW_KEY_LEFT_CONTROL]) {
      speed *= 2;
    }

    if (Context::key_pressed[GLFW_KEY_CAPS_LOCK]) {
      if (mousegrab) {
        Context::release_mouse();
        mousegrab = false;
      } else {
        Context::grab_mouse();
        mousegrab = true;
      }
    }

    if (mousegrab) {
      camera.yaw -= Context::mouse_dx / 10;
      camera.pitch -= Context::mouse_dy / 10;
      camera.pitch = std::clamp(camera.pitch, -89.99999f, 89.99999f);
    }

    if (Context::key_pressed[GLFW_KEY_W]) {
      camera.move_forward(speed);
    }
    if (Context::key_pressed[GLFW_KEY_S]) {
      camera.move_backward(speed);
    }
    if (Context::key_pressed[GLFW_KEY_A]) {
      camera.move_left(speed);
    }
    if (Context::key_pressed[GLFW_KEY_D]) {
      camera.move_right(speed);
    }
    if (Context::key_pressed[GLFW_KEY_LEFT_SHIFT]) {
      camera.move_down(speed);
    }
    if (Context::key_pressed[GLFW_KEY_SPACE]) {
      camera.move_up(speed);
    }

    glProgramUniformMatrix4fv(shader_program, view_uniform, 1, GL_FALSE, glm::value_ptr(camera.view()));
    glProgramUniformMatrix4fv(shader_program, projection_uniform, 1, GL_FALSE, glm::value_ptr(camera.projection()));

    glBindVertexArray(vao);
    glUseProgram(shader_program);
    glDrawArrays(GL_TRIANGLES, 0, sizeof(points)/sizeof(float));
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

    in vec3 position;
    in vec3 color;

    out vec3 fColor;

    void main() {
      gl_Position = projection * view * vec4(position, 1);
      fColor = color;
    }
  )";
  glShaderSource(vertex_shader, 1, &vertex_source, NULL);
  glCompileShader(vertex_shader);
  glAttachShader(shader_program, vertex_shader);

  auto fragment_shader = glCreateShader(GL_FRAGMENT_SHADER);
  auto fragment_source = R"(
    #version 400

    in vec3 fColor;

    out vec4 frag_color;

    void main() {
        frag_color = vec4(fColor, 1);
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
