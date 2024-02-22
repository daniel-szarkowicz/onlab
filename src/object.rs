use std::rc::Rc;

use nalgebra::{Matrix4, Point3, Rotation3, Translation3};

use crate::{mesh::Mesh, vertex::PNVertex};

pub struct Object {
    pub mesh: Rc<Mesh<PNVertex>>,
    pub position: Point3<f32>,
    pub rotation: Rotation3<f32>,
}

impl Object {
    pub fn new(mesh: Rc<Mesh<PNVertex>>) -> Self {
        Self {
            mesh,
            position: Point3::new(0.0, 0.0, 0.0),
            rotation: Rotation3::identity(),
        }
    }

    pub fn model(&self) -> Matrix4<f32> {
        (Translation3::new(self.position.x, self.position.y, self.position.z)
            * self.rotation)
            .to_homogeneous()
    }
}
