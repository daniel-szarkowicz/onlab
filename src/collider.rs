use nalgebra::{
    Matrix3, Matrix4, Point3, Rotation3, Scale3, Translation3, Vector3, Vector4,
};

use crate::ray::Ray;

#[derive(Clone, Copy, Debug)]
pub enum Collider {
    Sphere(f32),
    Box(f32, f32, f32),
}

impl Collider {
    #[must_use]
    pub fn check_ray_hit(
        &self,
        position: Point3<f32>,
        rotation: Rotation3<f32>,
        ray: &Ray,
    ) -> Option<f32> {
        match self {
            Self::Sphere(radius) => {
                let a = ray.direction.dot(&ray.direction);
                let b = 2.0 * ray.direction.dot(&(ray.start - position));
                let c = 2.0f32.mul_add(
                    -ray.start.coords.dot(&position.coords),
                    ray.start.coords.dot(&ray.start.coords)
                        + position.coords.dot(&position.coords)
                        - radius * radius,
                );
                let discriminant = 4.0f32.mul_add(-a * c, b * b);
                if discriminant < 0.0 {
                    return None;
                }
                // t1 < t2;
                let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
                let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
                if t1 >= 0.0 {
                    Some(t1)
                } else if t2 >= 0.0 {
                    Some(t2)
                } else {
                    None
                }
            }
            Self::Box(w, h, d) => check_box_hit(
                (Translation3::from(position) * rotation).to_homogeneous()
                    * Scale3::new(*w, *h, *d).to_homogeneous(),
                ray,
            ),
        }
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn inverse_inertia(&self, mass: f32) -> Matrix3<f32> {
        match &self {
            Self::Sphere(r) => Matrix3::identity() * 2.0 / 3.0 * mass * (r * r),
            #[rustfmt::skip]
            Self::Box(w, h, d) => Matrix3::new(
                mass / 12.0 * (h * h + d * d), 0.0, 0.0,
                0.0, mass / 12.0 * (d * d + w * w), 0.0,
                0.0, 0.0, mass / 12.0 * (w * w + h * h)
            ),
        }
        .try_inverse()
        .expect("Inertia tensor should be invertible")
    }

    #[must_use]
    pub fn aabb(
        &self,
        position: &Point3<f32>,
        rotation: &Rotation3<f32>,
    ) -> (Point3<f32>, Point3<f32>) {
        match self {
            Self::Sphere(r) => (
                position + Vector3::new(-r, -r, -r),
                position + Vector3::new(*r, *r, *r),
            ),
            Self::Box(w, h, d) => {
                let mut min = *position;
                let mut max = *position;
                for x in [-0.5, 0.5] {
                    for y in [-0.5, 0.5] {
                        for z in [-0.5, 0.5] {
                            let p = position
                                + rotation * Vector3::new(x * w, y * h, z * d);
                            min = min.inf(&p);
                            max = max.sup(&p);
                        }
                    }
                }
                (min, max)
            }
        }
    }
}

fn check_box_hit(transform: Matrix4<f32>, ray: &Ray) -> Option<f32> {
    let mut best = None;
    for face in BOX_FACES {
        let face = face
            .map(|p| transform * Vector4::from(p))
            .map(|v| Point3::new(v.x, v.y, v.z) / v.w);
        let n = (face[1] - face[0]).cross(&(face[2] - face[0]));
        let t =
            (face[0] - ray.start).dot(&n) / ray.direction.normalize().dot(&n);
        let p = ray.start + t * ray.direction;
        if t >= 0.0
            && !best.is_some_and(|b| t >= b)
            && (face[1] - face[0]).cross(&(p - face[0])).dot(&n) > 0.0
            && (face[2] - face[1]).cross(&(p - face[1])).dot(&n) > 0.0
            && (face[3] - face[2]).cross(&(p - face[2])).dot(&n) > 0.0
            && (face[0] - face[3]).cross(&(p - face[3])).dot(&n) > 0.0
        {
            best = Some(t);
        }
    }
    best
}

#[rustfmt::skip]
const BOX_FACES: [[Point3<f32>; 4]; 6] = [
    [
        Point3::new( 0.5,  0.5,  0.5),
        Point3::new(-0.5,  0.5,  0.5),
        Point3::new(-0.5, -0.5,  0.5),
        Point3::new( 0.5, -0.5,  0.5),
    ],
    [
        Point3::new(-0.5,  0.5, -0.5),
        Point3::new( 0.5,  0.5, -0.5),
        Point3::new( 0.5, -0.5, -0.5),
        Point3::new(-0.5, -0.5, -0.5),
    ],
    [
        Point3::new( 0.5,  0.5,  0.5),
        Point3::new( 0.5, -0.5,  0.5),
        Point3::new( 0.5, -0.5, -0.5),
        Point3::new( 0.5,  0.5, -0.5),
    ],
    [
        Point3::new(-0.5, -0.5,  0.5),
        Point3::new(-0.5,  0.5,  0.5),
        Point3::new(-0.5,  0.5, -0.5),
        Point3::new(-0.5, -0.5, -0.5),
    ],
    [
        Point3::new( 0.5,  0.5,  0.5),
        Point3::new( 0.5,  0.5, -0.5),
        Point3::new(-0.5,  0.5, -0.5),
        Point3::new(-0.5,  0.5,  0.5),
    ],
    [
        Point3::new( 0.5, -0.5, -0.5),
        Point3::new( 0.5, -0.5,  0.5),
        Point3::new(-0.5, -0.5,  0.5),
        Point3::new(-0.5, -0.5, -0.5),
    ],
];
