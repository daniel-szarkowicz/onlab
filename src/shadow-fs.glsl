#version 430

out float frag_data;

void main() {
    float depth = gl_FragCoord.z;
    frag_data = exp(80 * depth);
}
