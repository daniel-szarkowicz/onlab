#pragma once

#include "camera.hpp"
#include "object.hpp"
#include <GL/glew.h>
#include <vector>

// not to be confused with opengl geometry shaders
class GeometryShader {
  GLuint model;
  GLuint view;
  GLuint projection;
  GLuint model_inv;
  // GLuint model_buffer;
public:
  GLuint program;
  GeometryShader();
  void drawObjects(const Camera& camera, const std::vector<Object>& objects);
};

class AABBShader {
  GLuint model;
  GLuint view;
  GLuint projection;
  Mesh mesh;
public:
  GLuint program;
  AABBShader();
  void drawObjects(const Camera& camera, const std::vector<Object>& objects);
};
