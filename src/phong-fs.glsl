#version 430

uniform vec3 kd = vec3(0.7, 0.7, 0.7);
uniform vec3 ks = vec3(0.2, 0.2, 0.3);
uniform vec3 ka = vec3(0.4, 0.4, 0.4);
uniform float shine = 100;
uniform vec3 La = vec3(1, 1, 1);
uniform vec3 Le = vec3(1, 1, 1);
uniform sampler2D shadow_map;

in vec3 wNormal;
in vec3 wView;
in vec3 wLight;
in vec4 frag_pos_light_space;

out vec4 frag_color;

float calculate_shadow(vec4 light_space_pos) {
  vec3 proj_coords = light_space_pos.xyz / light_space_pos.w;
  proj_coords = proj_coords * 0.5 + 0.5;
  float shadow_depth = texture(shadow_map, proj_coords.xy).r;
  float current_depth = proj_coords.z;
  float shadow = current_depth > shadow_depth ? 1.0 : 0.0;
  return shadow;
}

void main() {
  vec3 N = normalize(wNormal);
  vec3 V = normalize(wView);
  vec3 L = normalize(wLight);
  vec3 H = normalize(L + V);
  float cost = max(dot(N, L), 0);
  float cosd = max(dot(N, H), 0);
  float cosa = max(dot(N, V), 0);
  float shadow = calculate_shadow(frag_pos_light_space);
  vec3 color = ka * (0.9 + cosa * 0.1) * La
    + (1.0 - shadow) * (kd * cost + ks * pow(cosd, shine)) * Le;
  frag_color = vec4(color, 1);
}
