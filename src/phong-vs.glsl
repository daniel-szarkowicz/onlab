#version 430

uniform mat4 model;
uniform mat4 view_proj;
uniform mat4 model_inv;

uniform vec4 wLiPos = vec4(10000, 10000, 10000, 0);
uniform vec3 wEye;

layout(location = 0) in vec4 vertexPosition;
layout(location = 1) in vec3 vertexNormal;

out vec3 wNormal;
out vec3 wView;
out vec3 wLight;

void main() {
  gl_Position = view_proj * model * vertexPosition;
  vec4 wPos = model * vertexPosition;
  wLight = wLiPos.xyz*wPos.w - wPos.xyz*wLiPos.w;
  wView = wEye*wPos.w - wPos.xyz;
  wNormal = (vec4(vertexNormal, 0) * model_inv).xyz;
}
