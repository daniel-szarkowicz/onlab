#version 430

uniform sampler2DArray shadow_map;
uniform bool horizontal;

in vec3 tex_coord;

out float frag_data;

void main() {
    frag_data = texture(shadow_map, tex_coord).r;
}
