#include "object.hpp"
#include "geometry.hpp"
#include <glm/ext/matrix_transform.hpp>
#include <glm/gtc/type_ptr.hpp>

Object::Object(
               std::shared_ptr<Geometry> geometry,
               glm::vec3 position,
               glm::vec3 scale,
               float mass)
  : geometry(geometry), position(position), scale(scale), mass(mass) {}

void Object::draw(GLuint model_uniform_location, GLuint model_inv_uniform_location) {
  glm::mat4 model(1.0f);
  model = glm::scale(model, scale);
  model = glm::translate(model, position);
  glUniformMatrix4fv(model_uniform_location, 1, GL_FALSE, glm::value_ptr(model));

  glm::mat4 model_inv(1.0f);
  model_inv = glm::translate(model_inv, -position);
  model_inv = glm::scale(model, 1.0f/scale);
  glUniformMatrix4fv(model_inv_uniform_location, 1, GL_FALSE, glm::value_ptr(model));

  geometry->draw();
}

void Object::update(float dt) {
  momentum += force * dt;
  position += momentum * dt / mass;
}
