#include "mesh.hpp"

Mesh Mesh::_box;
Mesh Mesh::_sphere;
Mesh Mesh::_bounding_box;

Mesh Mesh::box() {
  if (_box.vertex_array != 0) {
    return _box;
  }

  glCreateVertexArrays(1, &_box.vertex_array);
  glBindVertexArray(_box.vertex_array);

  GLuint vpb, vnb, ib;
  glCreateBuffers(1, &vpb);
  glBindBuffer(GL_ARRAY_BUFFER, vpb);
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

  glCreateBuffers(1, &vnb);
  glBindBuffer(GL_ARRAY_BUFFER, vnb);
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

  glCreateBuffers(1, &ib);
  glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, ib);
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
  _box.vertex_count = sizeof(indicies) / sizeof(GLushort);
  return _box;
}

Mesh Mesh::sphere() {
  // TODO
  return box();
}

Mesh Mesh::bounding_box() {
  if (_bounding_box.vertex_array != 0) {
    return _bounding_box;
  }

  glCreateVertexArrays(1, &_bounding_box.vertex_array);
  glBindVertexArray(_bounding_box.vertex_array);

  GLuint vpb, ib;
  glCreateBuffers(1, &vpb);
  glBindBuffer(GL_ARRAY_BUFFER, vpb);
  float positions[] = {
    0.5, 0.5, 0.5,
    -.5, 0.5, 0.5,
    0.5, -.5, 0.5,
    -.5, -.5, 0.5,

    -.5, 0.5, -.5,
    0.5, 0.5, -.5,
    -.5, -.5, -.5,
    0.5, -.5, -.5,
  };
  glBufferData(GL_ARRAY_BUFFER, sizeof(positions), positions, GL_STATIC_DRAW);
  glEnableVertexAttribArray(0);
  glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 0, 0);

  glCreateBuffers(1, &ib);
  glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, ib);
  GLushort indicies[] = {
     0,  1,  1,  3,  3,  2,  2,  0,
     4,  5,  5,  7,  7,  6,  6,  4,
     0,  5,  1,  4,  3,  6,  2,  7,
  };
  glBufferData(GL_ELEMENT_ARRAY_BUFFER, sizeof(indicies), indicies, GL_STATIC_DRAW);
  _bounding_box.vertex_count = sizeof(indicies) / sizeof(GLushort);
  return _bounding_box;
}
