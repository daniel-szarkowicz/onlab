#version 430

uniform uint directional_light_count;

uniform struct {
  vec3 direction;
  vec3 ambient_color;
  vec3 emissive_color;
  mat4 matrix;
  sampler2D shadow_map;
} directional_lights[8];

uniform mat4 model;
uniform mat4 view_proj;
uniform mat4 model_inv;
uniform mat4 light_space_matrix;

uniform vec4 wLiPos = vec4(10000, 10000, -10000, 0);
uniform vec3 wEye;

layout(location = 0) in vec4 vertexPosition;
layout(location = 1) in vec3 vertexNormal;

out vec3 wNormal;
out vec3 wView;
out vec4 directional_light_space_pos[8];

void main() {
  gl_Position = view_proj * model * vertexPosition;
  vec4 wPos = model * vertexPosition;
  wView = wEye*wPos.w - wPos.xyz;
  for (uint i = 0; i < directional_light_count; ++i) {
    directional_light_space_pos[i] = directional_lights[i].matrix * wPos;
  }
  wNormal = (vec4(vertexNormal, 0) * model_inv).xyz;
}
