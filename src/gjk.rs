use std::ops::{Add, Mul, Sub};

use nalgebra::{Const, DimMin, Matrix, Vector3};
use rand::random;
use smallvec::SmallVec;

type SimplexData = SmallVec<[SupportPoint; 5]>;
// type SimplexData = Vec<SupportPoint>;

type Vec3 = Vector3<f64>;

const TOLERANCE: f64 = 1e-7;
const SIMPLEX_MAX_DIM: usize = 4;
const EPA_MAX_ITER: usize = 1000;

pub trait Support {
    fn support(&self, direction: &Vec3) -> Vec3;
    fn radius(&self) -> f64;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SupportPoint {
    pub diff: Vec3,
    pub a: Vec3,
    pub b: Vec3,
}

impl SupportPoint {
    pub fn new(a: &impl Support, b: &impl Support, dir: &Vec3) -> Self {
        let a = a.support(dir);
        let b = b.support(&-dir);
        Self { diff: a - b, a, b }
    }
}

pub fn gjk(a: &impl Support, b: &impl Support) -> GJKResult {
    let mut s = SimplexData::with_capacity(5);
    s.push(SupportPoint::new(
        a,
        b,
        &Vec3::new(random(), random(), random()),
    ));
    let mut prev_dist = f64::INFINITY;
    loop {
        let closest_point;
        (closest_point, s) = closest_simplex::<false>(s);
        let dist = closest_point.diff.magnitude();
        // debug_assert!(
        //     dist <= prev_dist + TOLERANCE,
        //     "prev_dist={prev_dist}, dist={dist}"
        // );
        if s.len() == SIMPLEX_MAX_DIM {
            // return GJKResult::UnknownContact(s);
            return epa(a, b, s.into_vec());
        }
        // debug_assert!(
        //     dist > TOLERANCE,
        //     "if dist={dist} is smaller than TOLERANCE={TOLERANCE} \
        //      the simplex should have {SIMPLEX_MAX_DIM} points"
        // );
        if prev_dist - dist <= TOLERANCE {
            if dist <= a.radius() + b.radius() {
                let normal = closest_point.a - closest_point.b;
                return GJKResult::Contact {
                    points: (
                        closest_point.a
                            - normal * a.radius() / (a.radius() + b.radius()),
                        closest_point.b
                            + normal * b.radius() / (a.radius() + b.radius()),
                    ),
                    normal: normal.normalize(),
                };
            }
            return GJKResult::NoContact;
        }
        prev_dist = dist;
        let new_point = SupportPoint::new(a, b, &-closest_point.diff);
        if closest_point
            .diff
            .dot(&(new_point.diff - closest_point.diff))
            >= -TOLERANCE
        {
            // eprintln!("not further");
            if dist <= a.radius() + b.radius() {
                let normal = closest_point.a - closest_point.b;
                return GJKResult::Contact {
                    points: (
                        closest_point.a
                            - normal * a.radius() / (a.radius() + b.radius()),
                        closest_point.b
                            + normal * b.radius() / (a.radius() + b.radius()),
                    ),
                    normal: normal.normalize(),
                };
            }
            return GJKResult::NoContact;
        }
        s.push(new_point);
    }
}

fn closest_simplex<const DETCHECK: bool>(
    s: SimplexData,
) -> (SupportPoint, SimplexData) {
    match s.len() {
        0 => panic!("simplex has to contain at least 1 point"),
        1 => (s[0].clone(), s),
        2 if !DETCHECK => {
            let t = (-s[1].diff.dot(&(s[0].diff - s[1].diff))
                / (s[0].diff - s[1].diff).magnitude_squared())
            .clamp(0.0, 1.0);
            (s[1].clone() + t * &(s[0].clone() - s[1].clone()), s)
        }
        2 => closest_simplex_static::<2, DETCHECK>(s),
        3 => closest_simplex_static::<3, DETCHECK>(s),
        4 => closest_simplex_static::<4, DETCHECK>(s),
        _ => unreachable!(),
    }
}

fn closest_simplex_static<const N: usize, const DETCHECK: bool>(
    mut s: SimplexData,
) -> (SupportPoint, SimplexData)
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
    let mut mi = multipliers
        .iter()
        .enumerate()
        .filter(|(_, d)| d < &&0.0)
        .map(|(i, _)| i);
    if let Some(bad_i) = mi.next() {
        let find_best =
            |mut s: SimplexData, i, best: Option<(SupportPoint, _)>| {
                s.swap_remove(i);
                let (point, s) = closest_simplex::<DETCHECK>(s);
                if let Some((bp, bs)) = best {
                    if point.diff.magnitude() < bp.diff.magnitude() {
                        (point, s)
                    } else {
                        (bp, bs)
                    }
                } else {
                    (point, s)
                }
            };
        let best = mi.fold(None, |best, i| Some(find_best(s.clone(), i, best)));
        find_best(s, bad_i, best)
    } else {
        (
            multipliers
                .iter()
                .zip(&s)
                .map(|(t, v)| *t * v)
                .reduce(|a, b| a + b)
                .unwrap(),
            s,
        )
    }
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
        let closest_point;
        (closest_point, tmp) = closest_simplex::<true>(tmp);
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
        let minface = closest_points
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                a.diff.magnitude().total_cmp(&b.diff.magnitude())
            })
            .map(|(i, _)| i)
            .unwrap();
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
            return GJKResult::Contact {
                points: (closest_points[minface].a, closest_points[minface].b),
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
            let closest_point;
            (closest_point, tmp) = closest_simplex::<true>(tmp);
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
            b: self * rhs.b,
        }
    }
}

impl Add for SupportPoint {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.diff += rhs.diff;
        self.a += rhs.a;
        self.b += rhs.b;
        self
    }
}

impl Sub for SupportPoint {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.diff -= rhs.diff;
        self.a -= rhs.a;
        self.b -= rhs.b;
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
