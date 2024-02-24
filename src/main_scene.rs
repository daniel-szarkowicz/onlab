use std::rc::Rc;

use egui::Window;
use glow::HasContext;
use glutin::surface::GlSurface;
use nalgebra::{Point3, Rotation3, Scale3, Translation3, Vector3};
use winit::event::{ElementState, Event, WindowEvent};
use winit::window::CursorGrabMode;

use crate::camera::FirstPersonCamera;
use crate::collider::Collider;
use crate::mesh::{DrawMesh, Mesh};
use crate::meshes;
use crate::object::Object;
use crate::shader_program::{ShaderProgram, UseShaderProgram};
use crate::vertex::PVertex;
use crate::{scene::Scene, vertex::PNVertex, Context};

pub struct MainScene {
    objects: Vec<Object>,
    phong_shader_program: ShaderProgram<PNVertex>,
    debug_shader_program: ShaderProgram<PVertex>,
    camera: FirstPersonCamera,
    bounding_box_mesh: Mesh<PVertex>,
}

impl MainScene {
    pub fn new(ctx: &Context) -> Self {
        let mut objects = vec![];
        let box_mesh = Rc::new(meshes::box_mesh(ctx).unwrap());
        let sphere_mesh = Rc::new(meshes::sphere_mesh(ctx, 64, true).unwrap());
        let bounding_box_mesh = meshes::bounding_box_mesh(ctx).unwrap();
        use Collider::*;
        for x in 0..10 {
            objects.push(Object {
                position: Point3::new(2.0 * x as f32, 0.0, 0.0),
                rotation: Rotation3::new(Vector3::new(x as f32, 0.0, 0.0)),
                mesh_scale: Vector3::new(1.0, 20.0, 1.0),
                ..Object::new(&box_mesh, Box(1.0, 20.0, 1.0), 1.0)
            });
        }
        for x in 0..10 {
            objects.push(Object {
                position: Point3::new(2.0 * x as f32, 2.0, 0.0),
                rotation: Rotation3::new(Vector3::new(x as f32, 0.0, 0.0)),
                ..Object::new(&sphere_mesh, Sphere(1.0), 1.0)
            });
        }
        objects.push(Object {
            immovable: true,
            position: Point3::new(0.0, -1.0, 0.0),
            mesh_scale: Vector3::new(10000.0, 1.0, 10000.0),
            ..Object::new(&box_mesh, Box(10000.0, 1.0, 10000.0), 1.0)
        });
        let phong_shader_program =
            ShaderProgram::new(ctx, "src/phong-vs.glsl", "src/phong-fs.glsl")
                .unwrap();
        let debug_shader_program = ShaderProgram::new(
            ctx,
            "src/simple-vs.glsl",
            "src/simple_color-fs.glsl",
        )
        .unwrap();
        Self {
            objects,
            phong_shader_program,
            debug_shader_program,
            camera: FirstPersonCamera::default(),
            bounding_box_mesh,
        }
    }

    fn draw_phong(&self, ctx: &mut Context) {
        unsafe {
            ctx.use_shader_program(&self.phong_shader_program);
            let model = ctx
                .gl
                .get_uniform_location(
                    self.phong_shader_program.program,
                    "model",
                )
                .unwrap();
            let model_inv = ctx
                .gl
                .get_uniform_location(
                    self.phong_shader_program.program,
                    "model_inv",
                )
                .unwrap();
            let view_proj = ctx
                .gl
                .get_uniform_location(
                    self.phong_shader_program.program,
                    "view_proj",
                )
                .unwrap();
            let w_eye = ctx
                .gl
                .get_uniform_location(self.phong_shader_program.program, "wEye")
                .unwrap();
            let view_proj_m = self.camera.view_proj();
            ctx.gl.uniform_matrix_4_f32_slice(
                Some(&view_proj),
                false,
                view_proj_m.as_slice(),
            );
            ctx.gl.uniform_3_f32_slice(
                Some(&w_eye),
                self.camera.position().coords.as_slice(),
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
        }
    }

    fn draw_debug(&self, ctx: &mut Context) {
        unsafe {
            ctx.use_shader_program(&self.debug_shader_program);
            let model = ctx
                .gl
                .get_uniform_location(
                    self.debug_shader_program.program,
                    "model",
                )
                .unwrap();
            let view_proj = ctx
                .gl
                .get_uniform_location(
                    self.debug_shader_program.program,
                    "view_proj",
                )
                .unwrap();
            let view_proj_m = self.camera.view_proj();
            ctx.gl.uniform_matrix_4_f32_slice(
                Some(&view_proj),
                false,
                view_proj_m.as_slice(),
            );
            let color = ctx
                .gl
                .get_uniform_location(
                    self.debug_shader_program.program,
                    "color",
                )
                .unwrap();
            for object in &self.objects {
                let (min, max) = object.aabb();
                let size = max - min;
                let pos = min + size / 2.0;
                let model_m = Translation3::from(pos).to_homogeneous()
                    * Scale3::from(size).to_homogeneous();

                ctx.gl.uniform_matrix_4_f32_slice(
                    Some(&model),
                    false,
                    model_m.as_slice(),
                );
                ctx.gl.uniform_3_f32_slice(Some(&color), &[1.0, 0.0, 0.0]);
                ctx.draw_mesh(&self.bounding_box_mesh);
            }
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
            ctx.gl.enable(glow::CULL_FACE);
            ctx.gl.clear_color(0.69, 0.0, 1.0, 1.0);
            ctx.gl
                .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            ctx.gl.enable(glow::DEPTH_TEST);
            self.draw_phong(ctx);
            self.draw_debug(ctx);
            ctx.egui.run(&ctx.window, |egui_ctx| {
                Window::new("Hello").show(egui_ctx, |_| {});
            });
            ctx.egui.paint(&ctx.window);
            ctx.gl_surface.swap_buffers(&ctx.gl_context).unwrap();
        }
    }

    fn update(&mut self, delta: f32) {
        self.camera.update(delta);
        for obj in &mut self.objects {
            obj.update(delta);
        }
    }

    fn event<UserEvent>(&mut self, event: &Event<UserEvent>) -> bool {
        if self.camera.event(event) {
            return true;
        }
        match event {
            Event::WindowEvent {
                event:
                    WindowEvent::MouseInput {
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                let ray = self.camera.get_ray();
                if let Some((t, o)) = self
                    .objects
                    .iter_mut()
                    .filter_map(|o| {
                        o.collider
                            .check_ray_hit(o.position, o.rotation, &ray)
                            .map(|h| (h, o))
                    })
                    .min_by(|(h1, _), (h2, _)| h1.total_cmp(h2))
                {
                    o.apply_impulse(
                        ray.start + ray.direction * t,
                        ray.direction,
                    )
                }
                false
            }
            _ => false,
        }
    }
}
