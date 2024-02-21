use glow::HasContext;
use glutin::surface::GlSurface;
use winit::{
    event::{DeviceEvent, Event},
    window::CursorGrabMode,
};

use crate::mesh::{DrawMesh, Mesh, MeshPrimitive};
use crate::shader_program::{ShaderProgram, UseShaderProgram};
use crate::{scene::Scene, vertex::PNVertex, Context};

pub struct MainScene {
    meshes: Vec<Mesh<PNVertex>>,
    program: ShaderProgram<PNVertex>,
}

impl MainScene {
    pub fn new(ctx: &Context) -> Self {
        let mut meshes = Vec::new();
        meshes.push(
            Mesh::new(
                ctx,
                &[
                    PNVertex {
                        position: [0.0, 1.0, 0.0],
                        normal: [0.0, 0.0, 1.0],
                    },
                    PNVertex {
                        position: [1.0, 0.0, 0.0],
                        normal: [0.0, 0.0, 1.0],
                    },
                    PNVertex {
                        position: [0.0, 0.0, 0.0],
                        normal: [0.0, 0.0, 1.0],
                    },
                ],
                &[0, 1, 2],
                MeshPrimitive::Triangles,
            )
            .unwrap(),
        );
        meshes.push(
            Mesh::new(
                ctx,
                &[
                    PNVertex {
                        position: [0.0, 0.0, 0.0],
                        normal: [0.0, 0.0, 1.0],
                    },
                    PNVertex {
                        position: [-1.0, 0.0, 0.0],
                        normal: [0.0, 0.0, 1.0],
                    },
                    PNVertex {
                        position: [0.0, -1.0, 0.0],
                        normal: [0.0, 0.0, 1.0],
                    },
                    PNVertex {
                        position: [-1.0, -1.0, 0.0],
                        normal: [0.0, 0.0, 1.0],
                    },
                ],
                &[0, 1, 2, 2, 1, 3],
                MeshPrimitive::Triangles,
            )
            .unwrap(),
        );
        let program =
            ShaderProgram::new(ctx, "src/test-vs.glsl", "src/test-fs.glsl")
                .unwrap();
        Self { meshes, program }
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
            ctx.gl.clear(glow::COLOR_BUFFER_BIT);
            ctx.egui.run(&ctx.window, |egui_ctx| {
                egui::Window::new("Hello").show(egui_ctx, |ui| {});
            });
            ctx.use_shader_program(&self.program);
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
