#version 430

out float frag_data;

void main() {
    float depth = gl_FragCoord.z*2-1;
    frag_data = exp(80 * depth);
}
