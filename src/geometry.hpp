#pragma once
#include <glm/glm.hpp>
#include <GL/glew.h>
#include "mesh.hpp"

class Geometry {
public:
  Mesh mesh;
  glm::mat4 transform;

  Geometry(Mesh mesh, glm::mat4 transform);

  static Geometry box(glm::vec3 size);
  static Geometry sphere(float r);
};

