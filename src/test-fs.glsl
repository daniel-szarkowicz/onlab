#version 430

out vec4 frag_color;

in vec3 wNormal;

void main() {
    frag_color = vec4(wNormal, 1);
}
