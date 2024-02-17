#pragma once

#include <glm/glm.hpp>
#include <glm/gtc/quaternion.hpp>

struct AABB {
  glm::vec3 low;
  glm::vec3 high;
};

class Collider {
public:
  virtual AABB aabb(glm::vec3 position, glm::quat rotation) const = 0;
  virtual ~Collider() {};
};

class BoxCollider : public Collider {
  glm::vec3 size;
public:
  BoxCollider(glm::vec3 size);
  virtual AABB aabb(glm::vec3 position, glm::quat rotation) const override;
};

class SphereCollider : public Collider {
  float radius;
public:
  SphereCollider(float radius);
  virtual AABB aabb(glm::vec3 position, glm::quat rotation) const override;
};
