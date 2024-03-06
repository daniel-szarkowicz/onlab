use std::time::Instant;

use anyhow::Result;
use glutin::surface::GlSurface;
use onlab::{context::Context, context::UserEvent};
use winit::event::Event;

use onlab::main_scene::MainScene;
use onlab::scene::Scene;
use winit::event_loop::EventLoopBuilder;

fn main() -> Result<()> {
    let event_loop = EventLoopBuilder::with_user_event().build()?;
    let mut ctx = Context::new(&event_loop);
    let mut scene = MainScene::new(&ctx)?;
    let mut prev_time = Instant::now();

    event_loop.run(move |event, elwt| {
        if !scene.event(&event) {
            match event {
                Event::WindowEvent { window_id, event } => {
                    if window_id == ctx.window.id() {
                        let response =
                            ctx.egui.on_window_event(&ctx.window, &event);
                        if response.repaint {
                            ctx.window.request_redraw();
                        }
                        if !response.consumed {
                            match event {
                                winit::event::WindowEvent::Resized(size) => {
                                    ctx.gl_surface.resize(
                                        &ctx.gl_context,
                                        size.width
                                            .try_into()
                                            .expect("Screen width is zero!"),
                                        size.height
                                            .try_into()
                                            .expect("Screen height is zero!"),
                                    );
                                }
                                winit::event::WindowEvent::CloseRequested
                                | winit::event::WindowEvent::Destroyed => {
                                    elwt.exit();
                                }
                                winit::event::WindowEvent::RedrawRequested => {
                                    let time = Instant::now();
                                    let dt = time - prev_time;
                                    // println!("fps {}", 1.0 / dt.as_secs_f32());
                                    prev_time = time;
                                    scene.update(dt.as_secs_f64());
                                    scene.draw(&mut ctx, dt.as_secs_f64());
                                    ctx.window.request_redraw();
                                }
                                _ => {}
                            }
                        }
                    }
                }
                Event::UserEvent(UserEvent::Redraw) => {
                    ctx.window.request_redraw();
                }
                _ => {}
            }
        }
    })?;
    Ok(())
}
