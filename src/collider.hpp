#pragma once

#include <glm/glm.hpp>
#include <glm/gtc/quaternion.hpp>

struct AABB {
  glm::vec3 low;
  glm::vec3 high;
};

struct Ray {
  glm::vec3 start;
  glm::vec3 dir;
};

class Collider {
public:
  virtual AABB aabb(glm::vec3 position, glm::quat rotation) const = 0;
  virtual float ray_hit(glm::vec3 position, glm::quat rotation, Ray ray) const = 0;
  virtual ~Collider() {};
};

class BoxCollider : public Collider {
  glm::vec3 size;
public:
  BoxCollider(glm::vec3 size);
  virtual AABB aabb(glm::vec3 position, glm::quat rotation) const override;
  virtual float ray_hit(glm::vec3 position, glm::quat rotation, Ray ray) const override;
};

class SphereCollider : public Collider {
  float radius;
public:
  SphereCollider(float radius);
  virtual AABB aabb(glm::vec3 position, glm::quat rotation) const override;
  virtual float ray_hit(glm::vec3 position, glm::quat rotation, Ray ray) const override;
};
