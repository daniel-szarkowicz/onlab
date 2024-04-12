use std::ops::{Add, Mul};

use nalgebra::{Const, Dyn, Matrix, Vector3};
use rand::random;

type Vec3 = Vector3<f64>;

const TOLERANCE: f64 = 1e-7;
const SIMPLEX_MAX_DIM: usize = 4;

pub trait Support {
    fn support(&self, direction: &Vec3) -> Vec3;
    fn radius(&self) -> f64;
}

#[derive(Debug, Clone, PartialEq)]
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
    let mut s = Vec::with_capacity(5);
    s.push(SupportPoint::new(
        a,
        b,
        &Vec3::new(random(), random(), random()),
    ));
    let mut prev_dist = f64::INFINITY;
    loop {
        let closest_point;
        (closest_point, s) = closest_simplex(s);
        let dist = closest_point.diff.magnitude();
        debug_assert!(
            dist <= prev_dist + TOLERANCE,
            "prev_dist={prev_dist}, dist={dist}"
        );
        if s.len() == SIMPLEX_MAX_DIM {
            return GJKResult::UnknownContact(s);
        }
        debug_assert!(
            dist > TOLERANCE,
            "if dist={dist} is smaller than TOLERANCE={TOLERANCE} \
             the simplex should have {SIMPLEX_MAX_DIM} points"
        );
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
        if !s.contains(&new_point) {
            s.push(new_point);
        }
    }
}

fn closest_simplex(s: Vec<SupportPoint>) -> (SupportPoint, Vec<SupportPoint>) {
    match s.len() {
        0 => panic!("simplex has to contain at least 1 point"),
        // 1 => (s[0].clone(), s),
        len => {
            let diffs: Vec<_> =
                s.iter().skip(1).map(|p| p.diff - s[0].diff).collect();
            let mut a_data = Vec::with_capacity(len * len);
            a_data.push(1.0);
            #[allow(clippy::same_item_push)]
            for _ in 0..len - 1 {
                a_data.push(0.0);
            }
            for v1 in &diffs {
                a_data.push(1.0);
                for v2 in &diffs {
                    a_data.push(v1.dot(v2));
                }
            }
            let a = Matrix::from_vec_generic(Dyn(len), Dyn(len), a_data);
            let det = a.determinant();
            if det.abs() <= TOLERANCE {
                let mut best: Option<(SupportPoint, Vec<SupportPoint>)> = None;
                // TODO: looping over all possible sub-simplices is redundant
                // need to figure out a way to reduce these iterations
                for i in 0..len {
                    let mut new_s = s.clone();
                    new_s.swap_remove(i);
                    let (point, new_s) = closest_simplex(new_s);
                    if best.is_none()
                        || point.diff.magnitude()
                            < best.as_ref().unwrap().0.diff.magnitude()
                    {
                        best = Some((point, new_s));
                    }
                }
                return best.unwrap();
            }
            let a_inverse = a.try_inverse().expect("a is invertible");
            let mut b_data = Vec::with_capacity(len);
            b_data.push(1.0);
            for v in &diffs {
                b_data.push(-s[0].diff.dot(v));
            }
            let b = Matrix::from_vec_generic(Dyn(len), Const::<1>, b_data);
            let multipliers = a_inverse * b;
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
                let mut best: Option<(SupportPoint, Vec<SupportPoint>)> = None;
                for i in multipliers
                    .iter()
                    .enumerate()
                    .filter(|(_, d)| d < &&0.0)
                    .map(|(i, _)| i)
                    .rev()
                {
                    let mut new_s = s.clone();
                    new_s.swap_remove(i);
                    let (point, new_s) = closest_simplex(new_s);
                    if best.is_none()
                        || point.diff.magnitude()
                            < best.as_ref().unwrap().0.diff.magnitude()
                    {
                        best = Some((point, new_s));
                    }
                }
                best.unwrap()
            }
        }
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
