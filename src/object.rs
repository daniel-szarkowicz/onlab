use std::rc::Rc;

use nalgebra::{
    Matrix3, Matrix4, Point3, Rotation3, Scale3, Translation3, Vector3,
};

use crate::{aabb::AABB, collider::Collider, mesh::Mesh, vertex::PNVertex};

#[derive(Debug)]
pub struct Object {
    pub mesh: Rc<Mesh<PNVertex>>,
    // HACK collider should be private
    // because changing the collider changes the inverse inertia
    pub collider: Collider,
    pub position: Point3<f64>,
    pub rotation: Rotation3<f64>,
    pub immovable: bool,
    pub momentum: Vector3<f64>,
    pub angular_momentum: Vector3<f64>,
    // HACK mass should be private
    // because changing the mass changes the inverse inertia
    pub mass: f64,
    // HACK inverse_body inertia should be private
    // because it is inferred from collider and mass
    pub inverse_body_inertia: Matrix3<f64>,
    pub mesh_scale: Vector3<f32>,
    pub aabb: AABB,
}

impl Object {
    #[must_use]
    pub fn new(
        mesh: &Rc<Mesh<PNVertex>>,
        collider: Collider,
        mass: f64,
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
            aabb: AABB::new(Point3::default(), Point3::default()),
        }
    }

    #[must_use]
    pub fn model(&self) -> Matrix4<f32> {
        (Translation3::from(self.position.cast::<f32>())
            * self.rotation.cast::<f32>())
        .to_homogeneous()
            * Scale3::from(self.mesh_scale).to_homogeneous()
    }

    pub fn apply_impulse(
        &mut self,
        attack_point: Point3<f64>,
        impulse: Vector3<f64>,
    ) {
        if !self.immovable {
            self.momentum += impulse;
            self.angular_momentum +=
                (attack_point - self.position).cross(&impulse);
        }
    }

    pub fn update(&mut self, delta: f64) {
        self.position += self.momentum * delta / self.mass;
        self.rotation = Rotation3::new(
            self.inverse_inertia() * self.angular_momentum * delta,
        ) * self.rotation;
        self.aabb = self.collider.aabb(&self.position, &self.rotation);
    }

    #[must_use]
    pub const fn aabb(&self) -> &AABB {
        &self.aabb
    }

    #[must_use]
    pub fn inverse_inertia(&self) -> Matrix3<f64> {
        self.rotation * self.inverse_body_inertia * self.rotation.inverse()
    }

    #[must_use]
    pub fn local_velocity(&self, position: Point3<f64>) -> Vector3<f64> {
        self.momentum / self.mass
            + (self.inverse_inertia() * self.angular_momentum)
                .cross(&(position - self.position))
    }

    /// used for resolving collisions
    #[must_use]
    pub fn impulse_effectiveness(
        &self,
        attack_point: Point3<f64>,
        direction: Vector3<f64>,
    ) -> f64 {
        if self.immovable {
            return 0.0;
        }
        let attack_point_vector = attack_point - self.position;
        direction.dot(
            &(direction / self.mass
                + (self.inverse_inertia()
                    * attack_point_vector.cross(&direction))
                .cross(&attack_point_vector)),
        )
    }
}
