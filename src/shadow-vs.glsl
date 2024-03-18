#version 430

uniform mat4 model;

layout(location = 0) in vec4 vertexPosition;

void main() {
  gl_Position = model * vertexPosition;
}
