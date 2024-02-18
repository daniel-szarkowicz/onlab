#include "engine/context.hpp"
#include "engine/imgui/imgui.h"
#include <GL/glew.h>
#include <GLFW/glfw3.h>
#include <algorithm>
#include <glm/common.hpp>
#include <glm/ext/matrix_transform.hpp>
#include <glm/ext/quaternion_common.hpp>
#include <glm/ext/quaternion_trigonometric.hpp>
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
#include "shader.hpp"

int main() {
  Context::init(720, 1280, "Rigid body szimuláció");
  glEnable(GL_CULL_FACE);

  std::vector<Object> objects;
  for (int x = -5; x < 5; ++x) {
    for (int y = -5; y < 5; ++y) {
      for (int z = -5; z < 5; ++z) {
        if ((x + y + z) % 2) objects.push_back(Object::box(glm::vec3(1,1,1)));
        else objects.push_back(Object::sphere(1));
        objects.back().position = glm::vec3(2*x, 2*y, 2*z);
      }
    }
  }
  {
    auto ground = Object::box(glm::vec3(10000, 1, 10000));
    ground.position = glm::vec3(0, -21, 0);
    ground.immovable = true;
    objects.push_back(ground);
  }
  GeometryShader geometry_shader;
  AABBShader aabb_shader;

  bool mousegrab = false;
  bool paused = true;
  bool show_objects = true;
  bool show_bounds = false;

  auto camera = FirstPersonCamera({0, 1.5, 0}, 0, 0);

  Context::loop([&]() {
    glClearColor(0.3, 0.6, 0.9, 1.0);
    glClear(GL_COLOR_BUFFER_BIT);
    ImGui::Begin("Settings");
    ImGui::Checkbox("Pause", &paused);
    ImGui::Checkbox("Show objects", &show_objects);
    ImGui::Checkbox("Show bounds", &show_bounds);
    if (mousegrab) {
      ImGui::Text("Press Escape to release mouse");
    } else {
      ImGui::Text("Press Escape to grab mouse");
    }
    ImGui::Text("FPS: %2.2f", Context::fps());
    ImGui::Text("Delta: %2.2f", Context::delta());
    ImGui::End();

    float speed = 3;
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

    glm::vec3 dir(0, 0, 0);
    if (Context::key_pressed[GLFW_KEY_W])          dir.x += 1;
    if (Context::key_pressed[GLFW_KEY_S])          dir.x -= 1;
    if (Context::key_pressed[GLFW_KEY_A])          dir.z -= 1;
    if (Context::key_pressed[GLFW_KEY_D])          dir.z += 1;
    if (Context::key_pressed[GLFW_KEY_LEFT_SHIFT]) dir.y -= 1;
    if (Context::key_pressed[GLFW_KEY_SPACE])      dir.y += 1;
    if (glm::length(dir) > 0) {
      camera.move_facing(glm::normalize(dir) * speed * Context::delta());
    }

    for (auto& object : objects) {
      if (!object.immovable) {
        auto rotation = glm::angleAxis(glm::radians(20.0f)*Context::delta(), glm::normalize(object.position));
        object.rotation *= rotation;
      }
    }

    if (show_objects) geometry_shader.drawObjects(camera, objects);
    glLineWidth(3);
    if (show_bounds) aabb_shader.drawObjects(camera, objects);
  });

  Context::uninit();
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
