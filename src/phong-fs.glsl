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

float calculate_shadow(vec4 light_space_pos, vec3 normal, vec3 light_dir) {
  vec3 proj_coords = light_space_pos.xyz / light_space_pos.w;
  proj_coords = proj_coords * 0.5 + 0.5;
  float current_depth = proj_coords.z;
  if (current_depth > 1.0) {
    // no shadow if depth is outside of shadow map
    return 0.0;
  }
  float shadow_depth = texture(shadow_map, proj_coords.xy).r;
  // This fixes 'shadow acne', but causes 'peter panning' which is fixed by
  // using front face culling, instead of back face. But front face culling
  // also seems to fix 'shadow acne', so this might be useless.
  // TODO: more experimentation
  // float bias = max(0.005 * (1.0 - dot(normal, light_dir)), 0.001);
  float bias = 0.0;
  float shadow = current_depth - bias > shadow_depth ? 1.0 : 0.0;
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
  float shadow = calculate_shadow(frag_pos_light_space, N, L);
  vec3 color = ka * (0.9 + cosa * 0.1) * La
    + (1.0 - shadow) * (kd * cost + ks * pow(cosd, shine)) * Le;
  frag_color = vec4(color, 1);
}
