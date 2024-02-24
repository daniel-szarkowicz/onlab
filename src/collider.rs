use nalgebra::{
    Matrix4, Point3, Rotation3, Scale3, Translation3, Vector3, Vector4,
};

use crate::ray::Ray;

pub enum Collider {
    Sphere(f32),
    Box(f32, f32, f32),
}

impl Collider {
    pub fn check_ray_hit(
        &self,
        position: Point3<f32>,
        rotation: Rotation3<f32>,
        ray: &Ray,
    ) -> Option<f32> {
        use Collider::*;
        match self {
            Sphere(r) => {
                let a = ray.direction.dot(&ray.direction);
                let b = 2.0 * ray.direction.dot(&(ray.start - position));
                let c = ray.start.coords.dot(&ray.start.coords)
                    + position.coords.dot(&position.coords)
                    - 2.0 * ray.start.coords.dot(&position.coords)
                    - r * r;
                let d = b * b - 4.0 * a * c;
                if d < 0.0 {
                    return None;
                }
                // t1 < t2;
                let t1 = (-b - d.sqrt()) / a;
                let t2 = (-b + d.sqrt()) / a;
                if t1 >= 0.0 {
                    Some(t1)
                } else {
                    Some(t2)
                }
            }
            Box(w, h, d) => check_box_hit(
                (Translation3::from(position) * rotation).to_homogeneous()
                    * Scale3::new(*w, *h, *d).to_homogeneous(),
                ray,
            ),
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
