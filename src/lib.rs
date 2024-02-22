use std::num::NonZeroU32;
use std::sync::{Arc, Mutex};

use glow::HasContext;
use glutin::config::{Config, ConfigTemplateBuilder, GlConfig};
use glutin::context::{
    ContextApi, ContextAttributesBuilder, NotCurrentGlContext,
    PossiblyCurrentContext,
};
use glutin::display::{GetGlDisplay, GlDisplay};
use glutin::surface::{GlSurface, Surface, SwapInterval};
use glutin_winit::{DisplayBuilder, GlWindow};
use raw_window_handle::HasRawWindowHandle;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

pub mod camera;
pub mod main_scene;
pub mod mesh;
pub mod meshes;
pub mod object;
pub mod scene;
pub mod shader_program;
pub mod vertex;

impl Context {
    pub fn new(event_loop: &EventLoop<UserEvent>) -> Self {
        let window_builder = WindowBuilder::new()
            .with_title("Anya egy cím vagyok!")
            .with_inner_size(LogicalSize::new(1280, 720));

        let template = ConfigTemplateBuilder::new();

        let display_builder = DisplayBuilder::new()
            .with_window_builder(Some(window_builder.clone()));

        let (window, gl_config) = display_builder
            .build(&event_loop, template, |configs| {
                configs.max_by_key(Config::num_samples).unwrap()
            })
            .unwrap();

        let gl_display = gl_config.display();
        let raw_window_handle = window.as_ref().map(|w| w.raw_window_handle());
        let context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(
                glutin::context::Version { major: 4, minor: 3 },
            )))
            .build(raw_window_handle);

        let not_current_gl_context = unsafe {
            gl_display
                .create_context(&gl_config, &context_attributes)
                .unwrap()
        };

        let window = window.unwrap_or_else(|| {
            glutin_winit::finalize_window(
                &event_loop,
                window_builder,
                &gl_config,
            )
            .unwrap()
        });
        let attrs = window.build_surface_attributes(Default::default());
        let gl_surface = unsafe {
            gl_display
                .create_window_surface(&gl_config, &attrs)
                .unwrap()
        };

        let gl_context =
            not_current_gl_context.make_current(&gl_surface).unwrap();

        let mut gl = unsafe {
            glow::Context::from_loader_function_cstr(|s| {
                gl_display.get_proc_address(s)
            })
        };
        unsafe {
            gl.enable(glow::DEBUG_OUTPUT);
            gl.debug_message_callback(
                |_source, _typ, _id, _severity, message| {
                    println!("{}", message);
                },
            );
        }

        gl_surface
            .set_swap_interval(
                &gl_context,
                SwapInterval::Wait(NonZeroU32::new(1).unwrap()),
            )
            .unwrap();

        #[allow(clippy::arc_with_non_send_sync)]
        let gl = Arc::new(gl);

        let egui_glow =
            egui_glow::EguiGlow::new(&event_loop, gl.clone(), None, None);

        let event_loop_proxy = Mutex::new(event_loop.create_proxy());
        egui_glow.egui_ctx.set_request_repaint_callback(move |_| {
            event_loop_proxy
                .lock()
                .unwrap()
                .send_event(UserEvent::Redraw)
                .unwrap();
        });

        Self {
            gl,
            gl_surface,
            gl_context,
            egui: egui_glow,
            window,
        }
    }
}

#[derive(Debug)]
pub enum UserEvent {
    Redraw,
}

pub struct Context {
    pub gl: Arc<glow::Context>,
    pub gl_surface: Surface<glutin::surface::WindowSurface>,
    pub gl_context: PossiblyCurrentContext,
    pub egui: egui_glow::EguiGlow,
    pub window: Window,
}
