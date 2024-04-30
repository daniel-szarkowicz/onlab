use std::ops::{Add, Mul, Sub};

use nalgebra::{Const, DimMin, Matrix, Vector3};
use rand::random;
use smallvec::SmallVec;

type SimplexData = SmallVec<[SupportPoint; 4]>;
// type SimplexData = Vec<SupportPoint>;

type Vec3 = Vector3<f64>;

const TOLERANCE: f64 = 1e-7;
const SIMPLEX_MAX_DIM: usize = 4;
const EPA_MAX_ITER: usize = 1000;
const GJK_MAX_ITER: usize = 12;

pub trait Support {
    fn support(&self, direction: &Vec3) -> Vec3;
    fn radius(&self) -> f64;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SupportPoint {
    pub diff: Vec3,
    pub a: Vec3,
}

impl SupportPoint {
    pub fn new(a: &impl Support, b: &impl Support, dir: &Vec3) -> Self {
        let a = a.support(dir);
        let b = b.support(&-dir);
        Self { diff: a - b, a }
    }
}

pub fn gjk(a: &impl Support, b: &impl Support) -> GJKResult {
    let mut s = SimplexData::with_capacity(4);
    s.push(SupportPoint::new(
        a,
        b,
        &Vec3::new(random(), random(), random()),
    ));
    let mut prev_dist = f64::INFINITY;
    let mut closest_point = closest_simplex::<false>(&mut s);
    let mut dist_diff = 0.0;
    for _ in 0..GJK_MAX_ITER {
        let dist = closest_point.diff.magnitude();
        // debug_assert!(
        //     dist <= prev_dist + TOLERANCE,
        //     "prev_dist={prev_dist}, dist={dist}"
        // );
        if s.len() == SIMPLEX_MAX_DIM {
            return epa(a, b, s.into_vec());
        }
        // debug_assert!(
        //     dist > TOLERANCE,
        //     "if dist={dist} is smaller than TOLERANCE={TOLERANCE} \
        //      the simplex should have {SIMPLEX_MAX_DIM} points"
        // );
        dist_diff = prev_dist - dist;
        if prev_dist - dist <= TOLERANCE {
            return closest_point_to_contact(a, b, &closest_point);
        }
        prev_dist = dist;
        let new_point = SupportPoint::new(a, b, &-closest_point.diff);
        if closest_point
            .diff
            .dot(&(new_point.diff - closest_point.diff))
            >= -TOLERANCE
        {
            return closest_point_to_contact(a, b, &closest_point);
        }
        s.push(new_point);
        closest_point = closest_simplex::<false>(&mut s);
    }
    eprintln!(
        "gjk didn't converge in {GJK_MAX_ITER} steps \
        (dist = {prev_dist:0.10}, diff = {dist_diff:0.10})"
    );
    closest_point_to_contact(a, b, &closest_point)
}

#[allow(clippy::similar_names)]
fn best_simplex(s: &mut SimplexData) -> (Vec3, bool) {
    match s.len() {
        1 => {
            let dir = -s[0].diff;
            (dir, false)
        }
        2 => {
            let ab = s[1].diff - s[0].diff;
            if ab.dot(&-s[0].diff) < 0.0 {
                s.remove(0);
                return best_simplex(s);
            }
            if ab.dot(&-s[1].diff) > 0.0 {
                s.remove(1);
                return best_simplex(s);
            }
            let dir = (s[1].diff - s[0].diff)
                .cross(&-s[0].diff)
                .cross(&(s[1].diff - s[0].diff));
            (dir, false)
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
                return best_simplex(s);
            }

            // háromszögből kifele mutat, bc-re merőleges
            let bc_perp = (s[2].diff - s[1].diff).cross(&abc_perp);
            // ha az origó egy irányba van az oldal normáljával
            if bc_perp.dot(&-s[2].diff) > 0.0 {
                // a-t kivesszük, mert nem kell
                s.remove(0);
                return best_simplex(s);
            }

            let ab_perp = (s[1].diff - s[0].diff).cross(&abc_perp);
            if ab_perp.dot(&-s[1].diff) > 0.0 {
                s.remove(2);
                return best_simplex(s);
            }

            // abc_perp irányba van az origó
            if abc_perp.dot(&-s[2].diff) > 0.0 {
                (abc_perp, false)
            // -abc_perp irányba van az origó
            } else {
                s.reverse();
                (-abc_perp, false)
            }
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
            debug_assert!(
                abd_perp.dot(&(s[2].diff - s[3].diff)) < 0.0,
                "abd={abd_perp:?}"
            );
            if abd_perp.dot(&-s[3].diff) > 0.0 {
                s.remove(2);
                return tetrahedron_triangle_subcheck(s, abd_perp);
            }

            // Az origó lehet a bcd háromszög síkján kívül.
            //   Ha ott van, akkor lehet bd vagy a cd oldalon kívül
            //   vagy a bcd háromszög "fölött".
            let bcd_perp =
                (s[2].diff - s[1].diff).cross(&(s[3].diff - s[1].diff));
            debug_assert!(
                bcd_perp.dot(&(s[0].diff - s[3].diff)) < 0.0,
                "bcd={bcd_perp:?}"
            );
            if bcd_perp.dot(&-s[3].diff) > 0.0 {
                s.remove(0);
                return tetrahedron_triangle_subcheck(s, bcd_perp);
            }

            // Az origó lehet a cad háromszög síkján kívül.
            //   Ha ott van, akkor lehet cd vagy a ad oldalon kívül
            //   vagy a bcd háromszög "fölött".
            let cad_perp =
                (s[0].diff - s[2].diff).cross(&(s[3].diff - s[2].diff));
            debug_assert!(
                cad_perp.dot(&(s[1].diff - s[3].diff)) < 0.0,
                "cad={cad_perp:?}"
            );
            if cad_perp.dot(&-s[3].diff) > 0.0 {
                s.remove(1);
                let (s1, s2) = s.split_at_mut(1);
                std::mem::swap(&mut s1[0], &mut s2[0]);
                return tetrahedron_triangle_subcheck(s, cad_perp);
            }

            // Ha nincs egyik háromszög síkján kívül sem, akkor a tetraéderben van.
            (Vec3::zeros(), true)
        }
        _ => unreachable!(),
    }
}

#[allow(clippy::similar_names)]
fn tetrahedron_triangle_subcheck(
    s: &mut SimplexData,
    xyd_perp: Vec3,
) -> (Vec3, bool) {
    debug_assert!(s.len() == 3);
    // xd-re merőleges, kifelé mutat
    let xd_perp = xyd_perp.cross(&(s[2].diff - s[0].diff));
    // xd-n kívül van
    if xd_perp.dot(&-s[2].diff) > 0.0 {
        s.remove(1);
        let dir = (s[1].diff - s[0].diff)
            .cross(&-s[0].diff)
            .cross(&(s[1].diff - s[0].diff));
        return (dir, false);
    }

    // yd-re merőleges, kifelé mutat
    let yd_perp = (s[2].diff - s[1].diff).cross(&xyd_perp);
    // yd-n kívül van
    if yd_perp.dot(&-s[2].diff) > 0.0 {
        s.remove(0);
        let dir = (s[1].diff - s[0].diff)
            .cross(&-s[0].diff)
            .cross(&(s[1].diff - s[0].diff));
        return (dir, false);
    }
    (xyd_perp, false)
}

fn closest_point_to_contact(
    a: &impl Support,
    b: &impl Support,
    closest_point: &SupportPoint,
) -> GJKResult {
    if closest_point.diff.magnitude() <= a.radius() + b.radius() {
        let normal = closest_point.diff;
        let b_point = closest_point.a - closest_point.diff;
        GJKResult::Contact {
            points: (
                closest_point.a
                    - normal * a.radius() / (a.radius() + b.radius()),
                b_point + normal * b.radius() / (a.radius() + b.radius()),
            ),
            normal: normal.normalize(),
        }
    } else {
        GJKResult::NoContact
    }
}

fn closest_simplex<const DETCHECK: bool>(s: &mut SimplexData) -> SupportPoint {
    let (_, _) = best_simplex(s);
    match s.len() {
        0 => panic!("simplex has to contain at least 1 point"),
        1 => s[0].clone(),
        2 if !DETCHECK => {
            let t = (-s[1].diff.dot(&(s[0].diff - s[1].diff))
                / (s[0].diff - s[1].diff).magnitude_squared())
            .clamp(0.0, 1.0);
            s[1].clone() + t * &(s[0].clone() - s[1].clone())
        }
        2 => closest_simplex_static::<2, DETCHECK>(s),
        3 => closest_simplex_static::<3, DETCHECK>(s),
        4 => closest_simplex_static::<4, DETCHECK>(s),
        _ => unreachable!(),
    }
}

fn closest_simplex_static<const N: usize, const DETCHECK: bool>(
    s: &mut SimplexData,
) -> SupportPoint
where
    Const<N>: DimMin<Const<N>, Output = Const<N>>,
{
    let mut a = Matrix::zeros_generic(Const::<N>, Const::<N>);
    a.data.0[0][0] = 1.0;
    for i in 1..N {
        a.data.0[i][0] = 1.0f64;
        for j in 1..N {
            a.data.0[i][j] =
                (s[i].diff - s[0].diff).dot(&(s[j].diff - s[0].diff));
        }
    }
    if DETCHECK {
        let det = a.determinant();
        if det.abs() <= TOLERANCE {
            s.pop();
            return closest_simplex::<DETCHECK>(s);
        }
    }
    let a_inverse = a.try_inverse().expect("a is invertible");
    let mut b = Matrix::zeros_generic(Const::<N>, Const::<1>);
    b[0] = 1.0;
    for i in 1..N {
        b[i] = -s[0].diff.dot(&(s[i].diff - s[0].diff));
    }
    let multipliers = a_inverse * b;
    multipliers
        .iter()
        .inspect(|m| {
            assert!(m >= &&0.0, "invalid multiplier m={m}, should be >= 0");
        })
        .zip(&*s)
        .map(|(t, v)| *t * v)
        .reduce(|a, b| a + b)
        .unwrap()
}

pub fn epa(
    a: &impl Support,
    b: &impl Support,
    mut points: Vec<SupportPoint>,
) -> GJKResult {
    // eprintln!("============ START =============");
    debug_assert_eq!(points.len(), 4);
    let mut faces = vec![[0, 1, 2], [0, 2, 3], [0, 3, 1], [1, 2, 3]];
    let mut closest_points = vec![];
    let mut tmp = SimplexData::new();
    for [v1, v2, v3] in &faces {
        tmp.push(points[*v1].clone());
        tmp.push(points[*v2].clone());
        tmp.push(points[*v3].clone());
        let closest_point = closest_simplex::<true>(&mut tmp);
        tmp.clear();
        closest_points.push(closest_point);
    }
    let mut iter = 0;
    loop {
        // eprintln!("---------- LOOP ----------");
        // for point in &points {
        //     eprintln!("{:?}", point.diff);
        // }
        // dbg!(&faces);
        let Some(minface) = closest_points
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                a.diff.magnitude().total_cmp(&b.diff.magnitude())
            })
            .map(|(i, _)| i)
        else {
            eprintln!("math has failed!");
            return GJKResult::NoContact;
        };
        let new_point = SupportPoint::new(a, b, &closest_points[minface].diff);
        // dbg!(&closest_points[minface].diff);
        // dbg!(&new_point.diff);
        // dbg!(new_point.diff.dot(&closest_points[minface].diff));
        // dbg!(closest_points[minface].diff.magnitude_squared());
        debug_assert!(
            new_point.diff.magnitude()
                >= closest_points[minface].diff.magnitude() - TOLERANCE,
        );
        if new_point.diff.dot(&closest_points[minface].diff)
            <= closest_points[minface].diff.magnitude_squared() + TOLERANCE
            || iter == EPA_MAX_ITER
        {
            if iter == EPA_MAX_ITER {
                eprintln!("epa max reached");
            }
            // todo!("we have found the best, we can return");
            let b_point =
                closest_points[minface].a - closest_points[minface].diff;
            return GJKResult::Contact {
                points: (closest_points[minface].a, b_point),
                normal: -closest_points[minface].diff.normalize(),
            };
        }
        // todo!();
        // remove faces that are "below" the new point
        // collect the edges that only one of the removed faces had
        let mut edges = vec![];
        let mut i = 0;
        debug_assert_eq!(faces.len(), closest_points.len());
        while i < faces.len() {
            if closest_points[i]
                .diff
                .dot(&(new_point.diff - closest_points[i].diff))
                >= 0.0
            {
                // eprintln!("removing face {i}");
                edges.add_or_remove(minmax(faces[i][0], faces[i][1]));
                edges.add_or_remove(minmax(faces[i][1], faces[i][2]));
                edges.add_or_remove(minmax(faces[i][2], faces[i][0]));
                faces.swap_remove(i);
                closest_points.swap_remove(i);
            } else {
                i += 1;
            }
        }
        debug_assert_eq!(faces.len(), closest_points.len());

        // add new faces with the collected edges and the new point
        let mut new_faces = vec![];
        for (i, j) in edges {
            new_faces.push([i, j, points.len()]);
            // eprintln!("adding new face [{i}, {j}, new]");
        }
        points.push(new_point);
        // calculate closest points for the new faces
        let mut new_closest_points = vec![];
        for [v1, v2, v3] in &new_faces {
            tmp.push(points[*v1].clone());
            tmp.push(points[*v2].clone());
            tmp.push(points[*v3].clone());
            let closest_point = closest_simplex::<true>(&mut tmp);
            tmp.clear();
            new_closest_points.push(closest_point);
        }
        faces.append(&mut new_faces);
        closest_points.append(&mut new_closest_points);
        iter += 1;
    }
}

#[derive(Debug)]
pub enum GJKResult {
    Contact { points: (Vec3, Vec3), normal: Vec3 },
    UnknownContact(Vec<SupportPoint>),
    NoContact,
}

impl Mul<&SupportPoint> for f64 {
    type Output = SupportPoint;

    fn mul(self, rhs: &SupportPoint) -> Self::Output {
        SupportPoint {
            diff: self * rhs.diff,
            a: self * rhs.a,
        }
    }
}

impl Add for SupportPoint {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.diff += rhs.diff;
        self.a += rhs.a;
        self
    }
}

impl Sub for SupportPoint {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.diff -= rhs.diff;
        self.a -= rhs.a;
        self
    }
}

trait AddOrRemove<T: PartialEq> {
    fn add_or_remove(&mut self, elem: T);
}

impl<T: PartialEq> AddOrRemove<T> for Vec<T> {
    fn add_or_remove(&mut self, elem: T) {
        if let Some(index) = self.iter().position(|e| e == &elem) {
            self.swap_remove(index);
        } else {
            self.push(elem);
        }
    }
}

fn minmax<T: Ord>(a: T, b: T) -> (T, T) {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}
