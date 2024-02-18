#include "collider.hpp"
#include <glm/gtc/quaternion.hpp>

BoxCollider::BoxCollider(glm::vec3 size)
  : size(size) {}

AABB BoxCollider::aabb(glm::vec3 position, glm::quat rotation) const {
  AABB aabb = { position, position };
  glm::mat4 rot = glm::mat4_cast(rotation);
  for (int x = -1; x <= 1; x += 2) {
    for (int y = -1; y <= 1; y += 2) {
      for (int z = -1; z <= 1; z += 2) {
        auto p = glm::vec3(x, y, z) * size/2.0f;
        p = glm::vec3(rot * glm::vec4(p, 1)) + position;
        aabb.low = glm::min(aabb.low, p);
        aabb.high = glm::max(aabb.high, p);
      }
    }
  }
  return aabb;
}

SphereCollider::SphereCollider(float radius)
  :radius(radius) {}

AABB SphereCollider::aabb(glm::vec3 position, glm::quat) const {
  AABB aabb;
  auto offset = glm::vec3(radius);
  aabb.low = position - offset;
  aabb.high = position + offset;
  return aabb;
}
