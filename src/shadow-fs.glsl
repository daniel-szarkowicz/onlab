#version 430

in float z;

out float frag_data;

void main() {
    float depth = clamp(z, -1, 1);
    frag_data = exp(80 * depth);
    gl_FragDepth = (depth+1)/2;
}
