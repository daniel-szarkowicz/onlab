#include "object.hpp"
#include "collider.hpp"
#include "geometry.hpp"

Object::Object(Geometry geometry, Collider* collider)
  : geometry(geometry), collider(collider),
    position(0, 0, 0), rotation(1, 0, 0, 0),
    momentum(0, 0, 0),
    mass(1), immovable(false), force(0, 0, 0) {}

AABB Object::aabb() const {
  return collider->aabb(position, rotation);
}

float Object::ray_hit(Ray ray) const {
  return collider->ray_hit(position, rotation, ray);
}

Object Object::box(glm::vec3 size) {
  return Object(Geometry::box(size), new BoxCollider(size));
}

Object Object::sphere(float radius) {
  return Object(Geometry::sphere(radius), new SphereCollider(radius));
}

void Object::apply_force(glm::vec3 /*position*/, glm::vec3 force) {
  if (!immovable) {
    this->force += force;
  }
}

void Object::update(float dt) {
  if (!immovable) {
    momentum += force * dt;
    position += momentum * dt / mass;

    // angular_momentum += torque * dt;
    // rotation += angular_momentum * dt / inertia_tensor;
  }
}

void Object::reset_force() {
  force = glm::vec3(0);
}

Object::~Object() {
  // FIXME must delete collider to have no memory leaks
  
  // delete collider;
}
