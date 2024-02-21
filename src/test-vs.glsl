#version 430

layout(location = 0) in vec3 vtxPos;
layout(location = 1) in vec3 vtxNorm;

void main() {
    gl_Position = vec4(vtxPos, 1);
}
