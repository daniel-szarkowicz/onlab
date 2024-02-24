use nalgebra::{Matrix4, Perspective3, Point3, Rotation3, Vector3};
use winit::event::{DeviceEvent, Event, KeyEvent, WindowEvent};
use winit::keyboard::{Key, NamedKey};

use crate::ray::Ray;

pub struct FirstPersonCamera {
    position: Point3<f32>,
    forwards: bool,
    backwards: bool,
    left: bool,
    right: bool,
    up: bool,
    down: bool,
    fast: bool,
    aspect: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub slow_speed: f32,
    pub fast_speed: f32,
}

impl FirstPersonCamera {
    const UP: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
    const FORWARD: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);
    const LEFT: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);

    pub fn position(&self) -> Point3<f32> {
        self.position
    }

    fn yaw_rotation(&self) -> Rotation3<f32> {
        Rotation3::new(Self::UP * self.yaw.to_radians())
    }

    fn pitch_rotation(&self) -> Rotation3<f32> {
        Rotation3::new(Self::LEFT * self.pitch.to_radians())
    }

    pub fn look_direction(&self) -> Vector3<f32> {
        self.yaw_rotation() * self.pitch_rotation() * Self::FORWARD
    }

    fn look_at(&self) -> Point3<f32> {
        self.position + self.look_direction()
    }

    pub fn move_facing(&mut self, direction: Vector3<f32>) {
        self.position += self.yaw_rotation() * direction
    }

    pub fn view_proj(&self) -> Matrix4<f32> {
        Perspective3::new(self.aspect, 60.0f32.to_radians(), 0.1, 1000.0)
            .to_homogeneous()
            * Matrix4::look_at_rh(&self.position, &self.look_at(), &Self::UP)
    }

    #[rustfmt::skip]
    pub fn update(&mut self, delta: f32) {
        let mut dir = Vector3::zeros();
        if self.forwards  { dir += Self::FORWARD; }
        if self.backwards { dir -= Self::FORWARD; }
        if self.left      { dir += Self::LEFT;    }
        if self.right     { dir -= Self::LEFT;    }
        if self.up        { dir += Self::UP;      }
        if self.down      { dir -= Self::UP;      }
        let speed = if self.fast {
            self.fast_speed
        } else {
            self.slow_speed
        };
        if dir.magnitude() != 0.0 {
            self.move_facing(dir.normalize() * delta * speed);
        }
    }

    pub fn event<T>(&mut self, event: &Event<T>) -> bool {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => {
                    self.aspect = size.width as f32 / size.height as f32
                }
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            logical_key, state, ..
                        },
                    ..
                } => match logical_key {
                    Key::Character(ch) => match ch.as_str() {
                        "w" | "W" => self.forwards = state.is_pressed(),
                        "s" | "S" => self.backwards = state.is_pressed(),
                        "a" | "A" => self.left = state.is_pressed(),
                        "d" | "D" => self.right = state.is_pressed(),
                        _ => {}
                    },
                    Key::Named(key) => match key {
                        NamedKey::Shift => self.down = state.is_pressed(),
                        NamedKey::Space => self.up = state.is_pressed(),
                        NamedKey::Control => self.fast = state.is_pressed(),
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            },
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                self.yaw -= delta.0 as f32 * 0.15;
                self.pitch =
                    (self.pitch + delta.1 as f32 * 0.15).clamp(-89.9, 89.9);
            }
            _ => {}
        }
        false
    }

    pub fn get_ray(&self) -> Ray {
        Ray {
            start: self.position,
            direction: self.look_direction(),
        }
    }
}

impl Default for FirstPersonCamera {
    fn default() -> Self {
        Self {
            position: Point3::new(0.0, 0.0, 0.0),
            yaw: 0.0,
            pitch: 0.0,
            forwards: false,
            backwards: false,
            left: false,
            right: false,
            fast: false,
            up: false,
            down: false,
            aspect: 1.0,
            slow_speed: 3.0,
            fast_speed: 10.0,
        }
    }
}
