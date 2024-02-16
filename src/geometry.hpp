#pragma once
#include <GL/glew.h>

class Geometry {
public:
  virtual void draw() = 0;
};

class SquareGeometry: public Geometry {
private:
  GLuint vertex_array;
  GLuint vertex_position_buffer;
  GLuint vertex_normal_buffer;
  GLuint index_buffer;
public:
  SquareGeometry();
  ~SquareGeometry();
  void draw() override;
};

class CubeGeometry: public Geometry {
private:
  GLuint vertex_array;
  GLuint vertex_position_buffer;
  GLuint vertex_normal_buffer;
  GLuint index_buffer;
public:
  CubeGeometry();
  ~CubeGeometry();
  void draw() override;
};
