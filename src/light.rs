use glow::{HasContext, NativeFramebuffer, NativeTexture};
use nalgebra::{Matrix4, Orthographic3, Rotation3, Vector3, Vector4};

const UP: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);

use crate::{
    camera::FirstPersonCamera,
    mesh::DrawMesh,
    object::Object,
    render_state::{RenderState, SetUniform},
};

pub const SHADOW_WIDTH: i32 = 2048;
pub const SHADOW_HEIGHT: i32 = SHADOW_WIDTH;
pub const SHADOW_LAYERS: i32 = 3;
#[allow(clippy::assertions_on_constants)]
const _: () = assert!(
    SHADOW_LAYERS <= 4,
    "current shaders only support up to 8 shadow layers"
);
pub const LAYER_SPLITS: [f32; SHADOW_LAYERS as usize + 1] =
    [0.0, 0.05, 0.2, 0.5];

#[derive(Debug)]
pub struct DirectionalLight {
    direction: Vector3<f32>,
    shadow_buffer: NativeFramebuffer,
    shadow_map: NativeTexture,
    ambient_color: [f32; 3],
    emissive_color: [f32; 3],
    view_projs: [Matrix4<f32>; SHADOW_LAYERS as usize],
}

impl DirectionalLight {
    pub fn new(gl: &glow::Context) -> Self {
        #[allow(clippy::cast_possible_wrap)]
        let (shadow_buffer, shadow_map) = unsafe {
            let shadow_buffer = gl.create_framebuffer().unwrap();
            let depth_map = gl.create_texture().unwrap();
            gl.bind_texture(glow::TEXTURE_2D_ARRAY, Some(depth_map));
            gl.tex_image_3d(
                glow::TEXTURE_2D_ARRAY,
                0,
                glow::DEPTH_COMPONENT32F as i32,
                SHADOW_WIDTH,
                SHADOW_HEIGHT,
                SHADOW_LAYERS,
                0,
                glow::DEPTH_COMPONENT,
                glow::FLOAT,
                None,
            );
            let shadow_map = gl.create_texture().unwrap();
            gl.bind_texture(glow::TEXTURE_2D_ARRAY, Some(shadow_map));
            gl.tex_image_3d(
                glow::TEXTURE_2D_ARRAY,
                0,
                glow::R32F as i32,
                SHADOW_WIDTH,
                SHADOW_HEIGHT,
                SHADOW_LAYERS,
                0,
                glow::RED,
                glow::FLOAT,
                None,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D_ARRAY,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D_ARRAY,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D_ARRAY,
                glow::TEXTURE_WRAP_S,
                glow::CLAMP_TO_BORDER as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D_ARRAY,
                glow::TEXTURE_WRAP_T,
                glow::CLAMP_TO_BORDER as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D_ARRAY,
                glow::TEXTURE_COMPARE_MODE,
                glow::COMPARE_REF_TO_TEXTURE as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D_ARRAY,
                glow::TEXTURE_COMPARE_FUNC,
                glow::GEQUAL as i32,
            );
            let border_color = [1.0, 1.0, 1.0, 1.0];
            gl.tex_parameter_f32_slice(
                glow::TEXTURE_2D_ARRAY,
                glow::TEXTURE_BORDER_COLOR,
                &border_color,
            );
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(shadow_buffer));
            gl.framebuffer_texture(
                glow::FRAMEBUFFER,
                glow::DEPTH_ATTACHMENT,
                Some(depth_map),
                0,
            );
            gl.framebuffer_texture(
                glow::FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                Some(shadow_map),
                0,
            );
            gl.draw_buffer(glow::COLOR_ATTACHMENT0);
            gl.read_buffer(glow::NONE);
            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
            (shadow_buffer, shadow_map)
        };
        Self {
            direction: Vector3::new(-1.0, -1.0, 1.0).normalize(),
            shadow_buffer,
            shadow_map,
            ambient_color: [0.3; 3],
            emissive_color: [0.7; 3],
            view_projs: [Matrix4::identity(); SHADOW_LAYERS as usize],
        }
    }

    pub fn update(&mut self, camera: &FirstPersonCamera) {
        let left = self.direction.cross(&UP).normalize();
        let up = left.cross(&self.direction).normalize();
        let view = Rotation3::look_at_rh(&self.direction, &up).to_homogeneous();
        for (i, start_end) in LAYER_SPLITS.windows(2).enumerate() {
            let start = start_end[0];
            let end = start_end[1];
            let camera_inverse =
                camera.partial_view_proj(start, end).try_inverse().unwrap();
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
            let proj =
                Orthographic3::new(x_min, x_max, y_min, y_max, z_min, z_max)
                    .to_homogeneous();
            self.view_projs[i] = proj * view;
        }
    }

    #[must_use]
    pub const fn view_projs(&self) -> &[Matrix4<f32>] {
        &self.view_projs
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
    /// as `glow::TEXTURE_2D_ARRAY`.
    #[deprecated]
    #[must_use]
    pub const unsafe fn native_texture(&self) -> &NativeTexture {
        &self.shadow_map
    }

    #[deprecated]
    #[must_use]
    pub const fn native_framebuffer(&self) -> &NativeFramebuffer {
        &self.shadow_buffer
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
        unsafe {
            render_state.gl().enable(glow::DEPTH_CLAMP);
        }
        unsafe {
            render_state.gl().clear(glow::DEPTH_BUFFER_BIT);
        }
        // render_state.set_program <- this was already done by the caller
        render_state.set_uniform("layer_count", &(SHADOW_LAYERS as u32));
        for (i, view_proj) in self.view_projs.iter().enumerate() {
            render_state.set_uniform(&format!("view_projs[{i}]"), view_proj);
        }
        for o in objects {
            render_state.set_uniform("model", &o.model());
            unsafe { render_state.draw_mesh(&o.mesh) };
        }
        unsafe {
            render_state.gl().disable(glow::DEPTH_CLAMP);
        }
    }
}
