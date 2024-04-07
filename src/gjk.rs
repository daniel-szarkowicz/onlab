use std::ops::{Add, Mul};

use nalgebra::{Const, Dyn, Matrix, Vector3};
use rand::random;

type Vec3 = Vector3<f64>;

pub trait Support {
    fn support(&self, direction: &Vec3) -> Vec3;
}

#[derive(Debug, Clone)]
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

pub fn gjk2(a: &impl Support, b: &impl Support) -> bool {
    let mut s = Vec::with_capacity(5);
    s.push(SimplexPoint::new(
        a,
        b,
        &Vec3::new(random(), random(), random()),
    ));
    let mut prev_dist = f64::INFINITY;
    // println!("start");
    loop {
        let closest_point: SimplexPoint;
        (closest_point, s) = closest_simplex(s);
        let dist = closest_point.diff.magnitude();
        // dbg!(dist);
        // assert!(dist <= prev_dist);
        // if dist > prev_dist {
        //     // return true;
        //     dbg!(dist, prev_dist);
        // }
        if dist <= 0.01 {
            // return (
            //     closest_point.a,
            //     closest_point.b,
            //     todo!("calculate normal"),
            // );
            return true;
        }
        if prev_dist - dist <= 0.001 {
            // no contact
            // return (
            //     closest_point.a,
            //     closest_point.b,
            //     (closest_point.b - closest_point.a).normalize(),
            // );
            return false;
        }
        prev_dist = dist;
        s.push(SimplexPoint::new(a, b, &-closest_point.diff));
    }
}

fn closest_simplex(
    mut s: Vec<SimplexPoint>,
) -> (SimplexPoint, Vec<SimplexPoint>) {
    // dbg!(s.len());
    match s.len() {
        0 => panic!("simplex has to contain at leas 1 point"),
        // 1 => (s[0].clone(), s),
        len => {
            let mut a_data = Vec::with_capacity(len * len);
            for v1 in &s {
                for v2 in &s[0..len - 1] {
                    a_data.push(v1.diff.dot(&v2.diff));
                }
                a_data.push(1.0);
            }
            let mut a_inverse =
                Matrix::from_vec_generic(Dyn(len), Dyn(len), a_data);

            if !a_inverse.try_inverse_mut() {
                println!("matrix not invertible");
                println!("matrix = {a_inverse:?}");
                println!("using pseudo inverse");
                a_inverse = a_inverse
                    .pseudo_inverse(0.01)
                    .inspect_err(|e| println!("{e}"))
                    .unwrap();
            }
            let multipliers = a_inverse.column(len - 1);
            // for (asdf, sp) in multipliers.iter().zip(&s) {
            //     print!(
            //         "{asdf} * ({}, {}, {}) + ",
            //         sp.diff.x, sp.diff.y, sp.diff.z
            //     );
            // }
            // let asdf = multipliers
            //     .iter()
            //     .zip(&s)
            //     .map(|(t, v)| *t * v)
            //     .reduce(|a, b| a + b)
            //     .unwrap();
            // println!(" = ({}, {}, {})", asdf.diff.x, asdf.diff.y, asdf.diff.z);
            if multipliers.iter().all(|v| v >= &0.0) {
                (
                    multipliers
                        .iter()
                        .zip(&s)
                        .map(|(t, v)| *t * v)
                        .reduce(|a, b| a + b)
                        .unwrap(),
                    s,
                )
            } else {
                for i in multipliers
                    .iter()
                    .enumerate()
                    .filter(|(_, d)| d < &&0.0)
                    .map(|(i, _)| i)
                    .rev()
                {
                    // println!("removing {i}");
                    s.swap_remove(i);
                }
                // println!("recursing");
                closest_simplex(s)
            }
        }
    }
}

impl Mul<&SimplexPoint> for f64 {
    type Output = SimplexPoint;

    fn mul(self, rhs: &SimplexPoint) -> Self::Output {
        SimplexPoint {
            diff: self * rhs.diff,
            a: self * rhs.a,
            b: self * rhs.b,
        }
    }
}

impl Add for SimplexPoint {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.diff += rhs.diff;
        self.a += rhs.a;
        self.b += rhs.b;
        self
    }
}

// impl Sum for SimplexPoint {
//     fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
//         todo!()
//     }
// }

/*
simplex = init
prev dist = inf
loop
    p, simplex = closest point and simplex
    assert distance didn't increase
    compare distance to prev, exit if neccesary
    add new point to simplex with direction -p
*/
