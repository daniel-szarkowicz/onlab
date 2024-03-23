#version 430

uniform vec3 kd = vec3(1, 1, 1);
uniform vec3 ks = vec3(0.1, 0.1, 0.2);
uniform vec3 ka = vec3(1, 1, 1);
uniform float shine = 100;

#define MAX_LIGHTS 4
#define MAX_LAYERS 4
uniform uint directional_light_count;
uniform uint shadow_layer_count;

uniform struct {
  vec3 direction;
  vec3 ambient_color;
  vec3 emissive_color;
  mat4 matrices[MAX_LAYERS];
  sampler2DArray shadow_map;
} directional_lights[MAX_LIGHTS];

in vec3 wNormal;
in vec3 wView;
in vec4 directional_shadow_map_coords[MAX_LIGHTS][MAX_LAYERS];

out vec4 frag_color;

float calculate_shadow(uint i, vec3 normal, vec3 light_dir) {
  vec4 map_coords;
  uint j = 0;
  do {
    map_coords = directional_shadow_map_coords[i][j];
    ++j;
  } while ((map_coords.x < 0.0 || map_coords.x > 1.0
    || map_coords.y < 0.0 || map_coords.y > 1.0
    || map_coords.w < 0.0 || map_coords.w > 1.0)
    && j < shadow_layer_count);
  if (map_coords.x < 0.0 || map_coords.x > 1.0
    || map_coords.y < 0.0 || map_coords.y > 1.0
    || map_coords.w < 0.0 || map_coords.w > 1.0) {
    // no shadow outside shadow map
    return 1.0;
  }
  float exp_cz = texture(directional_lights[i].shadow_map, map_coords.xyz).r;
  float exp_minuscd = exp(-80 * map_coords.w);
  float shadow = exp_cz * exp_minuscd;
  if (shadow > 1.05) {
    shadow = 0;
    vec2 texel_size = 1.0/vec2(textureSize(directional_lights[i].shadow_map, 0));
    vec2 offset = mod(map_coords.xy, texel_size);
    float x = map_coords.x;
    float y = map_coords.y;
    float layer = map_coords.z;
    float x1 = map_coords.x - offset.x;
    float x2 = x1 + texel_size.x;
    float y1 = map_coords.y - offset.y;
    float y2 = y1 + texel_size.y;
    shadow = 1/(texel_size.x * texel_size.y) * (
      min(texture(directional_lights[i].shadow_map, vec3(x1, y1, layer)).r * exp_minuscd, 1) * (x2 - x) * (y2 - y) +
      min(texture(directional_lights[i].shadow_map, vec3(x2, y1, layer)).r * exp_minuscd, 1) * (x - x1) * (y2 - y) +
      min(texture(directional_lights[i].shadow_map, vec3(x1, y2, layer)).r * exp_minuscd, 1) * (x2 - x) * (y - y1) +
      min(texture(directional_lights[i].shadow_map, vec3(x2, y2, layer)).r * exp_minuscd, 1) * (x - x1) * (y - y1)
    );
    // float samples = 8;
    // float pi = 3.141592653589793;
    // for (float n = samples; n > 0; n -= 1) {
    //   for (float m = 1; m <= samples; m += 1) {
    //     float u = n/samples;
    //     float v = m/samples;
    //     float x = sqrt(u) * cos(2*pi * v) * texel_size.x;
    //     float y = sqrt(u) * sin(2*pi * v) * texel_size.y;
    //     exp_cz = texture(
    //       directional_lights[i].shadow_map, map_coords.xyz + vec3(x, y, 0)
    //     ).r;
    //     shadow += min(exp_cz * exp_minuscd, 1);
    //   }
    // }
    // for (float x = -1; x <= 1; x += 1) {
    //   for (float y = -1; y <= 1; y += 1) {
    //     exp_cz = texture(
    //       directional_lights[i].shadow_map, map_coords.xyz + vec3(x, y, 0) * vec3(texel_size, 0)
    //     ).r;
    //     shadow += min(exp_cz * exp_minuscd, 1);
    //   }
    // }
    // shadow = shadow / 9;
  }
  return shadow;
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
    if (shadow > 1.05) {
      frag_color = vec4(1, 0, 0, 1);
      return;
    }
    color += ka * (0.9 + cosa * 0.1) * directional_lights[i].ambient_color;
    color += clamp(shadow, 0, 1) * (kd * cost + ks * pow(cosd, shine))
      * directional_lights[i].emissive_color;
  }
  frag_color = vec4(color, 1);
}
