#include "geometry.hpp"
#include <iostream>

SquareGeometry::SquareGeometry() {
  glCreateVertexArrays(1, &vertex_array);
  glBindVertexArray(vertex_array);

  glCreateBuffers(1, &vertex_position_buffer);
  glBindBuffer(GL_ARRAY_BUFFER, vertex_position_buffer);
  float positions[] = {
    0.5, 0.5, 0,
    -.5, 0.5, 0,
    0.5, -.5, 0,
    -.5, -.5, 0,
  };
  glBufferData(GL_ARRAY_BUFFER, sizeof(positions), positions, GL_STATIC_DRAW);
  glEnableVertexAttribArray(0);
  glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 0, 0);

  glCreateBuffers(1, &vertex_normal_buffer);
  glBindBuffer(GL_ARRAY_BUFFER, vertex_normal_buffer);
  float normals[] = {
    0, 0, 1,
    0, 0, 1,
    0, 0, 1,
    0, 0, 1,
  };
  glBufferData(GL_ARRAY_BUFFER, sizeof(normals), normals, GL_STATIC_DRAW);
  glEnableVertexAttribArray(1);
  glVertexAttribPointer(1, 3, GL_FLOAT, GL_FALSE, 0, 0);

  glCreateBuffers(1, &index_buffer);
  glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, index_buffer);
  GLushort indicies[] = {
    0, 1, 2,
    2, 1, 3,
  };
  glBufferData(GL_ELEMENT_ARRAY_BUFFER, sizeof(indicies), indicies, GL_STATIC_DRAW);
}

void SquareGeometry::draw() {
  glBindVertexArray(vertex_array);
  glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, index_buffer);
  glDrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_SHORT, NULL);
}

SquareGeometry::~SquareGeometry() {
  glDeleteBuffers(1, &vertex_array);
  glDeleteBuffers(1, &vertex_position_buffer);
  glDeleteBuffers(1, &vertex_normal_buffer);
  glDeleteBuffers(1, &index_buffer);
}

CubeGeometry::CubeGeometry() {
  glCreateVertexArrays(1, &vertex_array);
  glBindVertexArray(vertex_array);

  glCreateBuffers(1, &vertex_position_buffer);
  glBindBuffer(GL_ARRAY_BUFFER, vertex_position_buffer);
  float positions[] = {
    0.5, 0.5, 0.5,
    -.5, 0.5, 0.5,
    0.5, -.5, 0.5,
    -.5, -.5, 0.5,

    -.5, 0.5, -.5,
    0.5, 0.5, -.5,
    -.5, -.5, -.5,
    0.5, -.5, -.5,

    0.5, 0.5, 0.5,
    0.5, -.5, 0.5,
    0.5, 0.5, -.5,
    0.5, -.5, -.5,

    -.5, -.5, 0.5,
    -.5, 0.5, 0.5,
    -.5, -.5, -.5,
    -.5, 0.5, -.5,

    0.5, 0.5, 0.5,
    0.5, 0.5, -.5,
    -.5, 0.5, 0.5,
    -.5, 0.5, -.5,

    0.5, -.5, -.5,
    0.5, -.5, 0.5,
    -.5, -.5, -.5,
    -.5, -.5, 0.5,
  };
  glBufferData(GL_ARRAY_BUFFER, sizeof(positions), positions, GL_STATIC_DRAW);
  glEnableVertexAttribArray(0);
  glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 0, 0);

  glCreateBuffers(1, &vertex_normal_buffer);
  glBindBuffer(GL_ARRAY_BUFFER, vertex_normal_buffer);
  float normals[] = {
    0, 0, 1,
    0, 0, 1,
    0, 0, 1,
    0, 0, 1,

    0, 0, -1,
    0, 0, -1,
    0, 0, -1,
    0, 0, -1,

     1, 0, 0,
     1, 0, 0,
     1, 0, 0,
     1, 0, 0,

    -1, 0, 0,
    -1, 0, 0,
    -1, 0, 0,
    -1, 0, 0,

    0,  1, 0,
    0,  1, 0,
    0,  1, 0,
    0,  1, 0,

    0, -1, 0,
    0, -1, 0,
    0, -1, 0,
    0, -1, 0,
  };
  glBufferData(GL_ARRAY_BUFFER, sizeof(normals), normals, GL_STATIC_DRAW);
  glEnableVertexAttribArray(1);
  glVertexAttribPointer(1, 3, GL_FLOAT, GL_FALSE, 0, 0);

  glCreateBuffers(1, &index_buffer);
  glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, index_buffer);
  GLushort indicies[] = {
    0, 1, 2,
    2, 1, 3,

    4, 5, 6,
    6, 5, 7,

    8, 9, 10,
    10, 9, 11,

    12, 13, 14,
    14, 13, 15,

    16, 17, 18,
    18, 17, 19,

    20, 21, 22,
    22, 21, 23,
  };
  glBufferData(GL_ELEMENT_ARRAY_BUFFER, sizeof(indicies), indicies, GL_STATIC_DRAW);
}

void CubeGeometry::draw() {
  glBindVertexArray(vertex_array);
  glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, index_buffer);
  glDrawElements(GL_TRIANGLES, 36, GL_UNSIGNED_SHORT, NULL);
}

CubeGeometry::~CubeGeometry() {
  glDeleteBuffers(1, &vertex_array);
  glDeleteBuffers(1, &vertex_position_buffer);
  glDeleteBuffers(1, &vertex_normal_buffer);
  glDeleteBuffers(1, &index_buffer);
}
