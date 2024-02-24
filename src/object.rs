use std::rc::Rc;

use nalgebra::{Matrix4, Point3, Rotation3, Scale3, Translation3, Vector3};

use crate::{collider::Collider, mesh::Mesh, vertex::PNVertex};

pub struct Object {
    pub mesh: Rc<Mesh<PNVertex>>,
    pub collider: Collider,
    pub position: Point3<f32>,
    pub rotation: Rotation3<f32>,
    pub immovable: bool,
    pub momentum: Vector3<f32>,
    pub mass: f32,
    pub mesh_scale: Vector3<f32>,
}

impl Object {
    pub fn new(mesh: &Rc<Mesh<PNVertex>>, collider: Collider) -> Self {
        Self {
            mesh: mesh.clone(),
            collider,
            position: Point3::new(0.0, 0.0, 0.0),
            rotation: Rotation3::identity(),
            immovable: false,
            momentum: Vector3::zeros(),
            mass: 1.0,
            mesh_scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }

    pub fn model(&self) -> Matrix4<f32> {
        (Translation3::from(self.position) * self.rotation).to_homogeneous()
            * Scale3::from(self.mesh_scale).to_homogeneous()
    }

    pub fn apply_impulse(
        &mut self,
        attack_point: Point3<f32>,
        impulse: Vector3<f32>,
    ) {
        if !self.immovable {
            self.momentum += impulse;
        }
    }

    pub fn update(&mut self, delta: f32) {
        if !self.immovable {
            self.position += self.momentum * delta / self.mass;
        }
    }
}
