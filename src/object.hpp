#pragma once

#include "geometry.hpp"
#include <glm/glm.hpp>
#include <glm/gtc/quaternion.hpp>
#include "collider.hpp"

class Object {
public:
  Geometry geometry;
  Collider* collider;
  glm::vec3 position;
  glm::quat rotation;
  glm::vec3 momentum;
  glm::vec3 angular_momentum;
  float mass;
  glm::vec3 inertia;
  bool immovable;

  Object(Geometry geometry, Collider* collider);

  AABB aabb();
  static Object box(glm::vec3 size);
  static Object sphere(float radius);

  ~Object();
};
