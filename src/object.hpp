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

  glm::vec3 force;

  Object(Geometry geometry, Collider* collider);

  AABB aabb() const;
  float ray_hit(Ray ray) const;

  void apply_force(glm::vec3 position, glm::vec3 force);
  void reset_force();
  void update(float dt);

  static Object box(glm::vec3 size);
  static Object sphere(float radius);

  ~Object();
};
