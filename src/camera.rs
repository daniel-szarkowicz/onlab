use std::f32::consts::PI;

use nalgebra::{Matrix4, Perspective3, Point3, Rotation3, Vector3, Vector4};

pub struct FirstPersonCamera {
    position: Point3<f32>,
    pub yaw: f32,
    pub pitch: f32,
}

impl FirstPersonCamera {
    pub fn new() -> Self {
        Self {
            position: Point3::new(0.0, 0.0, 0.0),
            yaw: 0.0,
            pitch: 0.0,
        }
    }

    pub fn position(&self) -> Point3<f32> {
        self.position
    }

    fn yaw_rotation(&self) -> Rotation3<f32> {
        Rotation3::new(Vector3::y() * self.yaw / 180.0 * PI)
    }

    fn pitch_rotation(&self) -> Rotation3<f32> {
        Rotation3::new(Vector3::x() * self.pitch / 180.0 * PI)
    }

    fn look_at(&self) -> Point3<f32> {
        self.position
            + self.yaw_rotation() * self.pitch_rotation() * Vector3::z()
    }

    pub fn move_facing(&mut self, direction: Vector3<f32>) {
        self.position += self.yaw_rotation() * direction
    }

    pub fn view_proj(&self, aspect: f32) -> Matrix4<f32> {
        Perspective3::new(aspect, 60.0f32.to_radians(), 0.1, 1000.0)
            .to_homogeneous()
            * Matrix4::look_at_rh(
                &self.position,
                &self.look_at(),
                &Vector3::y(),
            )
    }
}
