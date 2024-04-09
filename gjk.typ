= Simplex closest point to origin

A simplex of $n$ dimensions is defined by points ${A_1, A_2, ..., A_n, A_(n+1)}$

Point $P$ on the space of the simplex can be defined with barycentric coordinates
as
$
  P = sum_(i=1)^(n+1) t_i dot A_i, quad sum_(i=1)^(n+1) t_i = 1\
  // t_(n+1) = 1 - sum_(i=1)^n t_i\
  // P = A_(n+1) + sum_(i=1)^n t_i dot (A_i - A_(n+1))
$
This point is inside the simplex if and only if
$ limits(forall)_(i=1)^(n+1) t_i >= 0 $
The squared distance of point $P$ from the origin is
$
  // d^2 = P^2 = A_(n+1)^2 + sum_(i=1)^n A_(n+1)
  d^2 = P^2 = sum_(i=1)^(n+1) sum_(j=1)^(n+1) t_i dot t_j dot A_i dot A_j
$
We know this function has exactly one minimum, it can be found by its derivatives
$
  (diff d^2)/(diff t_i) = sum_(j=1)^(n+1) 2 t_j dot A_i dot A_j =& 0\
  sum_(j=1)^(n+1) t_j dot A_i dot A_j =& 0
$
We have a linear equation with $n+1$ variables and $n+2$ equations, we can
discard one of the equations. The linear equation can be solved by writing
it into a matrix and inverting it.
$
  mat(
    A_1 dot A_1, A_1 dot A_2, dots.c   , A_1 dot A_(n+1);
    A_2 dot A_1, A_2 dot A_2, dots.c   , A_2 dot A_(n+1);
    dots.v     , dots.v     , dots.down, dots.v         ;
    A_n dot A_1, A_n dot A_2, dots.c   , A_n dot A_(n+1);
    1          ,           1, dots.c  , 1
  ) dot
  mat(t_1; t_2; dots.v; t_n; t_(n+1)) =
  mat(0  ; 0  ; dots.v; 0  ;1)
$
If any point has a negative multiplier, than the nearest point is not in the
simplex. We simply remove the points with negative multipliers from the simplex
and calculate again.

#pagebreak()
= Simplex closest point to origin (attempt 2)

A simplex of $n$ dimensions is defined by points ${A_1, A_2, ..., A_n, A_(n+1)}$

Point $P$ on the space of the simplex can be defined with barycentric coordinates
as
$
  P = sum_(i=1)^(n+1) t_i dot A_i, quad sum_(i=1)^(n+1) t_i = 1\
  t_(n+1) = 1 - sum_(i=1)^n t_i\
  P = sum_(i=1)^n t_i dot A_i + (1 - sum_(i=1)^n t_i) dot A_(n+1) =
    A_(n+1) + sum_(i=1)^n t_i dot (A_i - A_(n+1))
$

The squared distance of point $P$ from the origin is
$
  d^2 = P^2 = A_(n+1)^2 + 2 sum_(i=1)^n t_i dot A_(n+1) dot (A_i - A_(n+1))
    + sum_(i=1)^n sum_(j=1)^n t_i dot t_j dot (A_i - A_(n+1)) dot (A_j - A_(n+1))
$

We know this function has exactly one minimum, it can be found by its derivatives
$
  (diff d^2)/(diff t_i) = 2 A_(n+1) dot (A_i - A_(n+1))
    + 2 sum_(j=1)^n t_j dot (A_i - A_(n+1)) dot (A_j - A_(n+1)) =& 0\
  sum_(j=1)^n t_j dot (A_i - A_(n+1)) dot (A_j - A_(n+1)) =&
    -A_(n+1) dot (A_i - A_(n+1))
$
We have a linear equation with $n+1$ variables and $n+1$ equations.
The linear equation can be solved by writing it into a matrix and inverting it.
$
  A'_i = A_i - A_(n+1)\
  mat(
    A'_1 dot A'_1, A'_1 dot A'_2, dots.c   , A'_1 dot A'_n, 0;
    A'_2 dot A'_1, A'_2 dot A'_2, dots.c   , A'_2 dot A'_n, 0;
    dots.v       , dots.v       , dots.down, dots.v       , dots.v;
    A'_n dot A'_1, A'_n dot A'_2, dots.c   , A'_n dot A'_n, 0;
    1            , 1            , dots.c   , 1            , 1;
  ) dot mat(t_1; t_2; dots.v; t_n; t_(n+1)) =
  mat(
    -A_(n+1) dot A'_1;
    -A_(n+1) dot A'_2;
    dots.v           ;
    -A_(n+1) dot A'_n;
    1;
  )
$

// ```rust
// type Simplex = Vec<Vector3>;

// trait Support {
//   fn support(&self, direction: Vector3) -> Vector3 {}
// }

// fn gjk(a: impl Support, b: impl Support) {
//   let mut s = vec![];
//   let mut dir = random();
//   loop {
//     let new_point = a_point - b_point;
//     if new_point.dot(dir) < 0 {
//       return todo!("calculate closest points");
//     }
//     s.push(new_point);
//     (s, dir, contains_origin) = best_simplex(s);
//     if contains_origin {
//       return todo!("calculate contact points and contact normal");
//     }
//   }
// }

// fn best_simplex(s: Simplex) -> (Simplex, Vector3, bool) {
//   match s {
//     One(a) => {
//       (One(a), -a, false)
//     },
//     Two(a, b) => {
//       let dir = (b - a).cross(-a).cross(b - a);
//       (Two(a, b), dir, false)
//     },
//     Three(a, b, c) => {
//       // háromszög síkjára merőleges
//       let abc_perp = (b-a).cross(c-a);

//       // háromszögből kifele mutat, ac-re merőleges
//       let ac_perp = abc_perp.cross(c-a);
//       // ha az origó egy irányba van az oldal normáljával
//       if ac_perp.dot(-c) > 0 {
//         // b-t kivesszük, mert nem kell
//         return (Two(a, c), ac_perp, false);
//       }

//       // háromszögből kifele mutat, bc-re merőleges
//       let bc_perp = (b-c).cross(abc_perp);
//       // ha az origó egy irányba van az oldal normáljával
//       if bc_perp.dot(-c) > 0 {
//         // a-t kivesszük, mert nem kell
//         return (Two(b, c), bc_perp, false);
//       // a háromszögön belül vagyunk
//       }

//       // abc_perp irányba van az origó
//       if abc_perp.dot(-c) > 0 {
//         (Three(a, b, c), abc_perp, false)
//       // -abc_perp irányba van az origó
//       } else {
//         (Three(a, b, c), -abc_perp, false)
//       }
//     },
//     Four(a, b, c, d) => {
//       // Az origó nem lehet az abc háromszög "alatt" és a d pont "fölött".
//       // Az abc háromszögre vetítve az origó nem lehet a háromszögön kívül.
//       // Tehát az origó az abc alapú hasábban van.

//       // Az origó lehet az abd háromszög síkján kívül.
//       //   Ha ott van, akkor lehet ad vagy a bd oldalon kívül
//       //   vagy az abd háromszög "fölött".
//       let abd_perp = (b-a).dot(d-a);
//       // Ha abd síkján kívül van
//       if abd_perp.dot(-d) > 0 {
//         return tetrahedron_triangle_subcheck(a, b, d, abd_perp);
//       }

//       // Az origó lehet a bcd háromszög síkján kívül.
//       //   Ha ott van, akkor lehet bd vagy a cd oldalon kívül
//       //   vagy a bcd háromszög "fölött".
//       let bcd_perp = (c-b).dot(d-b);
//       if bcd_perp.dot(-d) > 0 {
//         return tetrahedron_triangle_subcheck(b, c, d, bcd_perp);
//       }

//       // Az origó lehet a cad háromszög síkján kívül.
//       //   Ha ott van, akkor lehet cd vagy a ad oldalon kívül
//       //   vagy a bcd háromszög "fölött".
//       let cad_perp = (a-c).dot(d-c);
//       if cad_perp.dot(-d) > 0 {
//         return tetrahedron_triangle_subcheck(c, a, d, cad_perp);
//       }

//       // Ha nincs egyik háromszög síkján kívül sem, akkor a tetraéderben van.
//       return (Four(a, b, c, d), /**/, true);
//     }
//   }
// }

// // Az origó az xyd háromszög síkján kívül van
// //   Ha ott van, akkor lehet az xd vagy a yd oldalon kívül
// //   vagy az xyd háromszög "fölött"
// fn tetrahedron_triangle_subcheck(x, y, d, xyd_perp) -> (Simplex, Vector3, bool) {
//   // xd-re merőleges, kifelé mutat
//   let xd_perp = xyd_perp.cross(d-x);
//   // xd-n kívül van
//   if xd_perp.dot(-d) > 0 {
//     return (Two(x, d), xd_perp, false);
//   }

//   // yd-re merőleges, kifelé mutat
//   let yd_perp = (d-y).cross(xyd_perp);
//   // yd-n kívül van
//   if yd_perp.dot(-d) > 0 {
//     return (Two(y, d), yd_perp, false);
//   }

//   (Three(x, y, d), xyd_perp, false);
// }
// ```

// A `best_simplex` algoritmus feltételezi néhány invariáns teljesülését.
// Az invariánsok simplex típusonként:

// #let origo = $bold(accent(0, arrow))$

// - *One(a):* $a != origo$, azaz `a` nem az origó.
// - *Two(a, b):* $(origo-a) dot (b-a) > 0 and (origo-b) dot (a-b) > 0$, azaz az
//   `ab` egyenes nem tartalmazza az origót, `a` nem `b` és az origó az `ab`
//   szakasz által meghatározott sávban van.
// - *Three(a, b, c):* az origó az `ab` szakasz és a `c` pont által meghatározott
//   téglalap alapú végtelen hasábban van.
// - *Four(a, b, c, d):* az origó az előző hasábnak az `abc` háromszög és `d` pont
//   közötti részében található.
