#include "mesh.hpp"
#include <cmath>

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

const short LATITUDE = 36;
const short LONGITUDE = 18;

Mesh Mesh::sphere() {
  if (_sphere.vertex_array != 0) {
    return _sphere;
  }
  struct Vertex {float x, y, z/*, nx, ny, nz*/;};
  struct Index {short a, b, c;};
  Vertex vertices[LATITUDE * LONGITUDE];
  Index indices[LATITUDE * (LONGITUDE - 1) * 2];
  for (int b = 0; b < LONGITUDE; ++b) {
    for (int a = 0; a < LATITUDE; ++a) {
      float alpha = a*M_PI * 2 / LATITUDE;
      float beta =  (b*M_PI / (LONGITUDE - 1)) - M_PI/2;
      float y = sin(beta);
      float x = cos(beta) * sin(alpha);
      float z = cos(beta) * cos(alpha);
      vertices[a + b*LATITUDE] = {x, y, z/*, x, y, z*/};
    }
  }
  for (short b = 0; b < LONGITUDE-1; ++b) {
    for (short a = 0; a < LATITUDE; ++a) {
      indices[(a + b*LATITUDE) * 2] = {
        (short)(a + b * LATITUDE),
        (short)((a + 1) % LATITUDE + b * LATITUDE),
        (short)(a + b * LATITUDE + LATITUDE),
      };
      indices[(a + b*LATITUDE) * 2 + 1] = {
        (short)((a + 1) % LATITUDE + b * LATITUDE),
        (short)((a + 1) % LATITUDE + b * LATITUDE + LATITUDE),
        (short)(a + b * LATITUDE + LATITUDE),
      };
    }
  }
  glCreateVertexArrays(1, &_sphere.vertex_array);
  glBindVertexArray(_sphere.vertex_array);

  GLuint vbo, ib;
  glCreateBuffers(1, &vbo);
  glBindBuffer(GL_ARRAY_BUFFER, vbo);
  glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);
  glEnableVertexAttribArray(0);
  glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 0, 0);
  glEnableVertexAttribArray(1);
  glVertexAttribPointer(1, 3, GL_FLOAT, GL_FALSE, 0, 0);

  glCreateBuffers(1, &ib);
  glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, ib);
  glBufferData(GL_ELEMENT_ARRAY_BUFFER, sizeof(indices), indices, GL_STATIC_DRAW);

  _sphere.vertex_count = sizeof(indices) / sizeof(Index)
    * sizeof(Index) / sizeof(GLushort);
  return _sphere;
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
