use egui::widgets::DragValue;
use glow::HasContext;
use glutin::surface::GlSurface;

use crate::{scene::Scene, Context};

pub struct MainScene {
    value: f32,
}

impl MainScene {
    pub fn new(ctx: &Context) -> Self {
        Self { value: 0.0 }
    }
}

impl Scene for MainScene {
    fn draw(&mut self, ctx: &mut Context) {
        unsafe {
            ctx.gl.as_ref().clear_color(0.69, 0.0, 1.0, 1.0);
            ctx.gl.clear(glow::COLOR_BUFFER_BIT);
            ctx.egui.run(&ctx.window, |egui_ctx| {
                egui::Window::new("Hello").show(egui_ctx, |ui| {
                    ui.add(DragValue::new(&mut self.value).speed(0.1));
                });
            });
            ctx.egui.paint(&ctx.window);
            ctx.gl_surface.swap_buffers(&ctx.gl_context).unwrap();
        }
    }

    fn update(&mut self, delta: f32) {
        // todo!()
    }

    fn event(&mut self, event: &winit::event::WindowEvent) -> bool {
        // todo!()
        return false;
    }
}
