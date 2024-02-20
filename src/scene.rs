use crate::Context;

pub trait Scene {
    fn draw(&mut self, ctx: &mut Context);
    fn update(&mut self, delta: f32);
    fn event(&mut self, event: &winit::event::WindowEvent) -> bool;
}
