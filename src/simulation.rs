// simulation steps:
//   1. calculate aabbs
//   2. put aabb start/end + object reference pairs in lists
//   3. order lists by aabb start/end
//   4. filter for possible collisions
//   5. merge possible collisions from the three axis
//   6. check possible collisions for real collisions
//   7. resolve collisions one-by-one
//   8.

use std::f64;

use nalgebra::{Point3, Vector3};

use crate::{collider::Collider, object::Object};

// simulation should cache aabb ordering for a considerable speedup
#[derive(Debug)]
pub struct Simulation {
    pub epsilon: f64,
    pub mu: f64,
}

impl Simulation {
    pub fn simulate(&mut self, objects: &mut [Object], delta: f64) {
        for i in 0..objects.len() {
            for j in (i + 1)..objects.len() {
                let (a, b) = objects.split_at_mut(j);
                let o1 = &mut a[i];
                let o2 = &mut b[0];
                let contact = self.check_contact(o1, o2);
                match contact {
                    Contact::Colliding {
                        contact_point_1,
                        contact_point_2,
                        relative_velocity,
                        contact_normal,
                    } => {
                        let normal_ka = o1.impulse_effectiveness(
                            contact_point_1,
                            contact_normal,
                        );
                        let normal_kb = o2.impulse_effectiveness(
                            contact_point_2,
                            contact_normal,
                        );

                        let normal_impulse_strength = -(self.epsilon + 1.0)
                            * contact_normal.dot(&relative_velocity)
                            / (normal_ka + normal_kb);

                        let nonnormal_relative_velocity = relative_velocity
                            - contact_normal
                                * contact_normal.dot(&relative_velocity);
                        let nonnormal_relative_velocity_direction =
                            -nonnormal_relative_velocity
                                .try_normalize(f64::EPSILON)
                                .unwrap_or_else(Vector3::x);

                        let friction_ka = o1.impulse_effectiveness(
                            contact_point_1,
                            nonnormal_relative_velocity_direction,
                        );
                        let friction_kb = o1.impulse_effectiveness(
                            contact_point_1,
                            nonnormal_relative_velocity_direction,
                        );

                        let friction_impulse_max_strength =
                            -nonnormal_relative_velocity_direction
                                .dot(&relative_velocity)
                                / (friction_ka + friction_kb);
                        let friction_impulse_strength =
                            friction_impulse_max_strength
                                .min(self.mu * normal_impulse_strength);

                        let impulse = contact_normal * normal_impulse_strength
                            + nonnormal_relative_velocity_direction
                                * friction_impulse_strength;
                        o1.apply_impulse(contact_point_1, impulse);
                        o2.apply_impulse(contact_point_2, -impulse);
                    }
                    Contact::Resting {} => { /* TODO */ }
                    Contact::None => (),
                }
            }
        }
        for obj in objects {
            // obj.apply_impulse(
            //     obj.position,
            //     Vector3::new(0.0, -10.0 * delta * obj.mass, 0.0),
            // );
            obj.update(delta);
        }
    }

    #[allow(clippy::unused_self)]
    fn check_contact(&mut self, o1: &Object, o2: &Object) -> Contact {
        match (o1.collider, o2.collider) {
            (Collider::Sphere(r1), Collider::Sphere(r2)) => {
                let center_distance = o1.position - o2.position;
                if center_distance.magnitude() < r1 + r2 {
                    let contact_normal = center_distance.normalize();
                    let contact_point_1 = o1.position - contact_normal * r1;
                    let contact_point_2 = o2.position + contact_normal * r2;
                    let contact_velocity_1 = o1.local_velocity(contact_point_1);
                    let contact_velocity_2 = o2.local_velocity(contact_point_2);
                    let relative_velocity =
                        contact_velocity_1 - contact_velocity_2;
                    let normal_velocity =
                        contact_normal.dot(&relative_velocity);
                    if normal_velocity > f64::EPSILON {
                        Contact::None
                    } else if normal_velocity < -f64::EPSILON {
                        Contact::Colliding {
                            contact_point_1,
                            contact_point_2,
                            relative_velocity,
                            contact_normal,
                        }
                    } else {
                        Contact::Resting {}
                    }
                } else {
                    Contact::None
                }
            }
            (Collider::Sphere(r), Collider::Box(w, h, d)) => {
                let half_size = Vector3::new(w, h, d) / 2.0;
                let box_space_position =
                    o2.rotation.inverse() * (o1.position - o2.position);
                let component_wise_distance =
                    box_space_position.abs() - half_size;
                let box_space_normal = component_wise_distance
                    .zip_map(&box_space_position, |c, p| {
                        c.max(0.0) * p.signum()
                    });
                if box_space_normal.magnitude() > r {
                    return Contact::None;
                }
                let box_space_closest_offset = box_space_position
                    .zip_map(&half_size, |p, s| p.clamp(-s, s));
                let world_space_normal = o2.rotation * box_space_normal;
                let world_space_closest_offset =
                    o2.rotation * box_space_closest_offset;
                let contact_point_1 = o1.position - world_space_normal;
                let contact_point_2 = o2.position + world_space_closest_offset;
                let contact_normal = world_space_normal.normalize();
                let contact_velocity_1 = o1.local_velocity(contact_point_1);
                let contact_velocity_2 = o2.local_velocity(contact_point_2);
                let relative_velocity = contact_velocity_1 - contact_velocity_2;
                let normal_velocity = contact_normal.dot(&relative_velocity);
                if normal_velocity > f64::EPSILON {
                    Contact::None
                } else if normal_velocity < -f64::EPSILON {
                    Contact::Colliding {
                        contact_point_1,
                        contact_point_2,
                        relative_velocity,
                        contact_normal,
                    }
                } else {
                    Contact::Resting {}
                }
            }
            _ => Contact::None,
        }
    }
}

#[derive(Debug)]
pub enum Contact {
    None,
    Resting {/* TODO: fields */},
    Colliding {
        contact_point_1: Point3<f64>,
        contact_point_2: Point3<f64>,
        relative_velocity: Vector3<f64>,
        contact_normal: Vector3<f64>,
    },
}
