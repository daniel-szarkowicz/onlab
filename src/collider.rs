use nalgebra::Point3;

use crate::ray::Ray;

pub enum Collider {
    Sphere(f32),
    Box(f32, f32, f32),
}

impl Collider {
    pub fn check_ray_hit(
        &self,
        position: Point3<f32>,
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
            Box(..) => None,
        }
    }
}
