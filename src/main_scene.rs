use glow::{HasContext, NativeProgram, NativeVertexArray};
use glutin::surface::GlSurface;
use winit::{
    event::{DeviceEvent, Event},
    window::CursorGrabMode,
};

use crate::{scene::Scene, Context};

pub struct MainScene {
    vao: NativeVertexArray,
    program: NativeProgram,
}

impl MainScene {
    pub fn new(ctx: &Context) -> Self {
        let gl = &ctx.gl;
        let vao = unsafe {
            let vao = gl.create_vertex_array().unwrap();
            gl.bind_vertex_array(Some(vao));
            let vbo = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            let positions =
                [[0.0f32, 1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 0.0]];
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&positions),
                glow::STATIC_DRAW,
            );
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 0, 0);

            let ib = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ib));
            let indicies = [0u16, 1, 2];
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(&indicies),
                glow::STATIC_DRAW,
            );
            gl.bind_vertex_array(None);
            vao
        };
        let program = unsafe {
            let program = gl.create_program().unwrap();
            let vertex = gl.create_shader(glow::VERTEX_SHADER).unwrap();
            gl.shader_source(vertex, include_str!("test-vs.glsl"));
            gl.compile_shader(vertex);
            gl.attach_shader(program, vertex);

            let fragment = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();
            gl.shader_source(fragment, include_str!("test-fs.glsl"));
            gl.compile_shader(fragment);
            gl.attach_shader(program, fragment);
            gl.link_program(program);
            program
        };
        Self { vao, program }
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
            ctx.gl.use_program(Some(self.program));
            ctx.gl.bind_vertex_array(Some(self.vao));
            ctx.gl
                .draw_elements(glow::TRIANGLES, 3, glow::UNSIGNED_SHORT, 0);
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
