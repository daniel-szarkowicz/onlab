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
        // dbg!(&s);
        // dbg!(closest_point.diff);
        debug_assert!(
            dist <= prev_dist + TOLERANCE,
            "prev_dist={prev_dist}, dist={dist}"
        );
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
        // eprintln!("before: {}", s.len());
        closest_point = closest_simplex::<false>(&mut s);
        // eprintln!("after: {}", s.len());
        // dbg!(&s);
    }
    eprintln!(
        "gjk didn't converge in {GJK_MAX_ITER} steps \
        (dist = {prev_dist:0.10}, diff = {dist_diff:0.10})"
    );
    closest_point_to_contact(a, b, &closest_point)
}

#[allow(clippy::similar_names)]
#[allow(clippy::too_many_lines)]
fn best_simplex(s: &mut SimplexData) {
    // dbg!(&s);
    match s.len() {
        1 => {}
        2 => {
            let ab = s[1].diff - s[0].diff;
            if ab.dot(&-s[0].diff) < 0.0 {
                s.remove(1);
                // return best_simplex(s);
                return;
            }
            if ab.dot(&-s[1].diff) > 0.0 {
                s.remove(0);
                // best_simplex(s);
            }
        }
        3 => {
            // háromszög síkjára merőleges
            let abc_perp =
                (s[1].diff - s[0].diff).cross(&(s[2].diff - s[0].diff));

            // háromszögből kifele mutat, ac-re merőleges
            let ac_perp = abc_perp.cross(&(s[2].diff - s[0].diff));
            // debug_assert!(ac_perp.dot(&(s[1].diff - s[0].diff)) < 0.0);
            // ha az origó egy irányba van az oldal normáljával
            let ac = ac_perp.dot(&-s[2].diff) > 0.0;
            // háromszögből kifele mutat, bc-re merőleges
            let bc_perp = (s[2].diff - s[1].diff).cross(&abc_perp);
            // debug_assert!(bc_perp.dot(&(s[0].diff - s[1].diff)) < 0.0);
            // ha az origó egy irányba van az oldal normáljával
            let bc = bc_perp.dot(&-s[2].diff) > 0.0;
            let ab_perp = (s[1].diff - s[0].diff).cross(&abc_perp);
            // debug_assert!(ab_perp.dot(&(s[2].diff - s[0].diff)) < 0.0);
            let ab = ab_perp.dot(&-s[1].diff) > 0.0;

            match (ab, bc, ac) {
                (true, true, true) => unreachable!(
                    "point cannot be on all three sides of a triangle"
                ),
                (true, true, false) => {
                    triangle_two_sides_subcheck(s, ab_perp, bc_perp);
                }
                (false, true, true) => {
                    s.rotate_left(1);
                    triangle_two_sides_subcheck(s, bc_perp, ac_perp);
                }
                (true, false, true) => {
                    s.rotate_left(2);
                    triangle_two_sides_subcheck(s, ac_perp, ab_perp);
                }
                (true, false, false) => {
                    s.remove(2);
                    best_simplex(s);
                }
                (false, true, false) => {
                    s.remove(0);
                    best_simplex(s);
                }
                (false, false, true) => {
                    s.remove(1);
                    best_simplex(s);
                }
                (false, false, false) => {
                    if abc_perp.dot(&-s[2].diff) <= 0.0 {
                        s.reverse();
                    }
                }
            }

            // if ac {
            //     // b-t kivesszük, mert nem kell
            //     s.remove(1);
            //     return best_simplex(s);
            // }

            // if bc {
            //     // a-t kivesszük, mert nem kell
            //     s.remove(0);
            //     return best_simplex(s);
            // }

            // if ab {
            //     s.remove(2);
            //     return best_simplex(s);
            // }

            // abc_perp irányba van az origó
        }
        4 => {
            let abd_perp =
                (s[1].diff - s[0].diff).cross(&(s[3].diff - s[0].diff));
            let bcd_perp =
                (s[2].diff - s[1].diff).cross(&(s[3].diff - s[1].diff));
            let cad_perp =
                (s[0].diff - s[2].diff).cross(&(s[3].diff - s[2].diff));
            debug_assert!(
                abd_perp.dot(&(s[2].diff - s[3].diff)) < 0.0,
                "abd={abd_perp:?}"
            );
            debug_assert!(
                bcd_perp.dot(&(s[0].diff - s[3].diff)) < 0.0,
                "bcd={bcd_perp:?}"
            );
            debug_assert!(
                cad_perp.dot(&(s[1].diff - s[3].diff)) < 0.0,
                "cad={cad_perp:?}"
            );
            let abd = abd_perp.dot(&-s[3].diff) > 0.0;
            let bcd = bcd_perp.dot(&-s[3].diff) > 0.0;
            let cad = cad_perp.dot(&-s[3].diff) > 0.0;

            // dbg!(abd, bcd, cad);
            match (abd, bcd, cad) {
                (true, true, true) => {
                    // tetrahedron_two_sides_subcheck(s, abd_perp, bcd_perp);
                    let ad_perp = abd_perp.cross(&(s[3].diff - s[0].diff));
                    // eprintln!("three sides");
                    // dbg!(&s);
                    if ad_perp.dot(&-s[3].diff) < 0.0 {
                        // eprintln!("first case");
                        tetrahedron_two_sides_subcheck(s, abd_perp, bcd_perp);
                    } else {
                        // eprintln!("second case");
                        // dbg!("before", &s);
                        s[0..3].rotate_left(1);
                        tetrahedron_two_sides_subcheck(s, bcd_perp, cad_perp);
                        // dbg!("after", &s);
                    }
                    // dbg!(&s);
                }
                (true, true, false) => {
                    tetrahedron_two_sides_subcheck(s, abd_perp, bcd_perp);
                }
                (false, true, true) => {
                    s[0..3].rotate_left(1);
                    tetrahedron_two_sides_subcheck(s, bcd_perp, cad_perp);
                }
                (true, false, true) => {
                    s[0..3].rotate_left(2);
                    tetrahedron_two_sides_subcheck(s, cad_perp, abd_perp);
                }
                (true, false, false) => {
                    s.remove(2);
                    tetrahedron_triangle_subcheck(s, abd_perp);
                }
                (false, true, false) => {
                    s[0..3].rotate_left(1);
                    s.remove(2);
                    tetrahedron_triangle_subcheck(s, bcd_perp);
                }
                (false, false, true) => {
                    s[0..3].rotate_left(2);
                    s.remove(2);
                    // let (s1, s2) = s.split_at_mut(1);
                    // std::mem::swap(&mut s1[0], &mut s2[0]);
                    tetrahedron_triangle_subcheck(s, cad_perp);
                }
                (false, false, false) => {}
            }
        }
        _ => unreachable!(),
    }
}

// the edges with index 0, 1 and 1, 2 both have the origin above them
// the shared vertex is 1
fn triangle_two_sides_subcheck(s: &mut SimplexData, perp1: Vec3, perp2: Vec3) {
    let edgevec1 = s[1].diff - s[0].diff;
    if edgevec1.dot(&-s[1].diff) > 0.0 {
        s.remove(0);
        return best_simplex(s);
    }
    // let edgevec2 = s[1].diff - s[2].diff;
    // if edgevec2.dot(&-s[1].diff) > 0.0 {
    //     s.remove(2);
    //     return best_simplex(s);
    // }
    s.remove(2);
    // s.remove(0);
    best_simplex(s);
}

// the faces with index 0, 1, 3 and 1, 2, 3 both have the origin above them
// the shared edge is 1, 3
fn tetrahedron_two_sides_subcheck(
    s: &mut SimplexData,
    perp1: Vec3,
    perp2: Vec3,
) {
    eprintln!("two sides");
    let out1_1 = (s[3].diff - s[1].diff).cross(&perp1);
    let out1_2 = perp1.cross(&(s[3].diff - s[0].diff));
    debug_assert!(out1_1.dot(&(s[3].diff - s[0].diff)) > 0.0);
    debug_assert!(out1_2.dot(&(s[3].diff - s[1].diff)) > 0.0);

    let out2_1 = perp2.cross(&(s[3].diff - s[1].diff));
    let out2_2 = (s[3].diff - s[2].diff).cross(&perp2);
    debug_assert!(out2_1.dot(&(s[3].diff - s[2].diff)) > 0.0);
    debug_assert!(out2_2.dot(&(s[3].diff - s[1].diff)) > 0.0);

    let c1_1 = out1_1.dot(&-s[3].diff) < 0.0;
    let c1_2 = out1_2.dot(&-s[3].diff) < 0.0;
    let c2_1 = out2_1.dot(&-s[3].diff) < 0.0;
    let c2_2 = out2_2.dot(&-s[3].diff) < 0.0;

    eprintln!("c1_1 = {c1_1}, c1_2 = {c1_2}, c2_1 = {c2_1}, c2_2 = {c2_2}");

    // dbg!(&s);
    if c1_1 && c1_2 {
        // it is inside the first face
        s.remove(2);
        // return tetrahedron_triangle_subcheck(s, perp1);
        return best_simplex(s);
    }

    if c2_1 && c2_2 {
        // it is inside the second face
        s.remove(0);
        return best_simplex(s);
    }

    // it is on one of the edges
    let e1_1 = -s[0].diff.dot(&(s[0].diff - s[3].diff)) < 0.0;
    let e1_2 = -s[3].diff.dot(&(s[3].diff - s[0].diff)) < 0.0;

    let e2_1 = -s[1].diff.dot(&(s[1].diff - s[3].diff)) < 0.0;
    let e2_2 = -s[3].diff.dot(&(s[3].diff - s[1].diff)) < 0.0;

    let e3_1 = -s[2].diff.dot(&(s[2].diff - s[3].diff)) < 0.0;
    let e3_2 = -s[3].diff.dot(&(s[3].diff - s[2].diff)) < 0.0;

    eprintln!("e1_1 = {e1_1}, e1_2 = {e1_2}, e2_1 = {e2_1}, e2_2 = {e2_2}, e3_1 = {e3_1}, e3_2 = {e3_2}");
    dbg!(&s);

    if e1_1 && e1_2 {
        s.remove(2);
        s.remove(1);
        return;
    }

    if e2_1 && e2_2 {
        s.remove(2);
        s.remove(0);
        return;
    }

    if e3_1 && e3_2 {
        s.remove(1);
        s.remove(0);
        return;
    }

    // it is not on the edges, it must be the new point
    s.remove(2);
    s.remove(1);
    s.remove(0);

    // // it is on the edge
    // s.remove(2);
    // s.remove(0);
    // best_simplex(s);
    // todo!();
}

// all three faces have the origin above them
fn tetrahedron_three_sides_subcheck(s: &mut SimplexData) {
    todo!()
}

#[allow(clippy::similar_names)]
fn tetrahedron_triangle_subcheck(s: &mut SimplexData, xyd_perp: Vec3) {
    // eprintln!("one side");
    debug_assert!(s.len() == 3);
    // xd-re merőleges, kifelé mutat
    let xd_perp = xyd_perp.cross(&(s[2].diff - s[0].diff));
    // xd-n kívül van
    if xd_perp.dot(&-s[2].diff) > 0.0 {
        s.remove(1);
        return;
    }

    // yd-re merőleges, kifelé mutat
    let yd_perp = (s[2].diff - s[1].diff).cross(&xyd_perp);
    // yd-n kívül van
    if yd_perp.dot(&-s[2].diff) > 0.0 {
        s.remove(0);
    }
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
    best_simplex(s);
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
        // 4 => SupportPoint {
        //     diff: Vec3::zeros(),
        //     a: Vec3::zeros(),
        // },
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
            debug_assert!(
                m >= &&-TOLERANCE,
                "invalid multiplier m={m}, should be >= 0"
            );
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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_support_point(x: f64, y: f64, z: f64) -> SupportPoint {
        SupportPoint {
            diff: Vec3::new(x, y, z),
            a: Vec3::default(),
        }
    }

    #[test]
    fn one_simplex_best_simplex_is_self() {
        let mut s = SimplexData::new();
        s.push(test_support_point(1.0, 1.0, 1.0));
        let expected = s.clone();
        best_simplex(&mut s);
        assert_eq!(expected, s);
    }

    #[test]
    fn two_simplex_with_first_point_closest_best_simplex_is_first_point() {
        let mut s = SimplexData::new();
        s.push(test_support_point(1.0, 1.0, 1.0));
        let expected = s.clone();
        s.push(test_support_point(2.0, 2.0, 2.0));
        best_simplex(&mut s);
        assert_eq!(expected, s);
    }

    #[test]
    fn two_simplex_with_second_point_closest_best_simplex_is_second_point() {
        let mut s = SimplexData::new();
        s.push(test_support_point(1.0, 1.0, 1.0));
        let expected = s.clone();
        s.insert(0, test_support_point(2.0, 2.0, 2.0));
        best_simplex(&mut s);
        assert_eq!(expected, s);
    }

    #[test]
    fn two_simplex_with_neither_point_closest_best_simplex_is_self() {
        let mut s = SimplexData::new();
        s.push(test_support_point(1.0, 1.0, 1.0));
        s.insert(0, test_support_point(-1.0, -1.0, -1.0));
        let expected = s.clone();
        best_simplex(&mut s);
        assert_eq!(expected, s);
    }

    #[test]
    fn three_simplex_with_first_point_closest_best_simplex_is_first_point() {
        let mut s = SimplexData::new();
        s.push(test_support_point(1.0, 1.0, 1.0));
        let expected = s.clone();
        s.push(test_support_point(1.0, 3.0, 0.0));
        s.push(test_support_point(1.0, 0.0, 3.0));
        best_simplex(&mut s);
        assert_eq!(expected, s);
    }

    #[test]
    fn three_simplex_with_second_point_closest_best_simplex_is_second_point() {
        let mut s = SimplexData::new();
        s.push(test_support_point(1.0, 1.0, 1.0));
        let expected = s.clone();
        s.insert(0, test_support_point(1.0, 3.0, 0.0));
        s.push(test_support_point(1.0, 0.0, 3.0));
        best_simplex(&mut s);
        assert_eq!(expected, s);
    }

    #[test]
    fn three_simplex_with_third_point_closest_best_simplex_is_third_point() {
        let mut s = SimplexData::new();
        s.push(test_support_point(1.0, 1.0, 1.0));
        let expected = s.clone();
        s.insert(0, test_support_point(1.0, 3.0, 0.0));
        s.insert(0, test_support_point(1.0, 0.0, 3.0));
        best_simplex(&mut s);
        assert_eq!(expected, s);
    }

    #[test]
    fn three_simplex_with_first_and_second_point_closest_best_simplex_is_first_and_second_point(
    ) {
        let mut s = SimplexData::new();
        s.push(test_support_point(1.0, 0.5, 1.0));
        s.push(test_support_point(1.0, 3.0, -2.0));
        let expected = s.clone();
        s.push(test_support_point(1.0, 0.5, 3.0));
        best_simplex(&mut s);
        assert_eq!(expected, s);
    }

    #[test]
    fn three_simplex_with_second_and_third_point_closest_best_simplex_is_second_and_third_point(
    ) {
        let mut s = SimplexData::new();
        s.push(test_support_point(1.0, 0.5, 1.0));
        s.push(test_support_point(1.0, 3.0, -2.0));
        let expected = s.clone();
        s.insert(0, test_support_point(1.0, 0.5, 3.0));
        best_simplex(&mut s);
        assert_eq!(expected, s);
    }

    #[test]
    fn three_simplex_with_third_and_first_point_closest_best_simplex_is_third_and_first_point(
    ) {
        let mut s = SimplexData::new();
        s.push(test_support_point(1.0, 0.5, 1.0));
        s.insert(0, test_support_point(1.0, 3.0, -2.0));
        let expected = s.clone();
        s.insert(0, test_support_point(1.0, 0.5, 3.0));
        best_simplex(&mut s);
        assert_eq!(expected, s);
    }

    #[test]
    fn three_simplex_with_origin_above_best_simplex_is_self() {
        let mut s = SimplexData::new();
        s.push(test_support_point(-1.0, -1.0, 1.0));
        s.push(test_support_point(1.0, -1.0, 1.0));
        s.push(test_support_point(-0.5, -1.0, -1.0));
        let expected = s.clone();
        best_simplex(&mut s);
        assert_eq!(expected, s);
    }

    #[test]
    fn three_simplex_with_origin_below_best_simplex_is_self_reversed() {
        let mut s = SimplexData::new();
        s.push(test_support_point(-1.0, 1.0, 1.0));
        s.push(test_support_point(1.0, 1.0, 1.0));
        s.push(test_support_point(-0.5, 1.0, -1.0));
        let mut expected = s.clone();
        expected.reverse();
        best_simplex(&mut s);
        assert_eq!(expected, s);
    }

    #[test]
    fn four_simplex_origin_above_three_sides_cad() {
        let mut s = SimplexData::new();
        s.push(test_support_point(-1.0, -1.0, 1.0));
        s.push(test_support_point(1.0, -1.0, 1.0));
        s.push(test_support_point(-0.5, -1.0, -1.0));
        s.push(test_support_point(0.36704, -0.78627, 0.30012));
        let mut expected = SimplexData::new();
        expected.push(s[2]);
        expected.push(s[0]);
        expected.push(s[3]);
        best_simplex(&mut s);
        assert_eq!(expected, s);
    }

    #[test]
    fn four_simplex_origin_above_three_sides_cd() {
        let mut s = SimplexData::new();
        s.push(test_support_point(-1.0, -1.0, 1.0));
        s.push(test_support_point(1.0, -1.0, 1.0));
        s.push(test_support_point(-0.5, -1.0, -1.0));
        s.push(test_support_point(0.28652, -0.78627, 0.33763));
        let mut expected = SimplexData::new();
        expected.push(s[2]);
        expected.push(s[3]);
        best_simplex(&mut s);
        assert_eq!(expected, s);
    }

    #[test]
    fn four_simplex_origin_above_three_sides_db() {
        let mut s = SimplexData::new();
        s.push(test_support_point(-1.0, -1.0, 1.0));
        s.push(test_support_point(1.0, -1.0, 1.0));
        s.push(test_support_point(-0.5, -1.0, -1.0));
        s.push(test_support_point(-0.25293, -0.78627, -0.20727));
        let mut expected = SimplexData::new();
        expected.push(s[1]);
        expected.push(s[3]);
        best_simplex(&mut s);
        assert_eq!(expected, s);
    }

    #[test]
    fn four_simplex_origin_above_three_sides_abd() {
        let mut s = SimplexData::new();
        s.push(test_support_point(-1.0, -1.0, 1.0));
        s.push(test_support_point(1.0, -1.0, 1.0));
        s.push(test_support_point(-0.5, -1.0, -1.0));
        s.push(test_support_point(-0.26920, -0.53813, -0.78627));
        let mut expected = SimplexData::new();
        expected.push(s[0]);
        expected.push(s[1]);
        expected.push(s[3]);
        best_simplex(&mut s);
        assert_eq!(expected, s);
    }

    #[test]
    fn four_simplex_origin_above_three_sides_d() {
        let mut s = SimplexData::new();
        s.push(test_support_point(-1.0, -1.0, 1.0));
        s.push(test_support_point(1.0, -1.0, 1.0));
        s.push(test_support_point(-0.5, -1.0, -1.0));
        s.push(test_support_point(-0.057_238, -0.7, 0.135));
        let mut expected = SimplexData::new();
        expected.push(s[3]);
        best_simplex(&mut s);
        assert_eq!(expected, s);
    }

    #[test]
    fn four_simplex_origin_above_two_sides_real_case_1() {
        let mut s = SimplexData::new();
        s.push(test_support_point(
            0.157_176_221_303_004_35,
            -0.104_398_055_011_310_83,
            0.188_657_460_838_071_78,
        ));
        s.push(test_support_point(
            -1.947_285_518_635_449,
            -0.474_528_788_704_881_24,
            -0.411_726_347_281_799_6,
        ));
        s.push(test_support_point(
            -1.768_492_127_755_563_2,
            0.395_545_328_272_716_8,
            0.047_625_320_118_380_72,
        ));
        s.push(test_support_point(
            -0.795_245_122_271_985_1,
            0.165_784_004_328_146_22,
            0.047_606_976_564_171_79,
        ));
        let mut expected = SimplexData::new();
        expected.push(s[0]);
        expected.push(s[3]);
        best_simplex(&mut s);
        assert_eq!(expected, s);
    }

    #[test]
    fn four_simplex_origin_above_two_sides_real_case_2() {
        let mut s = SimplexData::new();
        s.push(test_support_point(
            -0.162_395_302_346_486_58,
            0.076_310_061_404_575_97,
            1.912_859_484_357_846_6,
        ));
        s.push(test_support_point(
            0.143_441_832_782_921,
            -0.092_231_565_004_763_37,
            -0.024_187_668_640_940_02,
        ));
        s.push(test_support_point(
            -0.807_911_975_194_552_9,
            -1.107_795_784_723_161_6,
            -0.331_894_813_685_246_2,
        ));
        s.push(test_support_point(
            -2.113_749_110_323_96,
            0.060_745_841_686_177_69,
            0.605_152_339_313_540_4,
        ));
        let mut expected = SimplexData::new();
        expected.push(s[1]);
        expected.push(s[3]);
        best_simplex(&mut s);
        assert_eq!(expected, s);
    }
}
