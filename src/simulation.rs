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

use nalgebra::{Matrix3, Point3, Scale3, Translation3, Vector3, Vector4};

use crate::{
    aabb::AABB,
    collider::Collider,
    gjk::{gjk, GJKResult},
    object::Object,
    rtree::RTree,
};

#[derive(Debug)]
pub struct Simulation {
    pub epsilon: f64,
    pub mu: f64,
    pub rtree: RTree<usize>,
}

impl Default for Simulation {
    fn default() -> Self {
        let mut rtree = RTree::new();
        for i in 0..100 {
            let x =
                rand::random::<f64>().mul_add(10.0, f64::from(i % 2) * 20.0);
            let y = rand::random::<f64>()
                .mul_add(10.0, f64::from(i / 2 % 2) * 20.0);
            let z = rand::random::<f64>()
                .mul_add(10.0, f64::from(i / 4 % 2) * 20.0);
            rtree.insert(
                AABB::new(
                    Point3::new(x, y, z),
                    Point3::new(x + 1.0, y + 1.0, z + 1.0),
                ),
                0,
            );
        }
        Self {
            epsilon: 1.0,
            mu: 1.0,
            rtree,
        }
    }
}

impl Simulation {
    pub fn simulate(&mut self, objects: &mut [Object], delta: f64) {
        for obj in objects.iter_mut() {
            // obj.apply_impulse(
            //     obj.position,
            //     Vector3::new(0.0, -10.0 * delta * obj.mass, 0.0),
            // );
            obj.update(delta);
        }
        let rtree_contacts = self.check_contacts_rtree(objects);
        // let axis_contacts = self.check_contacts_1axis(objects);
        // println!(
        //     "rtree: {}, 1axis: {}",
        //     rtree_contacts.len(),
        //     axis_contacts.len()
        // );
        let contacts = rtree_contacts;
        for (i, j, contact) in &*contacts {
            assert!(i < j);
            let (s1, s2) = objects.split_at_mut(*j);
            self.resolve_contact(&mut s1[*i], &mut s2[0], contact);
        }
    }

    fn check_contacts_rtree(
        &mut self,
        objects: &[Object],
    ) -> Box<[(usize, usize, Contact)]> {
        self.rtree.clear();
        for (i, obj) in objects.iter().enumerate() {
            self.rtree.insert(obj.aabb().clone(), i);
        }
        let mut contacts = Vec::with_capacity(objects.len());
        for (i, obj) in objects.iter().enumerate() {
            for &j in self.rtree.search(obj.aabb()) {
                // i > j was already checked
                // i == j should not be checked
                if i < j {
                    if let Some(contact) =
                        self.check_contact(&objects[i], &objects[j])
                    {
                        contacts.push((i, j, contact));
                    }
                }
            }
        }
        contacts.into()
    }

    #[allow(dead_code)]
    fn check_contacts_1axis(
        &mut self,
        objects: &[Object],
    ) -> Box<[(usize, usize, Contact)]> {
        #[derive(PartialEq, Eq, Clone)]
        enum Interval {
            Start,
            End,
        }

        let mut intervals: Vec<_> = objects
            .iter()
            .enumerate()
            .flat_map(|(i, o)| {
                let aabb = o.aabb();
                [
                    (i, aabb.start(), Interval::Start),
                    (i, aabb.end(), Interval::End),
                ]
            })
            .collect();

        intervals.sort_unstable_by(|(_, a, _), (_, b, _)| a.x.total_cmp(&b.x));

        let mut open_intervals =
            HashSet::<usize>::with_capacity(intervals.len() / 20);
        let mut potential_contacts = Vec::with_capacity(objects.len() * 2);
        for (i, _, interval) in intervals {
            match interval {
                Interval::Start => {
                    for &j in &open_intervals {
                        if objects[i].aabb().overlaps_yz(objects[j].aabb()) {
                            potential_contacts.push((i.min(j), i.max(j)));
                        }
                    }
                    open_intervals.insert(i);
                }
                Interval::End => {
                    open_intervals.remove(&i);
                }
            }
        }

        potential_contacts
            .iter()
            .filter_map(|&(i, j)| {
                self.check_contact(&objects[i], &objects[j])
                    .map(|contact| (i, j, contact))
            })
            .collect()
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

    #[allow(clippy::unused_self)]
    #[allow(clippy::too_many_lines)]
    fn check_contact(&self, o1: &Object, o2: &Object) -> Option<Contact> {
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
            (Collider::Box(..), Collider::Sphere(..)) => {
                self.check_contact(o2, o1).map(|c| Contact {
                    points: (c.points.1, c.points.0),
                    normal: -c.normal,
                })
            }
            (Collider::Box(w1, h1, d1), Collider::Box(w2, h2, d2)) => {
                let t1 = Translation3::from(o1.position).to_homogeneous()
                    * o1.rotation.to_homogeneous()
                    * Scale3::new(w1, h1, d1).to_homogeneous();
                let t1_inv = t1.try_inverse().unwrap();
                let t2 = Translation3::from(o2.position).to_homogeneous()
                    * o2.rotation.to_homogeneous()
                    * Scale3::new(w2, h2, d2).to_homogeneous();
                let t2_inv = t2.try_inverse().unwrap();
                let points = [
                    Vector4::new(0.5, 0.5, 0.5, 1.0),
                    Vector4::new(0.5, 0.5, -0.5, 1.0),
                    Vector4::new(0.5, -0.5, 0.5, 1.0),
                    Vector4::new(0.5, -0.5, -0.5, 1.0),
                    Vector4::new(-0.5, 0.5, 0.5, 1.0),
                    Vector4::new(-0.5, 0.5, -0.5, 1.0),
                    Vector4::new(-0.5, -0.5, 0.5, 1.0),
                    Vector4::new(-0.5, -0.5, -0.5, 1.0),
                ];
                let edges = [
                    (
                        Vector4::new(0.5, 0.5, 0.5, 1.0),
                        Vector4::new(-1.0, 0.0, 0.0, 0.0),
                    ),
                    (
                        Vector4::new(0.5, 0.5, 0.5, 1.0),
                        Vector4::new(0.0, -1.0, 0.0, 0.0),
                    ),
                    (
                        Vector4::new(0.5, 0.5, 0.5, 1.0),
                        Vector4::new(0.0, 0.0, -1.0, 0.0),
                    ),
                    (
                        Vector4::new(0.5, -0.5, -0.5, 1.0),
                        Vector4::new(-1.0, 0.0, 0.0, 0.0),
                    ),
                    (
                        Vector4::new(0.5, -0.5, -0.5, 1.0),
                        Vector4::new(0.0, 1.0, 0.0, 0.0),
                    ),
                    (
                        Vector4::new(0.5, -0.5, -0.5, 1.0),
                        Vector4::new(0.0, 0.0, 1.0, 0.0),
                    ),
                    (
                        Vector4::new(-0.5, 0.5, -0.5, 1.0),
                        Vector4::new(1.0, 0.0, 0.0, 0.0),
                    ),
                    (
                        Vector4::new(-0.5, 0.5, -0.5, 1.0),
                        Vector4::new(0.0, -1.0, 0.0, 0.0),
                    ),
                    (
                        Vector4::new(-0.5, 0.5, -0.5, 1.0),
                        Vector4::new(0.0, 0.0, 1.0, 0.0),
                    ),
                    (
                        Vector4::new(-0.5, -0.5, 0.5, 1.0),
                        Vector4::new(1.0, 0.0, 0.0, 0.0),
                    ),
                    (
                        Vector4::new(-0.5, -0.5, 0.5, 1.0),
                        Vector4::new(0.0, 1.0, 0.0, 0.0),
                    ),
                    (
                        Vector4::new(-0.5, -0.5, 0.5, 1.0),
                        Vector4::new(0.0, 0.0, -1.0, 0.0),
                    ),
                ];
                points
                    .iter()
                    .map(|v| t1_inv * (t2 * v))
                    // .inspect(|p| assert_eq!(p.w, 1.0))
                    .filter(|p| {
                        p.x.abs() <= 0.5 && p.y.abs() <= 0.5 && p.z.abs() <= 0.5
                    })
                    .map(|p2| {
                        let (i, _) = p2.xyz().map(f64::abs).argmax();
                        let mut p1 = p2;
                        let mut n = Vector4::zeros();
                        p1[i] = p2[i].signum() / 2.0;
                        n[i] = p2[i].signum();
                        (
                            t1 * p1,
                            t1 * p2,
                            -(n.transpose() * t1_inv).transpose(),
                        )
                    })
                    .chain(
                        points
                            .iter()
                            .map(|v| t2_inv * (t1 * v))
                            // .inspect(|p| assert_eq!(p.w, 1.0))
                            .filter(|p| {
                                p.x.abs() <= 0.5
                                    && p.y.abs() <= 0.5
                                    && p.z.abs() <= 0.5
                            })
                            .map(|p1| {
                                let (i, _) = p1.xyz().map(f64::abs).argmax();
                                let mut p2 = p1;
                                let mut n = Vector4::zeros();
                                p2[i] = p1[i].signum() / 2.0;
                                n[i] = p1[i].signum();
                                (
                                    t2 * p1,
                                    t2 * p2,
                                    (n.transpose() * t2_inv).transpose(),
                                )
                            }),
                    )
                    .chain(
                        edges
                            .iter()
                            .map(|(p, v)| {
                                (t2_inv * (t1 * p), t2_inv * (t1 * v))
                            })
                            .flat_map(|(p1, v1)| {
                                edges.iter().map(move |(p2, v2)| {
                                    let n = v1.xyz().cross(&v2.xyz()).push(0.0);
                                    (
                                        p1,
                                        *p2,
                                        v1,
                                        *v2,
                                        if n.dot(&p1) < 0.0 { -n } else { n },
                                    )
                                })
                            })
                            .filter(|(p1, p2, _, _, n)| n.dot(p1) < n.dot(p2))
                            .filter_map(|(p1, p2, v1, v2, n)| {
                                let Some(a_inverse) = Matrix3::new(
                                    v1.x, v1.y, v1.z, -v2.x, -v2.y, -v2.z, n.x,
                                    n.y, n.z,
                                )
                                .transpose()
                                .try_inverse() else {
                                    return None;
                                };
                                let b = Vector3::new(
                                    p2.x - p1.x,
                                    p2.y - p1.y,
                                    p2.z - p1.z,
                                );
                                let t = a_inverse * b;
                                if t[0] < 0.0
                                    || t[0] > 1.0
                                    || t[1] < 0.0
                                    || t[1] > 1.0
                                {
                                    return None;
                                }
                                Some((p1 + v1 * t[0], p2 + v2 * t[1], n))
                            })
                            .map(|(p1, p2, n)| {
                                (
                                    t2 * p1,
                                    t2 * p2,
                                    (n.transpose() * t2_inv).transpose(),
                                )
                            }),
                    )
                    .next()
                    .map(|(p1, p2, normal)| Contact {
                        points: (
                            Point3::from(p1.xyz()),
                            Point3::from(p2.xyz()),
                        ),
                        normal: normal.xyz().normalize(),
                    })
            }
        }
    }

    #[allow(clippy::unused_self)]
    fn check_contact_gjk(&self, o1: &Object, o2: &Object) -> Option<Contact> {
        match gjk(
            &(o1.position, o1.rotation, o1.collider),
            &(o2.position, o2.rotation, o2.collider),
        ) {
            GJKResult::Contact { points, normal } => Some(Contact {
                points: (points.0.into(), points.1.into()),
                normal,
            }),
            GJKResult::NoContact => None,
            GJKResult::UnknownContact(_) => {
                eprintln!(
                    "gjk gave unknown contact, falling back to other solution"
                );
                self.check_contact(o1, o2)
            }
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

#[derive(Debug)]
struct Contact {
    points: (Point3<f64>, Point3<f64>),
    normal: Vector3<f64>,
}
