#version 430

uniform vec3 kd = vec3(1, 1, 1);
uniform vec3 ks = vec3(0.1, 0.1, 0.2);
uniform vec3 ka = vec3(1, 1, 1);
uniform float shine = 100;

uniform uint directional_light_count;

uniform struct {
  vec3 direction;
  vec3 ambient_color;
  vec3 emissive_color;
  mat4 matrix;
  sampler2D shadow_map;
} directional_lights[8];

in vec3 wNormal;
in vec3 wView;
in vec4 directional_light_space_pos[8];

out vec4 frag_color;

float calculate_shadow(uint i, vec3 normal, vec3 light_dir) {
  vec3 proj_coords = directional_light_space_pos[i].xyz
    / directional_light_space_pos[i].w;
  proj_coords = proj_coords * 0.5 + 0.5;
  float current_depth = proj_coords.z;
  // if (proj_coords.x < 0.0 || proj_coords.x > 1.0
  //   || proj_coords.y < 0.0 || proj_coords.y > 1.0
  //   || proj_coords.z < 0.0 || proj_coords.z > 1.0) {
  //   return 0.5;
  // }
  if (current_depth > 1.0) {
    // no shadow if depth is outside of shadow map
    return 0.0;
  }
  float shadow = 0.0;
  vec2 texel_size = 1 / vec2(textureSize(directional_lights[i].shadow_map, 0));
  int radius = 2;
  for (int x = -radius; x <= radius; x += 1) {
    for (int y = -radius; y <= radius; y += 1) {
      float shadow_depth = texture(
        directional_lights[i].shadow_map, proj_coords.xy + vec2(x, y) * texel_size
      ).r;
      // This fixes 'shadow acne', but causes 'peter panning' which is fixed by
      // using front face culling, instead of back face. But front face culling
      // also seems to fix 'shadow acne', so this might be useless.
      // TODO: more experimentation
      // float bias = max(0.005 * (1.0 - dot(normal, light_dir)), 0.000);
      float bias = 0.0;
      shadow += current_depth - bias > shadow_depth ? 1.0 : 0.0;
    }
  }
  return shadow / ((2*radius + 1) * (2*radius + 1));
}

void main() {
  vec3 N = normalize(wNormal);
  vec3 V = normalize(wView);
  vec3 color = vec3(0, 0, 0);
  for (uint i = 0; i < directional_light_count; ++i) {
    vec3 L = normalize(-directional_lights[i].direction);
    vec3 H = normalize(L + V);
    float cost = max(dot(N, L), 0);
    float cosd = max(dot(N, H), 0);
    float cosa = max(dot(N, V), 0);
    float shadow = calculate_shadow(i, N, L);
    color += ka * (0.9 + cosa * 0.1) * directional_lights[i].ambient_color;
    color += (1.0 - shadow) * (kd * cost + ks * pow(cosd, shine))
      * directional_lights[i].emissive_color;
  }
  frag_color = vec4(color, 1);
}
