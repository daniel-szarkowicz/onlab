use glow::HasContext;
use glutin::surface::GlSurface;
use winit::{
    event::{DeviceEvent, Event},
    window::CursorGrabMode,
};

use crate::{
    scene::Scene, vertex::PNVertex, vertex_array::DrawVertexArray, Context,
};
use crate::{
    shader_program::{ShaderProgram, UseShaderProgram},
    vertex_array::VertexArray,
};

pub struct MainScene {
    vertex_array: VertexArray<PNVertex>,
    program: ShaderProgram<PNVertex>,
}

impl MainScene {
    pub fn new(ctx: &Context) -> Self {
        // let ib = unsafe {
        //     let ib = gl.create_buffer().unwrap();
        //     gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ib));
        //     let indicies = [0u16, 1, 2];
        //     gl.buffer_data_u8_slice(
        //         glow::ELEMENT_ARRAY_BUFFER,
        //         bytemuck::cast_slice(&indicies),
        //         glow::STATIC_DRAW,
        //     );
        //     ib
        // };
        let vertex_array = VertexArray::new(
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
        )
        .unwrap();
        let program =
            ShaderProgram::new(ctx, "src/test-vs.glsl", "src/test-fs.glsl")
                .unwrap();
        Self {
            vertex_array,
            program,
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
            ctx.gl.clear(glow::COLOR_BUFFER_BIT);
            ctx.egui.run(&ctx.window, |egui_ctx| {
                egui::Window::new("Hello").show(egui_ctx, |ui| {});
            });
            ctx.use_shader_program(&self.program);
            ctx.draw_triangles(&self.vertex_array);
            // ctx.gl.bind_vertex_array(Some(self.vertex_array.vao));
            // ctx.gl
            //     .draw_elements(glow::TRIANGLES, 3, glow::UNSIGNED_SHORT, 0);
            // ctx.gl.draw_arrays(glow::TRIANGLES, 0, 3);
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
