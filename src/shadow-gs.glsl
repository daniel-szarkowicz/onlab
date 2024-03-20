#version 430

#define MAX_LAYERS 4
uniform uint layer_count;
uniform mat4 view_projs[MAX_LAYERS];

layout(triangles) in;
layout(triangle_strip, max_vertices = 12) out;

out float z;

void main() {
    for (uint i = 0; i < layer_count; ++i) {
        gl_Layer = int(i);
        for (uint j = 0; j < 3; ++j) {
            vec4 pos = view_projs[i] * gl_in[j].gl_Position;
            z = pos.z / pos.w;
            pos.z = 0;
            gl_Position = pos;
            EmitVertex();
        }
        EndPrimitive();
    }
}
