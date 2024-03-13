#version 430

uniform vec3 kd = vec3(0.7, 0.7, 0.7);
uniform vec3 ks = vec3(0.2, 0.2, 0.3);
uniform vec3 ka = vec3(0.4, 0.4, 0.4);
uniform float shine = 100;
uniform vec3 La = vec3(1, 1, 1);
uniform vec3 Le = vec3(1, 1, 1);

in vec3 wNormal;
in vec3 wView;
in vec3 wLight;

out vec4 frag_color;


void main() {
  vec3 N = normalize(wNormal);
  vec3 V = normalize(wView);
  vec3 L = normalize(wLight);
  vec3 H = normalize(L + V);
  float cost = max(dot(N, L), 0);
  float cosd = max(dot(N, H), 0);
  float cosa = max(dot(N, V), 0);
  vec3 color = ka * (0.9 + cosa * 0.1) * La + (kd * cost + ks * pow(cosd, shine)) * Le;
  frag_color = vec4(color, 1);
}
