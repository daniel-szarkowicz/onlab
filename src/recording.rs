#![allow(warnings)]

use egui::Frame;
use nalgebra::{Point3, Rotation3};

use crate::object::Object;

#[derive(Debug, Default)]
struct RecordingData {
    position: Point3<f64>,
    rotation: (f64, f64, f64),
}

#[derive(Debug, Default)]
pub struct Recording {
    object_count: usize,
    data: Vec<RecordingData>,
    frame_count: usize,
}

impl Recording {
    pub fn save_frame_from(&mut self, objects: &[Object]) {
        if objects.len() != self.object_count {
            self.data.clear();
            self.frame_count = 0;
            self.object_count = objects.len();
        }
        self.data.extend(objects.into_iter().map(|o| RecordingData {
            position: o.position,
            rotation: o.rotation.euler_angles(),
        }));
        self.frame_count += 1;
    }

    pub fn load_frame_to(&self, frame_index: usize, objects: &mut [Object]) {
        if self.frame_count == 0 {
            return;
        }
        let frame_index = frame_index.clamp(0, self.last_frame_index());
        let frame_start = frame_index * self.object_count;
        for (o, d) in objects.into_iter().zip(&self.data[frame_start..]) {
            o.position = d.position;
            o.rotation = Rotation3::from_euler_angles(
                d.rotation.0,
                d.rotation.1,
                d.rotation.2,
            );
        }
    }

    pub fn frame_count(&self) -> usize {
        self.frame_count
    }

    pub fn last_frame_index(&self) -> usize {
        self.frame_count.checked_sub(1).unwrap_or(0)
    }

    pub fn clear(&mut self) {
        self.object_count = 0;
        self.data.clear();
        self.frame_count = 0;
    }
}
