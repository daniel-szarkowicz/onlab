#version 430

uniform mat4 model;
uniform mat4 view_proj;

layout(location = 0) in vec4 vertexPosition;

void main() {
  gl_Position = view_proj * model * vertexPosition;
}
