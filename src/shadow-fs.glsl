#version 430

in float z;

out float frag_depth;

void main() {
    float depth = clamp(z, -1, 1);
    frag_depth = (depth+1)/2;
    gl_FragDepth = (depth+1)/2;
}
