#version 430

uniform mat4 model;
uniform mat4 model_inv;
uniform mat4 view_proj;

layout(location = 0) in vec3 vtxPos;
layout(location = 1) in vec3 vtxNorm;

out vec3 wNormal;

void main() {
    gl_Position = view_proj * model * vec4(vtxPos, 1);
    // vec4 wPos = model * vec4(vtxPos);
    wNormal = vec3(vec4(vtxNorm, 0) * model_inv);
}
