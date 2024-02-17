#pragma once

#include "geometry.hpp"
#include <glm/glm.hpp>
#include <memory>

class Object {
private:
  std::shared_ptr<Geometry> geometry;
public:
  glm::vec3 position; // distance
  glm::vec3 scale;
  glm::vec3 momentum; // distance / time * mass
  glm::vec3 force; // distance / time^2 * mass
  float mass; // mass
  // glm::quat rotation;
  // glm::vec3 angular_momentum;
public:
  Object(
         std::shared_ptr<Geometry> geometry,
         glm::vec3 position = glm::vec3(0, 0, 0),
         glm::vec3 scale = glm::vec3(1, 1, 1),
         float mass = 1000 // 1000kg ≅ 1 cubic meter of water
       );

  void draw(GLuint model_uniform_location, GLuint model_inv_uniform_location);

  void update(float dt);
};
