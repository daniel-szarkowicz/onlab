use std::rc::Rc;

use anyhow::Result;
use egui::{DragValue, Window};
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
    hud_shader_program: ShaderProgram<PVertex>,
    draw_phong: bool,
    draw_debug: bool,
    camera: FirstPersonCamera,
    bounding_box_mesh: Mesh<PVertex>,
    rectangle_mesh: Mesh<PVertex>,
    box_mesh: Rc<Mesh<PNVertex>>,
    sphere_mesh: Rc<Mesh<PNVertex>>,
    surface_width: f32,
    surface_height: f32,
    epsilon: f32,
    paused: bool,
}

impl MainScene {
    pub fn new(ctx: &Context) -> Result<Self> {
        let objects = vec![];
        let box_mesh = Rc::new(meshes::box_mesh(ctx)?);
        let sphere_mesh = Rc::new(meshes::sphere_mesh(ctx, 16, true)?);
        let bounding_box_mesh = meshes::bounding_box_mesh(ctx)?;
        let rectangle_mesh = meshes::rectangle_mesh(ctx)?;
        let phong_shader_program =
            ShaderProgram::new(ctx, "src/phong-vs.glsl", "src/phong-fs.glsl")?;
        let debug_shader_program = ShaderProgram::new(
            ctx,
            "src/simple-vs.glsl",
            "src/simple_color-fs.glsl",
        )?;
        let hud_shader_program = ShaderProgram::new(
            ctx,
            "src/simple-vs.glsl",
            "src/simple_color-fs.glsl",
        )?;
        Ok(Self {
            objects,
            phong_shader_program,
            debug_shader_program,
            hud_shader_program,
            draw_phong: true,
            draw_debug: false,
            camera: FirstPersonCamera::default(),
            bounding_box_mesh,
            rectangle_mesh,
            box_mesh,
            sphere_mesh,
            surface_width: 1.0,
            surface_height: 1.0,
            epsilon: 1.0,
            paused: false,
        })
    }

    fn preset_many_spheres(&mut self) {
        self.objects.clear();
        use Collider::*;
        for x in -5..=5 {
            for y in 5..=15 {
                for z in -5..=5 {
                    self.objects.push(Object {
                        position: Point3::new(
                            2.0 * x as f32,
                            2.0 * y as f32,
                            2.0 * z as f32,
                        ),
                        rotation: Rotation3::new(Vector3::new(0.0, 0.0, 0.0)),
                        mesh_scale: Vector3::new(0.5, 0.5, 0.5),
                        ..Object::new(&self.sphere_mesh, Sphere(0.5), 1.0)
                    });
                }
            }
        }
        self.objects.push(Object {
            immovable: true,
            position: Point3::new(0.0, -1.0, 0.0),
            mesh_scale: Vector3::new(10000.0, 1.0, 10000.0),
            ..Object::new(&self.box_mesh, Box(10000.0, 1.0, 10000.0), 1.0)
        });
    }

    fn preset_two_spheres(&mut self) {
        self.objects.clear();
        use Collider::*;
        for i in 0..2 {
            self.objects.push(Object {
                position: Point3::new(3.0 * i as f32, 0.0, 0.0),
                rotation: Rotation3::new(Vector3::new(0.0, 0.0, 0.0)),
                mesh_scale: Vector3::new(1.0, 1.0, 1.0),
                ..Object::new(&self.sphere_mesh, Sphere(1.0), 1.0)
            });
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

    fn draw_hud(&self, ctx: &mut Context) {
        unsafe {
            ctx.use_shader_program(&self.hud_shader_program);
            let model = ctx
                .gl
                .get_uniform_location(self.hud_shader_program.program, "model")
                .unwrap();
            let view_proj = ctx
                .gl
                .get_uniform_location(
                    self.hud_shader_program.program,
                    "view_proj",
                )
                .unwrap();
            let view_proj_m = Scale3::new(
                1.0 / self.surface_width,
                1.0 / self.surface_height,
                1.0,
            )
            .to_homogeneous();
            let model_m = Scale3::new(20.0, 20.0, 1.0).to_homogeneous();
            ctx.gl.uniform_matrix_4_f32_slice(
                Some(&view_proj),
                false,
                view_proj_m.as_slice(),
            );
            let color = ctx
                .gl
                .get_uniform_location(self.hud_shader_program.program, "color")
                .unwrap();
            ctx.gl.uniform_matrix_4_f32_slice(
                Some(&model),
                false,
                model_m.as_slice(),
            );
            ctx.gl.uniform_3_f32_slice(Some(&color), &[0.0, 0.0, 1.0]);
            ctx.draw_mesh(&self.rectangle_mesh);
        }
    }

    fn simulate(&mut self, delta: f32) {
        for i in 0..self.objects.len() {
            for j in (i + 1)..self.objects.len() {
                let (a, b) = self.objects.split_at_mut(j);
                let o1 = &mut a[i];
                let o2 = &mut b[0];
                use Collider::*;
                match (&o1.collider, &o2.collider) {
                    (Sphere(r1), Sphere(r2)) => {
                        // o2 -> o1
                        let center_distance = o1.position - o2.position;
                        if center_distance.magnitude() <= (r1 + r2) {
                            let norm = center_distance.normalize();
                            let p1 = o1.position - norm * *r1;
                            let p2 = o2.position + norm * *r2;
                            let v1 = o1.local_velocity(p1);
                            let v2 = o2.local_velocity(p2);
                            // o2 -> o1
                            let dv = norm.dot(&(v1 - v2));
                            if dv < 0.0 {
                                let impulse_strength = -(self.epsilon + 1.0)
                                    * dv
                                    / (1.0 / o1.mass
                                        + 1.0 / o2.mass
                                        + norm.dot(
                                            &(o1.inverse_inertia()
                                                * (p1 - o1.position)
                                                    .cross(&norm))
                                            .cross(&(p1 - o1.position)),
                                        )
                                        + norm.dot(
                                            &(o2.inverse_inertia()
                                                * (p2 - o2.position)
                                                    .cross(&norm))
                                            .cross(&(p2 - o2.position)),
                                        ));
                                o1.apply_impulse(p1, impulse_strength * norm);
                                o2.apply_impulse(p2, -impulse_strength * norm);
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
        for obj in &mut self.objects {
            obj.update(delta);
        }
    }
}

impl Scene for MainScene {
    fn draw(&mut self, ctx: &mut Context) {
        if self.camera.focus() {
            ctx.window
                .set_cursor_grab(CursorGrabMode::Locked)
                .or_else(|_| {
                    ctx.window.set_cursor_grab(CursorGrabMode::Confined)
                })
                .unwrap();
            ctx.window.set_cursor_visible(false);
        } else {
            ctx.window.set_cursor_visible(true);
            ctx.window.set_cursor_grab(CursorGrabMode::None).unwrap();
        }
        unsafe {
            ctx.gl.enable(glow::CULL_FACE);
            ctx.gl.clear_color(0.69, 0.0, 1.0, 1.0);
            ctx.gl
                .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            ctx.gl.enable(glow::DEPTH_TEST);
            if self.draw_phong {
                self.draw_phong(ctx);
            }
            if self.draw_debug {
                self.draw_debug(ctx);
            }
            if self.camera.focus() {
                self.draw_hud(ctx);
            }
            ctx.egui.run(&ctx.window, |egui_ctx| {
                Window::new("Debug").show(egui_ctx, |ui| {
                    ui.set_min_width(200.0);
                    ui.checkbox(&mut self.paused, "Pause");
                    if ui.button("Many spheres").clicked() {
                        self.preset_many_spheres();
                    }
                    if ui.button("Two spheres").clicked() {
                        self.preset_two_spheres();
                    }
                    ui.checkbox(&mut self.draw_phong, "Draw objects");
                    ui.checkbox(&mut self.draw_debug, "Draw bounds");
                    ui.add(
                        DragValue::new(&mut self.epsilon)
                            .prefix("Collision energy multiplier: ")
                            .clamp_range(-1.0..=2.0)
                            .speed(0.005),
                    );
                    let total_momentum = self
                        .objects
                        .iter()
                        .map(|o| o.momentum)
                        .sum::<Vector3<f32>>();
                    ui.label(format!(
                        "Total momentum: {}",
                        total_momentum.magnitude()
                    ));
                    let total_energy = self
                        .objects
                        .iter()
                        .map(|o| o.momentum.magnitude_squared() / o.mass / 2.0)
                        .sum::<f32>();
                    ui.label(format!("Total energy: {total_energy}"));
                });
            });
            ctx.egui.paint(&ctx.window);
            ctx.gl_surface.swap_buffers(&ctx.gl_context).unwrap();
        }
    }

    fn update(&mut self, delta: f32) {
        self.camera.update(delta);
        const TICK_RATE_TARGET: f32 = 100.0;
        const MAX_STEP_COUNT: u32 = 10;
        if !self.paused {
            let step_count =
                MAX_STEP_COUNT.min((delta * TICK_RATE_TARGET).ceil() as u32);
            let step_size = delta / step_count as f32;
            for _ in 0..step_count {
                self.simulate(step_size);
            }
        }
    }

    fn event<UserEvent>(&mut self, event: &Event<UserEvent>) -> bool {
        if self.camera.event(event) {
            return true;
        }
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::MouseInput {
                    state: ElementState::Pressed,
                    ..
                } if self.camera.focus() => {
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
                            2.0 * ray.direction,
                        )
                    }
                    true
                }
                WindowEvent::Resized(size) => {
                    self.surface_width = size.width as f32;
                    self.surface_height = size.height as f32;
                    false
                }
                _ => false,
            },
            _ => false,
        }
    }
}
