use std::rc::Rc;

use anyhow::Result;
use egui::{DragValue, Ui, Window};
use glow::{HasContext, NativeFramebuffer, NativeTexture};
use glutin::surface::GlSurface;
use nalgebra::{
    Matrix4, Orthographic3, Point3, Rotation3, Scale3, Translation3, Vector3,
    Vector4,
};
use rand::Rng;
use winit::event::{ElementState, Event, WindowEvent};
use winit::window::CursorGrabMode;

use crate::camera::FirstPersonCamera;
use crate::collider::Collider;
use crate::mesh::{DrawMesh, Mesh};
use crate::object::Object;
use crate::shader_program::{ShaderProgram, UseShaderProgram};
use crate::simulation::Simulation;
use crate::vertex::PVertex;
use crate::{context::Context, scene::Scene, vertex::PNVertex};
use crate::{meshes, shadow_util};

#[derive(Debug)]
#[allow(clippy::struct_excessive_bools)]
pub struct MainScene {
    objects: Vec<Object>,
    depth_pass_program: ShaderProgram<PNVertex>,
    phong_shader_program: ShaderProgram<PNVertex>,
    debug_shader_program: ShaderProgram<PVertex>,
    hud_shader_program: ShaderProgram<PVertex>,
    depth_pass: bool,
    draw_phong: bool,
    draw_debug: bool,
    camera: FirstPersonCamera,
    bounding_box_mesh: Mesh<PVertex>,
    rectangle_mesh: Mesh<PVertex>,
    box_mesh: Rc<Mesh<PNVertex>>,
    sphere_mesh: Rc<Mesh<PNVertex>>,
    surface_width: f32,
    surface_height: f32,
    paused: bool,
    simulation: Simulation,
    max_depth: usize,
    light_pos: Vector4<f32>,
    shadow_buffer: NativeFramebuffer,
    shadow_map: NativeTexture,
}

impl MainScene {
    pub fn new(ctx: &Context) -> Result<Self> {
        let objects = vec![];
        let box_mesh = Rc::new(meshes::box_mesh(ctx)?);
        let sphere_mesh = Rc::new(meshes::sphere_mesh(ctx, 16, false)?);
        let bounding_box_mesh = meshes::bounding_box_mesh(ctx)?;
        let rectangle_mesh = meshes::rectangle_mesh(ctx)?;
        let depth_pass_program =
            ShaderProgram::new(ctx, "src/simple-vs.glsl", "src/empty-fs.glsl")?;
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
        let (shadow_buffer, shadow_map) =
            shadow_util::create_buffer(ctx.gl.as_ref());
        Ok(Self {
            objects,
            depth_pass_program,
            phong_shader_program,
            debug_shader_program,
            hud_shader_program,
            depth_pass: true,
            draw_phong: true,
            draw_debug: false,
            camera: FirstPersonCamera::default(),
            bounding_box_mesh,
            rectangle_mesh,
            box_mesh,
            sphere_mesh,
            surface_width: 1.0,
            surface_height: 1.0,
            paused: false,
            simulation: Simulation::default(),
            max_depth: 5,
            light_pos: Vector4::new(1.0, 1.0, -1.0, 0.0),
            shadow_buffer,
            shadow_map,
        })
    }

    fn preset_many_spheres(&mut self) {
        self.objects.clear();
        let mut random = rand::thread_rng();
        for x in -7..=7 {
            for y in 2..=16 {
                for z in -7..=7 {
                    let r = random.gen_range(0.25..=1.5);
                    self.objects.push(Object {
                        position: Point3::new(
                            f64::from(x)
                                .mul_add(4.0, random.gen_range(-0.5..=0.5)),
                            f64::from(y)
                                .mul_add(4.0, random.gen_range(-0.5..=0.5)),
                            f64::from(z)
                                .mul_add(4.0, random.gen_range(-0.5..=0.5)),
                        ),
                        rotation: Rotation3::new(Vector3::new(0.0, 0.0, 0.0)),
                        mesh_scale: Vector3::new(r as f32, r as f32, r as f32),
                        ..Object::new(
                            &self.sphere_mesh,
                            Collider::Sphere(r),
                            r * r * r * 8.0,
                        )
                    });
                }
            }
        }
        self.objects.push(Object {
            immovable: true,
            position: Point3::new(0.0, -1.0, 0.0),
            mesh_scale: Vector3::new(10000.0, 1.0, 10000.0),
            ..Object::new(
                &self.box_mesh,
                Collider::Box(10000.0, 1.0, 10000.0),
                1.0,
            )
        });
    }

    fn preset_two_spheres(&mut self) {
        self.objects.clear();
        self.objects.push(Object {
            position: Point3::new(0.0, 0.0, 0.0),
            rotation: Rotation3::new(Vector3::new(0.0, 0.0, 0.0)),
            mesh_scale: Vector3::new(1.0, 1.0, 1.0),
            ..Object::new(&self.sphere_mesh, Collider::Sphere(1.0), 1.0)
        });
        self.objects.push(Object {
            position: Point3::new(3.0, 0.0, 0.0),
            rotation: Rotation3::new(Vector3::new(0.0, 0.0, 0.0)),
            mesh_scale: Vector3::new(1.0, 1.0, 1.0),
            immovable: true,
            ..Object::new(&self.sphere_mesh, Collider::Sphere(1.0), 1.0)
        });
    }

    fn preset_sphere_and_box(&mut self) {
        self.objects.clear();
        self.objects.push(Object {
            position: Point3::new(0.0, 0.0, 0.0),
            rotation: Rotation3::new(Vector3::new(0.0, 0.0, 0.0)),
            mesh_scale: Vector3::new(1.0, 1.0, 1.0),
            ..Object::new(&self.sphere_mesh, Collider::Sphere(1.0), 1.0)
        });
        self.objects.push(Object {
            position: Point3::new(3.0, 0.0, 0.0),
            rotation: Rotation3::new(Vector3::new(0.0, 0.0, 0.0)),
            mesh_scale: Vector3::new(1.5, 1.5, 1.5),
            ..Object::new(&self.box_mesh, Collider::Box(1.5, 1.5, 1.5), 1.0)
        });
    }

    fn preset_wrecking_ball(&mut self) {
        self.objects.clear();
        for x in -7..=7 {
            for y in -7..=7 {
                for z in 0..10 {
                    self.objects.push(Object {
                        position: Point3::new(
                            f64::from(x),
                            f64::from(y),
                            f64::from(z) * 1.5,
                        ),
                        mesh_scale: Vector3::new(0.5, 0.5, 0.5),
                        ..Object::new(
                            &self.sphere_mesh,
                            Collider::Sphere(0.5),
                            1.0,
                        )
                    });
                }
            }
        }
        self.objects.push(Object {
            position: Point3::new(0.0, 0.0, -30.0),
            mesh_scale: Vector3::new(2.0, 2.0, 2.0),
            momentum: Vector3::new(0.0, 0.0, 450.0),
            // immovable: true,
            ..Object::new(&self.sphere_mesh, Collider::Sphere(2.0), 20.0)
        });
    }

    fn preset_spinning_ball(&mut self) {
        self.objects.clear();
        for x in -5..=5 {
            for y in -5..=5 {
                self.objects.push(Object {
                    position: Point3::new(
                        1.5 * f64::from(x),
                        1.5 * f64::from(y),
                        0.0,
                    ),
                    mesh_scale: Vector3::new(0.5, 0.5, 0.5),
                    ..Object::new(&self.sphere_mesh, Collider::Sphere(0.5), 1.0)
                });
            }
        }
        self.objects.push(Object {
            position: Point3::new(0.0, 0.0, 50.0),
            angular_momentum: Vector3::new(0.0, 5e6, 0.0),
            mesh_scale: Vector3::new(20.0, 20.0, 20.0),
            immovable: true,
            ..Object::new(&self.sphere_mesh, Collider::Sphere(20.0), 100_000.0)
        });
    }

    fn preset_rotating_board(&mut self) {
        self.objects.clear();
        for x in -5..=5 {
            for y in -5..=5 {
                self.objects.push(Object {
                    position: Point3::new(
                        1.5 * f64::from(x),
                        1.5 * f64::from(y),
                        0.0,
                    ),
                    mesh_scale: Vector3::new(0.5, 0.5, 0.5),
                    ..Object::new(&self.sphere_mesh, Collider::Sphere(0.5), 1.0)
                });
            }
        }
        self.objects.push(Object {
            position: Point3::new(0.0, 0.0, 30.0),
            angular_momentum: Vector3::new(0.0, 1e4, 0.0),
            mesh_scale: Vector3::new(40.0, 10.0, 1.0),
            immovable: true,
            ..Object::new(&self.box_mesh, Collider::Box(40.0, 10.0, 1.0), 100.0)
        });
    }

    fn depth_pass(&self, ctx: &mut Context) {
        unsafe {
            ctx.use_shader_program(&self.depth_pass_program);
            let model = ctx
                .gl
                .get_uniform_location(self.depth_pass_program.program, "model")
                .unwrap();
            let view_proj = ctx
                .gl
                .get_uniform_location(
                    self.depth_pass_program.program,
                    "view_proj",
                )
                .unwrap();
            let view_proj_m = self.camera.view_proj();
            ctx.gl.uniform_matrix_4_f32_slice(
                Some(&view_proj),
                false,
                view_proj_m.as_slice(),
            );
            for object in &self.objects {
                let model_m = object.model();
                ctx.gl.uniform_matrix_4_f32_slice(
                    Some(&model),
                    false,
                    model_m.as_slice(),
                );
                ctx.draw_mesh(&object.mesh);
            }
        }
    }

    fn draw_shadow(&self, ctx: &Context, light_space_matrix: &Matrix4<f32>) {
        unsafe {
            ctx.gl.viewport(
                0,
                0,
                shadow_util::SHADOW_WIDTH,
                shadow_util::SHADOW_HEIGHT,
            );
            ctx.gl
                .bind_framebuffer(glow::FRAMEBUFFER, Some(self.shadow_buffer));
            ctx.gl.clear(glow::DEPTH_BUFFER_BIT);
            ctx.use_shader_program(&self.depth_pass_program);
            let model = ctx
                .gl
                .get_uniform_location(self.depth_pass_program.program, "model")
                .unwrap();
            let view_proj = ctx
                .gl
                .get_uniform_location(
                    self.depth_pass_program.program,
                    "view_proj",
                )
                .unwrap();
            ctx.gl.uniform_matrix_4_f32_slice(
                Some(&view_proj),
                false,
                light_space_matrix.as_slice(),
            );
            for object in &self.objects {
                let model_m = object.model();
                ctx.gl.uniform_matrix_4_f32_slice(
                    Some(&model),
                    false,
                    model_m.as_slice(),
                );
                ctx.draw_mesh(&object.mesh);
            }
            ctx.gl.bind_framebuffer(glow::FRAMEBUFFER, None);
            ctx.gl.viewport(
                0,
                0,
                self.surface_width as i32,
                self.surface_height as i32,
            );
        }
    }

    fn draw_phong(&self, ctx: &mut Context, light_space_matrix: &Matrix4<f32>) {
        unsafe {
            ctx.use_shader_program(&self.phong_shader_program);
            let model = ctx.gl.get_uniform_location(
                self.phong_shader_program.program,
                "model",
            );
            let model_inv = ctx.gl.get_uniform_location(
                self.phong_shader_program.program,
                "model_inv",
            );
            let view_proj = ctx.gl.get_uniform_location(
                self.phong_shader_program.program,
                "view_proj",
            );
            let w_eye = ctx.gl.get_uniform_location(
                self.phong_shader_program.program,
                "wEye",
            );
            let w_li_pos = ctx.gl.get_uniform_location(
                self.phong_shader_program.program,
                "wLiPos",
            );
            let light_space = ctx.gl.get_uniform_location(
                self.phong_shader_program.program,
                "light_space_matrix",
            );
            let shadow_map = ctx.gl.get_uniform_location(
                self.phong_shader_program.program,
                "shadow_map",
            );
            ctx.gl.uniform_1_i32(shadow_map.as_ref(), 0);
            ctx.gl.bind_texture(glow::TEXTURE_2D, Some(self.shadow_map));
            ctx.gl.uniform_matrix_4_f32_slice(
                light_space.as_ref(),
                false,
                light_space_matrix.as_slice(),
            );
            let view_proj_m = self.camera.view_proj();
            ctx.gl.uniform_matrix_4_f32_slice(
                view_proj.as_ref(),
                false,
                view_proj_m.as_slice(),
            );
            ctx.gl.uniform_3_f32_slice(
                w_eye.as_ref(),
                self.camera.position().coords.as_slice(),
            );
            ctx.gl.uniform_4_f32_slice(
                w_li_pos.as_ref(),
                self.light_pos.as_slice(),
            );
            for object in &self.objects {
                let model_m = object.model();
                let model_inv_m = model_m.try_inverse().unwrap();
                ctx.gl.uniform_matrix_4_f32_slice(
                    model.as_ref(),
                    false,
                    model_m.as_slice(),
                );
                ctx.gl.uniform_matrix_4_f32_slice(
                    model_inv.as_ref(),
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
            // for object in &self.objects {
            //     let aabb = object.aabb();
            //     let size = aabb.end() - aabb.start();
            //     let pos = aabb.start() + size / 2.0;
            //     let model_m = Translation3::from(pos.cast::<f32>())
            //         .to_homogeneous()
            //         * Scale3::from(size.cast::<f32>()).to_homogeneous();

            //     ctx.gl.uniform_matrix_4_f32_slice(
            //         Some(&model),
            //         false,
            //         model_m.as_slice(),
            //     );
            //     ctx.gl.uniform_3_f32_slice(Some(&color), &[1.0, 0.0, 0.0]);
            //     ctx.draw_mesh(&self.bounding_box_mesh);
            // }
            for (depth, aabb) in self.simulation.rtree.aabbs() {
                if depth < self.max_depth {
                    ctx.gl.line_width((self.max_depth - depth) as f32 + 1.0);
                    let size = aabb.end() - aabb.start();
                    let pos = aabb.start() + size / 2.0;
                    let model_m = Translation3::from(pos.cast::<f32>())
                        .to_homogeneous()
                        * Scale3::from(size.cast::<f32>()).to_homogeneous();

                    ctx.gl.uniform_matrix_4_f32_slice(
                        Some(&model),
                        false,
                        model_m.as_slice(),
                    );
                    ctx.gl.uniform_3_f32_slice(
                        Some(&color),
                        &[
                            (depth % 2) as f32 * 0.75,
                            (depth / 2 % 2) as f32 * 0.75,
                            (depth / 4 % 2) as f32 * 0.75,
                        ],
                    );
                    ctx.draw_mesh(&self.bounding_box_mesh);
                }
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

    fn draw_ui(&mut self, ui: &mut Ui) {
        ui.set_min_width(200.0);
        ui.checkbox(&mut self.paused, "Pause");
        if ui.button("Many spheres").clicked() {
            self.preset_many_spheres();
        }
        if ui.button("Two spheres").clicked() {
            self.preset_two_spheres();
        }
        if ui.button("Sphere and box").clicked() {
            self.preset_sphere_and_box();
        }
        if ui.button("Wrecking ball").clicked() {
            self.preset_wrecking_ball();
        }
        if ui.button("Spinning ball").clicked() {
            self.preset_spinning_ball();
        }
        if ui.button("Rotating board").clicked() {
            self.preset_rotating_board();
        }
        ui.checkbox(&mut self.depth_pass, "Depth pass");
        ui.checkbox(&mut self.draw_phong, "Draw objects");
        ui.checkbox(&mut self.draw_debug, "Draw bounds");
        ui.add(
            DragValue::new(&mut self.max_depth)
                .prefix("Max depth: ")
                .clamp_range(1..=10)
                .speed(0.1),
        );
        ui.add(
            DragValue::new(&mut self.simulation.epsilon)
                .prefix("Collision energy multiplier: ")
                .clamp_range(-1.0..=2.0)
                .speed(0.005),
        );
        if ui.button("Set light direction").clicked() {
            self.light_pos = -self.camera.look_direction().push(0.0);
        }
        if ui.button("Set light position").clicked() {
            self.light_pos = self.camera.position().coords.push(1.0);
        }
        ui.add(
            DragValue::new(&mut self.simulation.mu)
                .prefix("Coefficient of friction: ")
                .clamp_range(-1.0..=2.0)
                .speed(0.005),
        );
        let total_momentum = self
            .objects
            .iter()
            .map(|o| o.momentum)
            .sum::<Vector3<f64>>();
        ui.label(format!("Total momentum: {}", total_momentum.magnitude()));
        let total_directional_energy = self
            .objects
            .iter()
            .map(|o| o.momentum.magnitude_squared() / o.mass / 2.0)
            .sum::<f64>();
        let total_rotational_energy = self
            .objects
            .iter()
            .map(|o| {
                (o.angular_momentum.transpose()
                    * o.inverse_inertia()
                    * o.angular_momentum)
                    .magnitude()
                    / 2.0
            })
            .sum::<f64>();
        ui.label(format!(
            "Total directional energy: {total_directional_energy}"
        ));
        ui.label(format!(
            "Total rotational energy: {total_rotational_energy}"
        ));
        ui.label(format!(
            "Total energy: {}",
            total_directional_energy + total_rotational_energy
        ));
    }
}

impl Scene for MainScene {
    fn draw(&mut self, ctx: &mut Context, delta: f64) {
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
            ctx.gl.disable(glow::BLEND);
            if self.depth_pass {
                self.depth_pass(ctx);
                ctx.gl.depth_func(glow::LEQUAL);
            } else {
                ctx.gl.depth_func(glow::LESS);
            }
            let light_proj =
                Orthographic3::new(-50.0, 50.0, -50.0, 50.0, 1.0, 500.0)
                    .to_homogeneous();
            let light_dir = (-self.light_pos.xyz()).normalize();
            let light_look_at =
                self.camera.position() + 50.0 * self.camera.look_direction();
            let light_position = light_look_at - 50.0 * light_dir;
            let light_view = Matrix4::look_at_rh(
                &light_position,
                &light_look_at,
                &Vector3::new(0.0, 1.0, 0.0),
            );
            let light_space_matrix = light_proj * light_view;
            self.draw_shadow(ctx, &light_space_matrix);
            if self.draw_phong {
                self.draw_phong(ctx, &light_space_matrix);
            }
            ctx.gl.depth_func(glow::LESS);
            if self.draw_debug {
                self.draw_debug(ctx);
            }
            if self.camera.focus() {
                self.draw_hud(ctx);
            }
            ctx.egui.run(&ctx.window, |egui_ctx| {
                Window::new("Debug").show(egui_ctx, |ui| {
                    ui.label(format!("FPS: {:2.2}", 1.0 / delta));
                    self.draw_ui(ui);
                });
            });
            ctx.egui.paint(&ctx.window);
            ctx.gl_surface.swap_buffers(&ctx.gl_context).unwrap();
        }
    }

    fn update(&mut self, delta: f64) {
        const TICK_RATE_TARGET: f64 = 100.0;
        const MAX_STEP_COUNT: u32 = 10;
        self.camera.update(delta as f32);
        if !self.paused {
            #[allow(clippy::cast_sign_loss)]
            let step_count = MAX_STEP_COUNT
                .min((delta * TICK_RATE_TARGET).abs().ceil() as u32);
            let step_size = delta / f64::from(step_count);
            for _ in 0..step_count {
                self.simulation.simulate(&mut self.objects, step_size);
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
                        );
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
