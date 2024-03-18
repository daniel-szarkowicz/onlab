#version 430

uniform mat4 view_proj;

layout(triangles) in;
layout(triangle_strip, max_vertices = 3) out;

void main() {
    gl_Layer = 0;
    gl_Position = view_proj * gl_in[0].gl_Position;
    EmitVertex();
    gl_Position = view_proj * gl_in[1].gl_Position;
    EmitVertex();
    gl_Position = view_proj * gl_in[2].gl_Position;
    EmitVertex();
    EndPrimitive();
}
