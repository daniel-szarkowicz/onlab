use crate::context::Context;

pub trait Scene {
    fn draw(&mut self, ctx: &mut Context, delta: f64);
    fn update(&mut self, delta: f64);
    fn event<T>(&mut self, event: &winit::event::Event<T>) -> bool;
}
