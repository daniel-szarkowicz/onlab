use glow::{HasContext, NativeFramebuffer, NativeTexture};
use nalgebra::{Matrix4, Orthographic3, Rotation3, Vector3, Vector4};

const UP: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);

use crate::{
    mesh::DrawMesh,
    object::Object,
    render_state::{RenderState, SetUniform},
};

pub const SHADOW_WIDTH: i32 = 2048;
pub const SHADOW_HEIGHT: i32 = SHADOW_WIDTH;

#[derive(Debug, Clone)]
pub struct DirectionalLight {
    direction: Vector3<f32>,
    shadow_buffer: NativeFramebuffer,
    shadow_map: NativeTexture,
    ambient_color: [f32; 3],
    emissive_color: [f32; 3],
    view_proj: Matrix4<f32>,
}

impl DirectionalLight {
    pub fn new(gl: &glow::Context) -> Self {
        #[allow(clippy::cast_possible_wrap)]
        let (shadow_buffer, shadow_map) = unsafe {
            let shadow_buffer = gl.create_framebuffer().unwrap();
            let shadow_map = gl.create_texture().unwrap();
            gl.bind_texture(glow::TEXTURE_2D, Some(shadow_map));
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::DEPTH_COMPONENT as i32,
                SHADOW_WIDTH,
                SHADOW_HEIGHT,
                0,
                glow::DEPTH_COMPONENT,
                glow::FLOAT,
                None,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::NEAREST as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::NEAREST as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_S,
                glow::CLAMP_TO_BORDER as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_T,
                glow::CLAMP_TO_BORDER as i32,
            );
            let border_color = [1.0, 1.0, 1.0, 1.0];
            gl.tex_parameter_f32_slice(
                glow::TEXTURE_2D,
                glow::TEXTURE_BORDER_COLOR,
                &border_color,
            );
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(shadow_buffer));
            gl.framebuffer_texture_2d(
                glow::FRAMEBUFFER,
                glow::DEPTH_ATTACHMENT,
                glow::TEXTURE_2D,
                Some(shadow_map),
                0,
            );
            gl.draw_buffer(glow::NONE);
            gl.read_buffer(glow::NONE);
            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
            (shadow_buffer, shadow_map)
        };
        Self {
            direction: Vector3::new(-1.0, -1.0, 1.0),
            shadow_buffer,
            shadow_map,
            ambient_color: [0.3; 3],
            emissive_color: [0.7; 3],
            view_proj: Matrix4::identity(),
        }
    }

    pub fn update(&mut self, camera_inverse: &Matrix4<f32>) {
        let camera_frustum_points = [
            Vector4::new(1.0, 1.0, 1.0, 1.0),
            Vector4::new(1.0, 1.0, -1.0, 1.0),
            Vector4::new(1.0, -1.0, 1.0, 1.0),
            Vector4::new(1.0, -1.0, -1.0, 1.0),
            Vector4::new(-1.0, 1.0, 1.0, 1.0),
            Vector4::new(-1.0, 1.0, -1.0, 1.0),
            Vector4::new(-1.0, -1.0, 1.0, 1.0),
            Vector4::new(-1.0, -1.0, -1.0, 1.0),
        ]
        .map(|v| {
            let p = camera_inverse * v;
            p.xyz() / p.w
        });
        let left = self.direction.cross(&UP).normalize();
        let up = left.cross(&self.direction).normalize();
        let minmax = |dir: &Vector3<f32>| {
            camera_frustum_points
                .iter()
                .map(|v| dir.dot(v))
                .fold((f32::MAX, f32::MIN), |(min, max), elem| {
                    (min.min(elem), max.max(elem))
                })
        };
        let (x_min, x_max) = minmax(&left);
        let (y_min, y_max) = minmax(&up);
        let (z_min, z_max) = minmax(&self.direction);
        let proj = Orthographic3::new(
            x_min,
            x_max,
            y_min,
            y_max,
            z_min.mul_add(2.0, -z_max),
            z_max,
        )
        .to_homogeneous();
        let view = Rotation3::look_at_rh(&self.direction, &up).to_homogeneous();
        self.view_proj = proj * view;
    }

    #[must_use]
    pub const fn view_proj(&self) -> &Matrix4<f32> {
        &self.view_proj
    }

    pub fn set_direction(&mut self, direction: &Vector3<f32>) {
        self.direction = direction.normalize();
    }

    #[must_use]
    pub const fn ambient_color(&self) -> &[f32; 3] {
        &self.ambient_color
    }

    #[must_use]
    pub fn ambient_color_mut(&mut self) -> &mut [f32; 3] {
        &mut self.ambient_color
    }

    #[must_use]
    pub const fn emissive_color(&self) -> &[f32; 3] {
        &self.emissive_color
    }

    #[must_use]
    pub fn emissive_color_mut(&mut self) -> &mut [f32; 3] {
        &mut self.emissive_color
    }

    #[must_use]
    pub const fn direction(&self) -> &Vector3<f32> {
        &self.direction
    }

    /// # Safety
    /// The texture should not be written to, the texture should be bound
    /// as `glow::TEXTURE_2D`.
    #[deprecated]
    #[must_use]
    pub const unsafe fn native_texture(&self) -> &NativeTexture {
        &self.shadow_map
    }

    /// # Safety
    /// The correct shader should be bound.
    /// The render state should be cleaned up after this function
    pub unsafe fn render_shadows<'a>(
        &mut self,
        render_state: &mut RenderState,
        objects: impl IntoIterator<Item = &'a Object>,
    ) {
        render_state.set_viewport(0, 0, SHADOW_WIDTH, SHADOW_HEIGHT);
        render_state.set_framebuffer(self.shadow_buffer);
        render_state.set_cull_face(glow::FRONT);
        unsafe { render_state.gl().clear(glow::DEPTH_BUFFER_BIT) };
        // render_state.set_program <- this was already done by the caller
        render_state.set_uniform("view_proj", &self.view_proj);
        for o in objects {
            render_state.set_uniform("model", &o.model());
            unsafe { render_state.draw_mesh(&o.mesh) };
        }
    }
}