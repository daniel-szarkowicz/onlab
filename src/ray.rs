use nalgebra::{Point3, Vector3};

use crate::camera::FirstPersonCamera;

pub struct Ray {
    pub start: Point3<f32>,
    pub direction: Vector3<f32>,
}

impl FirstPersonCamera {
    pub fn get_ray(&self) -> Ray {
        Ray {
            start: self.position(),
            direction: self.look_direction(),
        }
    }
}
