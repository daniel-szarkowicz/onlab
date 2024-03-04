// simulation steps:
//   1. calculate aabbs
//   2. put aabb start/end + object reference pairs in lists
//   3. order lists by aabb start/end
//   4. filter for possible collisions
//   5. merge possible collisions from the three axis
//   6. check possible collisions for real collisions
//   7. resolve collisions one-by-one
//   8.

use std::{collections::HashSet, vec::Vec};

use nalgebra::{Point3, Vector3};

use crate::{collider::Collider, object::Object};

// simulation should cache aabb ordering for a considerable speedup
#[derive(Debug)]
pub struct Simulation {
    pub epsilon: f64,
    pub mu: f64,
    pub pc_scratchbuf: Vec<u8>,
}

impl Simulation {
    pub fn simulate(&mut self, objects: &mut [Object], delta: f64) {
        let contacts = self.check_contacts(objects);
        for (i, j, contact) in &*contacts {
            assert!(i < j);
            let (s1, s2) = objects.split_at_mut(*j);
            self.resolve_contact(&mut s1[*i], &mut s2[0], contact);
        }
        for obj in objects {
            // obj.apply_impulse(
            //     obj.position,
            //     Vector3::new(0.0, -10.0 * delta * obj.mass, 0.0),
            // );
            obj.update(delta);
        }
    }

    #[allow(clippy::too_many_lines)]
    fn check_contacts(
        &mut self,
        objects: &[Object],
    ) -> Box<[(usize, usize, Contact)]> {
        #[derive(PartialEq, Eq, Clone)]
        enum Interval {
            Start,
            End,
        }

        let objects_iter = objects.iter().enumerate();
        let mut intervals: Vec<_> = objects_iter
            .clone()
            .flat_map(|(i, o)| {
                let aabb = o.aabb();
                [
                    (i, aabb.start(), Interval::Start),
                    (i, aabb.end(), Interval::End),
                ]
            })
            .collect();

        let mut open_intervals = HashSet::with_capacity(objects.len() / 10);
        let mut x_potential_contacts =
            Vec::with_capacity(objects.len() * objects.len() / 10);
        self.pc_scratchbuf.resize(objects.len() * objects.len(), 0);
        let scratchbuf = &mut self.pc_scratchbuf[..];
        intervals.sort_unstable_by(|(_, a, _), (_, b, _)| a.x.total_cmp(&b.x));
        for (i, _, interval) in &intervals {
            match interval {
                Interval::Start => {
                    for j in &open_intervals {
                        let mini = *i.min(j);
                        let maxi = *i.max(j);
                        scratchbuf[mini * objects.len() + maxi] = 1;
                        x_potential_contacts.push((mini, maxi));
                    }
                    open_intervals.insert(*i);
                }
                Interval::End => {
                    open_intervals.remove(i);
                }
            }
        }

        let mut pc_upper_bound = 0;
        intervals.sort_unstable_by(|(_, a, _), (_, b, _)| a.y.total_cmp(&b.y));
        for (i, _, interval) in &intervals {
            match interval {
                Interval::Start => {
                    for j in &open_intervals {
                        let mini = *i.min(j);
                        let maxi = *i.max(j);
                        scratchbuf[mini * objects.len() + maxi] <<= 1;
                        pc_upper_bound += 1;
                    }
                    open_intervals.insert(*i);
                }
                Interval::End => {
                    open_intervals.remove(i);
                }
            }
        }

        let mut true_potential_contacts = Vec::with_capacity(pc_upper_bound);

        intervals.sort_unstable_by(|(_, a, _), (_, b, _)| a.z.total_cmp(&b.z));
        for (i, _, interval) in &intervals {
            match interval {
                Interval::Start => {
                    for j in &open_intervals {
                        let mini = *i.min(j);
                        let maxi = *i.max(j);
                        if scratchbuf[mini * objects.len() + maxi] == 2 {
                            true_potential_contacts.push((mini, maxi));
                        }
                    }
                    open_intervals.insert(*i);
                }
                Interval::End => {
                    open_intervals.remove(i);
                }
            }
        }

        let contacts = true_potential_contacts
            .into_iter()
            .filter_map(|(i, j)| {
                let o1 = &objects[i];
                let o2 = &objects[j];
                Self::check_contact(o1, o2).map(|contact| (i, j, contact))
            })
            .collect();
        for (i, j) in x_potential_contacts {
            self.pc_scratchbuf[i * objects.len() + j] = 0;
        }
        contacts
    }

    fn resolve_contact(
        &self,
        o1: &mut Object,
        o2: &mut Object,
        contact: &Contact,
    ) {
        let relative_velocity = o1.local_velocity(contact.points.0)
            - o2.local_velocity(contact.points.1);
        let normal_velocity = relative_velocity.dot(&contact.normal);
        #[allow(clippy::if_same_then_else)]
        if normal_velocity < -f64::EPSILON {
            self.resolve_colliding_contact(o1, o2, contact);
        } else if normal_velocity < f64::EPSILON {
            // TODO: handle resting contact separately
            self.resolve_colliding_contact(o1, o2, contact);
        } else {
            // separating contact
        }
    }

    fn check_contact(o1: &Object, o2: &Object) -> Option<Contact> {
        match (o1.collider, o2.collider) {
            (Collider::Sphere(r1), Collider::Sphere(r2)) => {
                let center_distance = o1.position - o2.position;
                if center_distance.magnitude() <= r1 + r2 {
                    let contact_normal = center_distance.normalize();
                    Some(Contact {
                        points: (
                            o1.position - contact_normal * r1,
                            o2.position + contact_normal * r2,
                        ),
                        normal: contact_normal,
                    })
                } else {
                    None
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
                    return None;
                }
                let box_space_closest_offset = box_space_position
                    .zip_map(&half_size, |p, s| p.clamp(-s, s));
                let world_space_normal = o2.rotation * box_space_normal;
                let world_space_closest_offset =
                    o2.rotation * box_space_closest_offset;
                Some(Contact {
                    points: (
                        o1.position - world_space_normal,
                        o2.position + world_space_closest_offset,
                    ),
                    normal: world_space_normal.normalize(),
                })
            }
            _ => None,
        }
    }

    fn resolve_colliding_contact(
        &self,
        o1: &mut Object,
        o2: &mut Object,
        contact: &Contact,
    ) {
        let relative_velocity = o1.local_velocity(contact.points.0)
            - o2.local_velocity(contact.points.1);
        let normal_impulse_strength = -(self.epsilon + 1.0)
            * contact.normal.dot(&relative_velocity)
            / (o1.impulse_effectiveness(contact.points.0, contact.normal)
                + o2.impulse_effectiveness(contact.points.1, contact.normal));

        let nonnormal_relative_velocity = relative_velocity
            - contact.normal * contact.normal.dot(&relative_velocity);
        let nonnormal_relative_velocity_direction =
            -nonnormal_relative_velocity
                .try_normalize(f64::EPSILON)
                .unwrap_or_else(Vector3::x);

        let friction_impulse_max_strength =
            -nonnormal_relative_velocity_direction.dot(&relative_velocity)
                / (o1.impulse_effectiveness(
                    contact.points.0,
                    nonnormal_relative_velocity_direction,
                ) + o2.impulse_effectiveness(
                    contact.points.1,
                    nonnormal_relative_velocity_direction,
                ));
        let friction_impulse_strength = friction_impulse_max_strength
            .min(self.mu * normal_impulse_strength);

        let impulse = contact.normal * normal_impulse_strength
            + nonnormal_relative_velocity_direction * friction_impulse_strength;
        o1.apply_impulse(contact.points.0, impulse);
        o2.apply_impulse(contact.points.1, -impulse);
    }
}

struct Contact {
    points: (Point3<f64>, Point3<f64>),
    normal: Vector3<f64>,
}
