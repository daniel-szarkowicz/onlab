#pragma once
#include <GL/glew.h>

class Mesh {
private:
  static Mesh _box;
  static Mesh _sphere;
  static Mesh _bounding_box;
  static Mesh _line;
public:
  GLuint vertex_array;
  GLsizei vertex_count;

  static Mesh box();
  static Mesh sphere();
  static Mesh bounding_box();
  static Mesh line();
};

