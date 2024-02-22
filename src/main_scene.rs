use std::rc::Rc;

use egui::DragValue;
use glow::HasContext;
use glutin::surface::GlSurface;
use nalgebra::{
    Matrix4, Perspective3, Point3, RealField, Rotation3, Translation3, Vector3,
};
use winit::event::{DeviceEvent, Event, KeyEvent, WindowEvent};
use winit::keyboard::{Key, NamedKey, SmolStr};
use winit::window::CursorGrabMode;

use crate::camera::FirstPersonCamera;
use crate::mesh::DrawMesh;
use crate::meshes;
use crate::object::Object;
use crate::shader_program::{ShaderProgram, UseShaderProgram};
use crate::{scene::Scene, vertex::PNVertex, Context};

pub struct MainScene {
    objects: Vec<Object>,
    program: ShaderProgram<PNVertex>,
    aspect: f32,
    x: f32,
    y: f32,
    forwards: bool,
    backwards: bool,
    left: bool,
    right: bool,
    up: bool,
    down: bool,
    camera: FirstPersonCamera,
}

impl MainScene {
    pub fn new(ctx: &Context) -> Self {
        let mut objects = vec![];
        let box_mesh = Rc::new(meshes::box_mesh(ctx).unwrap());
        let sphere_mesh = Rc::new(meshes::sphere_mesh(ctx, 16).unwrap());
        for x in 0..10 {
            objects.push(Object {
                mesh: box_mesh.clone(),
                position: Point3::new(2.0 * x as f32, 0.0, 0.0),
                rotation: Rotation3::new(Vector3::new(x as f32, 0.0, 0.0)),
            });
        }
        for x in 0..10 {
            objects.push(Object {
                mesh: sphere_mesh.clone(),
                position: Point3::new(2.0 * x as f32, 2.0, 0.0),
                rotation: Rotation3::new(Vector3::new(x as f32, 0.0, 0.0)),
            });
        }
        let program =
            ShaderProgram::new(ctx, "src/test-vs.glsl", "src/test-fs.glsl")
                .unwrap();
        Self {
            objects,
            program,
            x: 0.0,
            y: 0.0,
            aspect: 1.0,
            forwards: false,
            backwards: false,
            left: false,
            right: false,
            up: false,
            down: false,
            camera: FirstPersonCamera::new(),
        }
    }
}

impl Scene for MainScene {
    fn draw(&mut self, ctx: &mut Context) {
        ctx.window
            .set_cursor_grab(CursorGrabMode::Confined)
            .or_else(|_| ctx.window.set_cursor_grab(CursorGrabMode::Locked))
            .unwrap();
        // ctx.window.set_cursor_visible(false);
        unsafe {
            ctx.gl.clear_color(0.69, 0.0, 1.0, 1.0);
            ctx.gl
                .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            ctx.gl.enable(glow::DEPTH_TEST);
            ctx.egui.run(&ctx.window, |egui_ctx| {
                egui::Window::new("Hello").show(egui_ctx, |ui| {
                    ui.add(DragValue::new(&mut self.x).speed(0.01));
                    ui.add(DragValue::new(&mut self.y).speed(0.01));
                });
            });
            ctx.use_shader_program(&self.program);
            let model = ctx
                .gl
                .get_uniform_location(self.program.program, "model")
                .unwrap();
            let model_inv = ctx
                .gl
                .get_uniform_location(self.program.program, "model_inv")
                .unwrap();
            let view_proj = ctx
                .gl
                .get_uniform_location(self.program.program, "view_proj")
                .unwrap();
            let view_proj_m = self.camera.view_proj(self.aspect);
            ctx.gl.uniform_matrix_4_f32_slice(
                Some(&view_proj),
                false,
                view_proj_m.as_slice(),
            );
            for object in &self.objects {
                let model_m = object.model();
                let model_inv_m = model_m.try_inverse().unwrap();
                ctx.gl.uniform_matrix_4_f32_slice(
                    Some(&model),
                    false,
                    model_m.as_slice(),
                );
                ctx.gl.uniform_matrix_4_f32_slice(
                    Some(&model_inv),
                    false,
                    model_inv_m.as_slice(),
                );
                ctx.draw_mesh(&object.mesh);
            }
            ctx.egui.paint(&ctx.window);
            ctx.gl_surface.swap_buffers(&ctx.gl_context).unwrap();
        }
    }

    fn update(&mut self, delta: f32) {
        let mut dir = Vector3::zeros();
        if self.forwards {
            dir.z += 1.0
        }
        if self.backwards {
            dir.z -= 1.0
        }
        if self.left {
            dir.x += 1.0
        }
        if self.right {
            dir.x -= 1.0
        }
        if self.up {
            dir.y += 1.0
        }
        if self.down {
            dir.y -= 1.0
        }
        if dir.magnitude() != 0.0 {
            self.camera.move_facing(dir.normalize() * delta * 3.0);
        }
    }

    fn event<UserEvent>(&mut self, event: &Event<UserEvent>) -> bool {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => {
                    self.aspect = size.width as f32 / size.height as f32;
                    false
                }
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            logical_key, state, ..
                        },
                    ..
                } => {
                    match logical_key {
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
                            _ => {}
                        },
                        _ => {}
                    }
                    false
                }
                _ => false,
            },
            Event::DeviceEvent { event, .. } => {
                if let DeviceEvent::MouseMotion { delta } = event {
                    self.camera.yaw -= delta.0 as f32 * 0.15;
                    self.camera.pitch = (self.camera.pitch
                        + delta.1 as f32 * 0.15)
                        .clamp(-89.0, 89.0);
                }
                false
            }

            _ => false,
        }
    }
}
