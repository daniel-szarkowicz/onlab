use nalgebra::Vector3;
use rand::random;

type Vec3 = Vector3<f64>;

pub trait Support {
    fn support(&self, direction: &Vec3) -> Vec3;
}

#[derive(Debug)]
struct SimplexPoint {
    diff: Vec3,
    a: Vec3,
    b: Vec3,
}

impl SimplexPoint {
    fn new(a: &impl Support, b: &impl Support, dir: &Vec3) -> Self {
        let a = a.support(dir);
        let b = b.support(&-dir);
        Self { diff: a - b, a, b }
    }
}

pub fn gjk(a: &impl Support, b: &impl Support) -> bool {
    let mut dir = Vec3::new(random(), random(), random());
    let mut s = Vec::with_capacity(5);
    loop {
        // if dir.magnitude() < 0.001 {
        //     println!("simplex contains origin");
        //     return true;
        // }
        assert!(dir.magnitude() > f64::EPSILON);
        // dir.normalize_mut();
        let new_point = SimplexPoint::new(a, b, &dir);
        // println!("dir dot {:?}", new_point.diff.dot(&dir.normalize()));
        if new_point.diff.dot(&dir) < 0.001 {
            return false;
            // todo!("calculate closest points");
        }
        // println!("distance form origin: {}", new_point.diff.magnitude());
        // if new_point.diff.magnitude() < 0.001 {
        //     dbg!("close to the origin");
        //     return true;
        // }
        s.push(new_point);
        // dbg!(s.len());

        // for p in &s {
        //     println!("({}, {}, {}),", p.diff.x, p.diff.y, p.diff.z);
        // }
        let contains_origin;
        (s, dir, contains_origin) = best_simplex(s);
        // dbg!(s.len(), dir);
        if contains_origin {
            return true;
            // todo!("calculate contact points and contact normal");
        }
    }
}

#[allow(clippy::similar_names)]
fn best_simplex(mut s: Vec<SimplexPoint>) -> (Vec<SimplexPoint>, Vec3, bool) {
    match s.len() {
        1 => {
            let dir = -s[0].diff;
            (s, dir, false)
        }
        2 => {
            let dir = (s[1].diff - s[0].diff)
                .cross(&-s[0].diff)
                .cross(&(s[1].diff - s[0].diff));
            (s, dir, false)
        }
        3 => {
            // háromszög síkjára merőleges
            let abc_perp =
                (s[1].diff - s[0].diff).cross(&(s[2].diff - s[0].diff));

            // háromszögből kifele mutat, ac-re merőleges
            let ac_perp = abc_perp.cross(&(s[2].diff - s[0].diff));
            // ha az origó egy irányba van az oldal normáljával
            if ac_perp.dot(&-s[2].diff) > 0.0 {
                // b-t kivesszük, mert nem kell
                s.remove(1);
                let dir = (s[1].diff - s[0].diff)
                    .cross(&-s[0].diff)
                    .cross(&(s[1].diff - s[0].diff));
                // println!("ac");
                return (s, dir, false);
            }

            // háromszögből kifele mutat, bc-re merőleges
            let bc_perp = (s[2].diff - s[1].diff).cross(&abc_perp);
            // ha az origó egy irányba van az oldal normáljával
            if bc_perp.dot(&-s[2].diff) > 0.0 {
                // a-t kivesszük, mert nem kell
                s.remove(0);
                let dir = (s[1].diff - s[0].diff)
                    .cross(&-s[0].diff)
                    .cross(&(s[1].diff - s[0].diff));
                // println!("bc");
                return (s, dir, false);
                // a háromszögön belül vagyunk
            }

            // abc_perp irányba van az origó
            if abc_perp.dot(&-s[2].diff) > 0.0 {
                (s, abc_perp, false)
            // -abc_perp irányba van az origó
            } else {
                s.reverse();
                (s, -abc_perp, false)
            }
            // let dot = abc_perp.dot(&-s[2].diff);
            // if dot < -f64::EPSILON {
            //     // println!("origin \"below\" triangle");
            //     s.reverse();
            //     (s, -abc_perp, false)
            // } else if dot <= f64::EPSILON {
            //     // println!("triangle contains origin");
            //     (s, Vec3::zeros(), true)
            // } else {
            //     // println!("origin \"above\" triangle");
            //     (s, abc_perp, false)
            // }
        }
        4 => {
            // Az origó nem lehet az abc háromszög "alatt" és a d pont "fölött".
            // Az abc háromszögre vetítve az origó nem lehet a háromszögön kívül.
            // Tehát az origó az abc alapú hasábban van.

            // Az origó lehet az abd háromszög síkján kívül.
            //   Ha ott van, akkor lehet ad vagy a bd oldalon kívül
            //   vagy az abd háromszög "fölött".
            let abd_perp =
                (s[1].diff - s[0].diff).cross(&(s[3].diff - s[0].diff));
            // Ha abd síkján kívül van
            assert!(
                abd_perp.dot(&(s[2].diff - s[3].diff)) < 0.0,
                "abd={abd_perp:?}"
            );
            if abd_perp.dot(&-s[3].diff) > 0.0 {
                // println!("origin \"above\" abd");
                s.remove(2);
                return tetrahedron_triangle_subcheck(s, abd_perp);
            }

            // Az origó lehet a bcd háromszög síkján kívül.
            //   Ha ott van, akkor lehet bd vagy a cd oldalon kívül
            //   vagy a bcd háromszög "fölött".
            let bcd_perp =
                (s[2].diff - s[1].diff).cross(&(s[3].diff - s[1].diff));
            assert!(
                bcd_perp.dot(&(s[0].diff - s[3].diff)) < 0.0,
                "bcd={bcd_perp:?}"
            );
            if bcd_perp.dot(&-s[3].diff) > 0.0 {
                // println!("origin \"above\" bcd");
                s.remove(0);
                return tetrahedron_triangle_subcheck(s, bcd_perp);
            }

            // Az origó lehet a cad háromszög síkján kívül.
            //   Ha ott van, akkor lehet cd vagy a ad oldalon kívül
            //   vagy a bcd háromszög "fölött".
            let cad_perp =
                (s[0].diff - s[2].diff).cross(&(s[3].diff - s[2].diff));
            assert!(
                cad_perp.dot(&(s[1].diff - s[3].diff)) < 0.0,
                "cad={cad_perp:?}"
            );
            if cad_perp.dot(&-s[3].diff) > 0.0 {
                // println!("origin \"above\" cad");
                s.remove(1);
                let (s1, s2) = s.split_at_mut(1);
                std::mem::swap(&mut s1[0], &mut s2[0]);
                return tetrahedron_triangle_subcheck(s, cad_perp);
            }

            // Ha nincs egyik háromszög síkján kívül sem, akkor a tetraéderben van.
            (s, Vec3::zeros(), true)
            // todo!()
        }
        _ => unreachable!(),
    }
}

#[allow(clippy::similar_names)]
fn tetrahedron_triangle_subcheck(
    mut s: Vec<SimplexPoint>,
    xyd_perp: Vec3,
) -> (Vec<SimplexPoint>, Vec3, bool) {
    assert!(s.len() == 3);
    // xd-re merőleges, kifelé mutat
    // dbg!(&xyd_perp);
    let xd_perp = xyd_perp.cross(&(s[2].diff - s[0].diff));
    // xd-n kívül van
    if xd_perp.dot(&-s[2].diff) > 0.0 {
        s.remove(1);
        let dir = (s[1].diff - s[0].diff)
            .cross(&-s[0].diff)
            .cross(&(s[1].diff - s[0].diff));
        // println!("xd");
        return (s, dir, false);
    }

    // yd-re merőleges, kifelé mutat
    let yd_perp = (s[2].diff - s[1].diff).cross(&xyd_perp);
    // yd-n kívül van
    if yd_perp.dot(&-s[2].diff) > 0.0 {
        s.remove(0);
        let dir = (s[1].diff - s[0].diff)
            .cross(&-s[0].diff)
            .cross(&(s[1].diff - s[0].diff));
        // println!("yd");
        return (s, dir, false);
    }

    (s, xyd_perp, false)
}
