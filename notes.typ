#align(center)[#text(size: 25pt)[3D Rigid Body szimuláció]]

= Források
- https://www.toptal.com/game/video-game-physics-part-i-an-introduction-to-rigid-body-dynamics
- https://www.cs.cmu.edu/~baraff/sigcourse/index.html

= Tanult cuccok
- Rigid body paraméterei:
  - mass ($M$, constant)
  - position ($x$)
  - velocity or linear momentum, momentum is preferred ($v$ or $P$)
  - rotation (3x3 matrix or quaternion, quaternion is preferred) ($bold(R)$ or $q$)
  - inertia ($bold(I)$ = $bold(R) dot bold(I)_"body" dot bold(R)^T$, $bold(I)_"body"$ is constant)
  - angular momentum $L$
- Rigid body "bemenetei"
  - force ($F$)
  - torque ($tau$)

= Ray-Sphere intersection
#let vec(x) = $overline(bold(#x))$
Ray parameters: position $vec(s)$, direction $vec(d)$\
Sphere parameters: center $vec(c)$, radius $r$\
Ray: $vec(p)(t) = vec(s) + t dot vec(d)$\
Sphere: $|vec(p)-vec(c)| = r$\
Intersection: $
|vec(p)(t)-vec(c)| &= r\
|vec(s) + t dot vec(d) - vec(c)| &= r\
(vec(s) + t dot vec(d) - vec(c))^2 &= r^2\
vec(s)^2 + t^2 vec(d)^2 + vec(c)^2 +
2 t vec(s) vec(d) - 2 vec(s) vec(c) - 2 t vec(d) vec(c) - r^2 &= 0\
a = vec(d) vec(d), quad b = 2 vec(s) vec(d) - 2 vec(d) vec(d),& quad
c = vec(s) vec(s) + vec(c) vec(c) - 2 vec(s) vec(c) - r^2\
t_(1,2) = - vec(s) vec(d)
$

= Ray-Box intersection

https://cg.iit.bme.hu/portal/oktatott-targyak/szamitogepes-grafika-es-kepfeldolgozas/sugarkoevetes
