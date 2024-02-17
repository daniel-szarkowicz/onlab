#include "object.hpp"
#include "geometry.hpp"

Object::Object(Geometry geometry, Collider* collider)
  : geometry(geometry), collider(collider),
    position(glm::vec3(0, 0, 0)), rotation(glm::quat(1, 0, 0, 0)), immovable(false) {}

AABB Object::aabb() {
  return collider->aabb(position, rotation);
}

Object Object::box(glm::vec3 size) {
  return Object(Geometry::box(size), new BoxCollider(size));
}

Object Object::sphere(float radius) {
  return Object(Geometry::sphere(radius), new SphereCollider(radius));
}

Object::~Object() {
  // FIXME must delete collider to have no memory leaks
  
  // delete collider;
}
