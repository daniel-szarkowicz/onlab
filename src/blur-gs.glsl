#version 430

#define MAX_LAYERS 4
uniform uint layer_count;

layout(triangles) in;
layout(triangle_strip, max_vertices = 12) out;

out vec3 tex_coord;

void main() {
    for (uint i = 0; i < layer_count; ++i) {
        gl_Layer = int(i);
        for (uint j = 0; j < 3; ++j) {
            vec4 pos = gl_in[j].gl_Position;
            gl_Position = vec4(2 * pos.xyz, 1);
            tex_coord = pos.xyz + vec3(0.5, 0.5, i);
            EmitVertex();
        }
        EndPrimitive();
    }
}
