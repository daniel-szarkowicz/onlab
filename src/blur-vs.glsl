#version 430

layout(location = 0) in vec4 vertexPosition;

void main() {
    gl_Position = vertexPosition;
}
