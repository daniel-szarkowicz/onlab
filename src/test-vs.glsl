#version 430

in vec3 vtxPos;

void main() {
    gl_Position = vec4(vtxPos, 1);
}
