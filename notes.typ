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

