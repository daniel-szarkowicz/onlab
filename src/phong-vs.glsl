#version 430

uniform uint directional_light_count;

uniform struct {
  vec3 direction;
  vec3 ambient_color;
  vec3 emissive_color;
  mat4 matrix;
  sampler2DArrayShadow shadow_map;
} directional_lights[8];

uniform mat4 model;
uniform mat4 view_proj;
uniform mat4 model_inv;

uniform vec3 wEye;

layout(location = 0) in vec4 vertexPosition;
layout(location = 1) in vec3 vertexNormal;

out vec3 wNormal;
out vec3 wView;
out vec4 directional_shadow_map_coords[8];

void main() {
  gl_Position = view_proj * model * vertexPosition;
  vec4 wPos = model * vertexPosition;
  wView = wEye*wPos.w - wPos.xyz;
  for (uint i = 0; i < directional_light_count; ++i) {
    vec4 light_space_pos = directional_lights[i].matrix * wPos;
    vec3 map_coords = (light_space_pos.xyz / light_space_pos.w) * 0.5 + 0.5;
    directional_shadow_map_coords[i] = vec4(map_coords.xy, 0, map_coords.z);
  }
  wNormal = (vec4(vertexNormal, 0) * model_inv).xyz;
}
