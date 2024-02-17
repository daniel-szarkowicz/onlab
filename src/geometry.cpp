#include "geometry.hpp"
#include <glm/ext/matrix_transform.hpp>

Geometry::Geometry(Mesh mesh,
                   glm::mat4 transform)
  :mesh(mesh), transform(transform) {}

Geometry Geometry::box(glm::vec3 size) {
  return Geometry(Mesh::box(),
                  glm::scale(glm::mat4(1), size));
}

Geometry Geometry::sphere(float r) {
  glm::vec3 scale = glm::vec3(r, r, r);
  return Geometry(Mesh::sphere(),
                  glm::scale(glm::mat4(1), scale));
}
