#version 430

#define BLUR_RADIUS 5

uniform sampler2DArray shadow_map;
uniform bool horizontal;
uniform float weight[BLUR_RADIUS] = float[] (0.227027, 0.1945946, 0.1216216, 0.054054, 0.016216);

in vec3 tex_coord;

out float frag_data;


void main() {

    vec2 texel_size = 1.0/vec2(textureSize(shadow_map, 0));
    float result = texture(shadow_map, tex_coord).r * weight[0];
    if (horizontal) {
        for (uint i = 1; i < BLUR_RADIUS; ++i) {
            result += texture(shadow_map, tex_coord + vec3(texel_size.x*i, 0, 0)).r * weight[i];
            result += texture(shadow_map, tex_coord - vec3(texel_size.x*i, 0, 0)).r * weight[i];
        }
    } else {
        for (uint i = 1; i < BLUR_RADIUS; ++i) {
            result += texture(shadow_map, tex_coord + vec3(0, texel_size.y*i, 0)).r * weight[i];
            result += texture(shadow_map, tex_coord - vec3(0, texel_size.y*i, 0)).r * weight[i];
        }
    }
    frag_data = result;
}
