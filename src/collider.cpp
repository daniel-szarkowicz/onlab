#include "collider.hpp"
#include <glm/ext/matrix_transform.hpp>
#include <glm/gtc/quaternion.hpp>
#include <iostream>

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

float BoxCollider::ray_hit(glm::vec3 position, glm::quat rotation, Ray ray) const {
  // TODO
  glm::vec3 faces[6][4] = {
    {
      {0.5, 0.5, 0.5},
      {-.5, 0.5, 0.5},
      {-.5, -.5, 0.5},
      {0.5, -.5, 0.5},
    },

    {
      {-.5, 0.5, -.5},
      {0.5, 0.5, -.5},
      {0.5, -.5, -.5},
      {-.5, -.5, -.5},
    },

    {
      {0.5, 0.5, 0.5},
      {0.5, -.5, 0.5},
      {0.5, -.5, -.5},
      {0.5, 0.5, -.5},
    },

    {
      {-.5, -.5, 0.5},
      {-.5, 0.5, 0.5},
      {-.5, 0.5, -.5},
      {-.5, -.5, -.5},
    },

    {
      {0.5, 0.5, 0.5},
      {0.5, 0.5, -.5},
      {-.5, 0.5, -.5},
      {-.5, 0.5, 0.5},
    },

    {
      {0.5, -.5, -.5},
      {0.5, -.5, 0.5},
      {-.5, -.5, 0.5},
      {-.5, -.5, -.5},
    },
  };
  auto transform = glm::scale(glm::mat4(1), size);
  transform = glm::mat4_cast(rotation) * transform;
  transform = glm::translate(glm::mat4(1), position) * transform;
  float best = -1;
  for (auto& face : faces) {
    glm::vec3 p[4];
    for (int i = 0; i < 4; ++i) {
      p[i] = glm::vec3(transform * glm::vec4(face[i], 1));
    }
    auto n = glm::normalize(glm::cross(p[1] - p[0], p[2] - p[0]));
    auto t = glm::dot(p[0] - ray.start, n) / glm::dot(glm::normalize(ray.dir), n);
    auto r = ray.start + t * ray.dir;
    if (glm::dot(glm::cross(p[1] - p[0], r - p[0]), n) > 0 && 
        glm::dot(glm::cross(p[2] - p[1], r - p[1]), n) > 0 && 
        glm::dot(glm::cross(p[3] - p[2], r - p[2]), n) > 0 && 
        glm::dot(glm::cross(p[0] - p[3], r - p[3]), n) > 0 &&
        (t < best || best < 0) && t >= 0) {
      best = t;
    }
  }
  return best;
}

SphereCollider::SphereCollider(float radius)
  :radius(radius) {}

AABB SphereCollider::aabb(glm::vec3 position, glm::quat /*rotation*/) const {
  AABB aabb;
  auto offset = glm::vec3(radius);
  aabb.low = position - offset;
  aabb.high = position + offset;
  return aabb;
}

float SphereCollider::ray_hit(glm::vec3 position, glm::quat /*rotation*/, Ray ray) const {
  float a = glm::dot(ray.dir, ray.dir);
  float b = 2 * glm::dot(ray.dir, ray.start-position);
  float c = glm::dot(ray.start, ray.start) + glm::dot(position, position)
    - 2 * glm::dot(ray.start, position) - radius * radius;
  float D = b*b - 4*a*c;
  if (D < 0) {
    return -1;
  }
  // t1 < t2;
  float t1 = (-b - sqrt(D))/a;
  float t2 = (-b + sqrt(D))/a;
  if (t1 >= 0) {
    return t1;
  }
  return t2;
}
