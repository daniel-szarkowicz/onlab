use std::rc::Rc;

use nalgebra::{
    Matrix3, Matrix4, Point3, Rotation3, Scale3, Translation3, Vector3,
};

use crate::{collider::Collider, mesh::Mesh, vertex::PNVertex};

#[derive(Debug)]
pub struct Object {
    pub mesh: Rc<Mesh<PNVertex>>,
    // HACK collider should be private
    // because changing the collider changes the inverse inertia
    pub collider: Collider,
    pub position: Point3<f32>,
    pub rotation: Rotation3<f32>,
    pub immovable: bool,
    pub momentum: Vector3<f32>,
    pub angular_momentum: Vector3<f32>,
    // HACK mass should be private
    // because changing the mass changes the inverse inertia
    pub mass: f32,
    // HACK inverse_body inertia should be private
    // because it is inferred from collider and mass
    pub inverse_body_inertia: Matrix3<f32>,
    pub mesh_scale: Vector3<f32>,
}

impl Object {
    #[must_use]
    pub fn new(
        mesh: &Rc<Mesh<PNVertex>>,
        collider: Collider,
        mass: f32,
    ) -> Self {
        Self {
            mesh: mesh.clone(),
            position: Point3::new(0.0, 0.0, 0.0),
            rotation: Rotation3::identity(),
            immovable: false,
            momentum: Vector3::zeros(),
            angular_momentum: Vector3::new(0.0, 0.0, 0.0),
            mass,
            inverse_body_inertia: collider.inverse_inertia(mass),
            collider,
            mesh_scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }

    #[must_use]
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
            self.angular_momentum +=
                (attack_point - self.position).cross(&impulse);
        }
    }

    pub fn update(&mut self, delta: f32) {
        if !self.immovable {
            self.position += self.momentum * delta / self.mass;
            self.rotation = Rotation3::new(
                self.inverse_inertia() * self.angular_momentum * delta
                    / self.mass,
            ) * self.rotation;
        }
    }

    #[must_use]
    pub fn aabb(&self) -> (Point3<f32>, Point3<f32>) {
        self.collider.aabb(&self.position, &self.rotation)
    }

    #[must_use]
    pub fn inverse_inertia(&self) -> Matrix3<f32> {
        self.rotation * self.inverse_body_inertia * self.rotation.inverse()
    }

    #[must_use]
    pub fn local_velocity(&self, position: Point3<f32>) -> Vector3<f32> {
        self.momentum / self.mass
            + (self.inverse_inertia() * self.angular_momentum)
                .cross(&(position - self.position))
    }
}
