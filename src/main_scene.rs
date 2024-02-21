use egui::DragValue;
use glow::HasContext;
use glutin::surface::GlSurface;
use nalgebra::{Matrix4, Perspective3, Point3, Translation3, Vector3};
use winit::event::{DeviceEvent, Event};
use winit::window::CursorGrabMode;

use crate::mesh::{DrawMesh, Mesh};
use crate::shader_program::{ShaderProgram, UseShaderProgram};
use crate::{scene::Scene, vertex::PNVertex, Context};

pub struct MainScene {
    meshes: Vec<Mesh<PNVertex>>,
    program: ShaderProgram<PNVertex>,
    aspect: f32,
    x: f32,
    y: f32,
    camera_x: f32,
    camera_y: f32,
    camera_z: f32,
}

impl MainScene {
    pub fn new(ctx: &Context) -> Self {
        let mut meshes = Vec::new();
        meshes.push(crate::meshes::box_mesh(ctx).unwrap());
        let program =
            ShaderProgram::new(ctx, "src/test-vs.glsl", "src/test-fs.glsl")
                .unwrap();
        Self {
            meshes,
            program,
            x: 0.0,
            y: 0.0,
            aspect: 1.0,
            camera_x: 1.0,
            camera_y: 1.0,
            camera_z: 1.0,
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
                    ui.label("Camera position");
                    ui.add(DragValue::new(&mut self.camera_x).speed(0.01));
                    ui.add(DragValue::new(&mut self.camera_y).speed(0.01));
                    ui.add(DragValue::new(&mut self.camera_z).speed(0.01));
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
            let model_m =
                Translation3::new(self.x, self.y, 0.0).to_homogeneous();
            let model_inv_m = model_m.try_inverse().unwrap();
            let view_proj_m = Perspective3::new(self.aspect, 30.0, 0.1, 1000.0)
                .as_matrix()
                * Matrix4::look_at_rh(
                    &Point3::new(self.camera_x, self.camera_y, self.camera_z),
                    &Point3::new(0.0, 0.0, 0.0),
                    &Vector3::new(0.0, -1.0, 0.0),
                );
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
            ctx.gl.uniform_matrix_4_f32_slice(
                Some(&view_proj),
                false,
                view_proj_m.as_slice(),
            );
            for mesh in &self.meshes {
                ctx.draw_mesh(mesh);
            }
            ctx.egui.paint(&ctx.window);
            ctx.gl_surface.swap_buffers(&ctx.gl_context).unwrap();
        }
    }

    fn update(&mut self, delta: f32) {
        // todo!()
    }

    fn event<UserEvent>(&mut self, event: &Event<UserEvent>) -> bool {
        match event {
            Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::Resized(size) => {
                    self.aspect = size.width as f32 / size.height as f32;
                    false
                }
                _ => false,
            },
            Event::DeviceEvent { event, .. } => {
                if let DeviceEvent::MouseMotion { delta } = event {
                    // self.camera.rotate(delta)
                }
                false
            }

            _ => false,
        }
    }
}
