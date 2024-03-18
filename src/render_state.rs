use std::sync::Arc;

use glow::{
    HasContext, NativeFramebuffer, NativeProgram, NativeTexture,
    NativeUniformLocation, NativeVertexArray,
};
use nalgebra::{Matrix4, Vector3, Vector4};

use crate::{shader_program::ShaderProgram, vertex::Vertex};

#[allow(missing_debug_implementations)]
pub struct RenderState {
    gl: Arc<glow::Context>,
    viewport: [i32; 4],
    framebuffer: Option<NativeFramebuffer>,
    cull_face: u32,
    line_width: f32,
    program: Option<NativeProgram>,
    vertex_array: Option<NativeVertexArray>,
}

impl RenderState {
    pub fn new(gl: Arc<glow::Context>) -> Self {
        let this = Self {
            gl,
            viewport: [0, 0, 100, 100],
            framebuffer: None,
            cull_face: glow::BACK,
            line_width: 1.0,
            program: None,
            vertex_array: None,
        };
        unsafe {
            let [x, y, w, h] = this.viewport;
            this.gl.viewport(x, y, w, h);
            this.gl
                .bind_framebuffer(glow::FRAMEBUFFER, this.framebuffer);
            this.gl.cull_face(this.cull_face);
            this.gl.line_width(this.line_width);
            this.gl.use_program(this.program);
            this.gl.bind_vertex_array(this.vertex_array);
        }
        this
    }

    #[must_use]
    pub fn get_uniform_location(
        &self,
        name: &str,
    ) -> Option<NativeUniformLocation> {
        unsafe {
            self.program.map_or_else(
                || {
                    eprintln!("No shader bound");
                    None
                },
                |program| {
                    self.gl.get_uniform_location(program, name).or_else(|| {
                        eprintln!("No uniform with name {name}");
                        None
                    })
                },
            )
        }
    }

    pub fn set_program<V: Vertex>(&mut self, program: &ShaderProgram<V>) {
        // if self.program != Some(program.program) {
        self.program = Some(program.program);
        unsafe { self.gl.use_program(self.program) };
        // }
    }

    pub fn set_vertex_array(&mut self, va: NativeVertexArray) {
        // if self.vertex_array != Some(va) {
        self.vertex_array = Some(va);
        unsafe { self.gl.bind_vertex_array(self.vertex_array) };
        // }
    }

    pub fn set_viewport(&mut self, x: i32, y: i32, w: i32, h: i32) {
        let vp = [x, y, w, h];
        // if vp != self.viewport {
        self.viewport = vp;
        unsafe { self.gl.viewport(x, y, w, h) };
        // }
    }

    pub fn set_framebuffer(&mut self, fb: NativeFramebuffer) {
        // if self.framebuffer != Some(fb) {
        self.framebuffer = Some(fb);
        unsafe {
            self.gl
                .bind_framebuffer(glow::FRAMEBUFFER, self.framebuffer);
        }
        // }
    }

    pub fn unset_framebuffer(&mut self) {
        // if self.framebuffer.is_some() {
        self.framebuffer = None;
        unsafe {
            self.gl
                .bind_framebuffer(glow::FRAMEBUFFER, self.framebuffer);
        }
        // }
    }

    pub fn set_cull_face(&mut self, face: u32) {
        // if self.cull_face != face {
        self.cull_face = face;
        unsafe { self.gl.cull_face(face) };
        // }
    }

    pub fn set_line_width(&mut self, size: f32) {
        // #[allow(clippy::float_cmp)]
        // if self.line_width != size {
        self.line_width = size;
        unsafe { self.gl.line_width(self.line_width) };
        // }
    }

    /// # Safety
    /// `texture_index` should be different for every texture used in the
    /// same draw call.
    /// `texture` should be backed by a `glow::TEXTURE_2D_ARRAY`.
    #[deprecated = "TODO: implement proper texture handling"]
    pub unsafe fn set_texture_2d_array_uniform(
        &mut self,
        name: &str,
        texture_index: u32,
        texture: NativeTexture,
    ) {
        assert!(texture_index < 16);
        unsafe {
            self.gl.active_texture(glow::TEXTURE0 + texture_index);
            self.gl.bind_texture(glow::TEXTURE_2D_ARRAY, Some(texture));
            #[allow(clippy::cast_possible_wrap)]
            self.set_uniform(name, &(texture_index as i32));
        }
    }

    /// # Safety
    /// Changing the state of the GL context should always be reflected
    /// in `RenderState`
    #[must_use]
    pub unsafe fn gl(&self) -> &glow::Context {
        &self.gl
    }
}

pub trait SetUniform<T> {
    fn set_uniform(&mut self, name: &str, data: &T);
}

impl SetUniform<Vector3<f32>> for RenderState {
    fn set_uniform(&mut self, name: &str, data: &Vector3<f32>) {
        unsafe {
            self.gl.uniform_3_f32_slice(
                self.get_uniform_location(name).as_ref(),
                data.as_slice(),
            );
        }
    }
}

impl SetUniform<Vector4<f32>> for RenderState {
    fn set_uniform(&mut self, name: &str, data: &Vector4<f32>) {
        unsafe {
            self.gl.uniform_4_f32_slice(
                self.get_uniform_location(name).as_ref(),
                data.as_slice(),
            );
        }
    }
}

impl SetUniform<Matrix4<f32>> for RenderState {
    fn set_uniform(&mut self, name: &str, data: &Matrix4<f32>) {
        unsafe {
            self.gl.uniform_matrix_4_f32_slice(
                self.get_uniform_location(name).as_ref(),
                false,
                data.as_slice(),
            );
        }
    }
}

impl SetUniform<i32> for RenderState {
    fn set_uniform(&mut self, name: &str, data: &i32) {
        unsafe {
            self.gl
                .uniform_1_i32(self.get_uniform_location(name).as_ref(), *data);
        }
    }
}

impl SetUniform<[f32; 3]> for RenderState {
    fn set_uniform(&mut self, name: &str, data: &[f32; 3]) {
        unsafe {
            self.gl.uniform_3_f32_slice(
                self.get_uniform_location(name).as_ref(),
                data,
            );
        }
    }
}

impl SetUniform<u32> for RenderState {
    fn set_uniform(&mut self, name: &str, data: &u32) {
        unsafe {
            self.gl
                .uniform_1_u32(self.get_uniform_location(name).as_ref(), *data);
        }
    }
}
