```rust
type Simplex = Vec<Vector3>;

trait Support {
  fn support(&self, direction: Vector3) -> Vector3 {}
}

fn gjk(a: impl Support, b: impl Support) {
  let mut s = vec![];
  let mut dir = random();
  loop {
    let new_point = a_point - b_point;
    if new_point.dot(dir) < 0 {
      return todo!("calculate closest points");
    }
    s.push(new_point);
    (s, dir, contains_origin) = best_simplex(s);
    if contains_origin {
      return todo!("calculate contact points and contact normal");
    }
  }
}

fn best_simplex(s: Simplex) -> (Simplex, Vector3, bool) {
  match s {
    One(a) => {
      (One(a), -a, false)
    },
    Two(a, b) => {
      let dir = (b - a).cross(-a).cross(b - a);
      (Two(a, b), dir, false)
    },
    Three(a, b, c) => {
      // háromszög síkjára merőleges
      let abc_perp = (b-a).cross(c-a);

      // háromszögből kifele mutat, ac-re merőleges
      let ac_perp = abc_perp.cross(c-a);
      // ha az origó egy irányba van az oldal normáljával
      if ac_perp.dot(-c) > 0 {
        // b-t kivesszük, mert nem kell
        return (Two(a, c), ac_perp, false);
      }

      // háromszögből kifele mutat, bc-re merőleges
      let bc_perp = (b-c).cross(abc_perp);
      // ha az origó egy irányba van az oldal normáljával
      if bc_perp.dot(-c) > 0 {
        // a-t kivesszük, mert nem kell
        return (Two(b, c), bc_perp, false);
      // a háromszögön belül vagyunk
      }

      // abc_perp irányba van az origó
      if abc_perp.dot(-c) > 0 {
        (Three(a, b, c), abc_perp, false)
      // -abc_perp irányba van az origó
      } else {
        (Three(a, b, c), -abc_perp, false)
      }
    },
    Four(a, b, c, d) => {
      // Az origó nem lehet az abc háromszög "alatt" és a d pont "fölött".
      // Az abc háromszögre vetítve az origó nem lehet a háromszögön kívül.
      // Tehát az origó az abc alapú hasábban van.

      // Az origó lehet az abd háromszög síkján kívül.
      //   Ha ott van, akkor lehet ad vagy a bd oldalon kívül
      //   vagy az abd háromszög "fölött".
      let abd_perp = (b-a).dot(d-a);
      // Ha abd síkján kívül van
      if abd_perp.dot(-d) > 0 {
        return tetrahedron_triangle_subcheck(a, b, d, abd_perp);
      }

      // Az origó lehet a bcd háromszög síkján kívül.
      //   Ha ott van, akkor lehet bd vagy a cd oldalon kívül
      //   vagy a bcd háromszög "fölött".
      let bcd_perp = (c-b).dot(d-b);
      if bcd_perp.dot(-d) > 0 {
        return tetrahedron_triangle_subcheck(b, c, d, bcd_perp);
      }

      // Az origó lehet a cad háromszög síkján kívül.
      //   Ha ott van, akkor lehet cd vagy a ad oldalon kívül
      //   vagy a bcd háromszög "fölött".
      let cad_perp = (a-c).dot(d-c);
      if cad_perp.dot(-d) > 0 {
        return tetrahedron_triangle_subcheck(c, a, d, cad_perp);
      }

      // Ha nincs egyik háromszög síkján kívül sem, akkor a tetraéderben van.
      return (Four(a, b, c, d), /**/, true);
    }
  }
}

// Az origó az xyd háromszög síkján kívül van
//   Ha ott van, akkor lehet az xd vagy a yd oldalon kívül
//   vagy az xyd háromszög "fölött"
fn tetrahedron_triangle_subcheck(x, y, d, xyd_perp) -> (Simplex, Vector3, bool) {
  // xd-re merőleges, kifelé mutat
  let xd_perp = xyd_perp.cross(d-x);
  // xd-n kívül van
  if xd_perp.dot(-d) > 0 {
    return (Two(x, d), xd_perp, false);
  }

  // yd-re merőleges, kifelé mutat
  let yd_perp = (d-y).cross(xyd_perp);
  // yd-n kívül van
  if yd_perp.dot(-d) > 0 {
    return (Two(y, d), yd_perp, false);
  }

  (Three(x, y, d), xyd_perp, false);
}
```
