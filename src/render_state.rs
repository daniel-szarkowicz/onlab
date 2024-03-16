use std::sync::Arc;

use glow::{
    HasContext, NativeFramebuffer, NativeProgram, NativeUniformLocation,
};
use nalgebra::{Matrix4, Vector3};

use crate::{shader_program::ShaderProgram, vertex::Vertex};

#[allow(missing_debug_implementations)]
pub struct RenderState {
    gl: Arc<glow::Context>,
    viewport: [i32; 4],
    framebuffer: Option<NativeFramebuffer>,
    cull_face: u32,
    program: Option<NativeProgram>,
}

impl RenderState {
    pub fn new(gl: Arc<glow::Context>) -> Self {
        let this = Self {
            gl,
            viewport: [0, 0, 100, 100],
            framebuffer: None,
            cull_face: glow::BACK,
            program: None,
        };
        unsafe {
            let [x, y, w, h] = this.viewport;
            this.gl.viewport(x, y, w, h);
            this.gl
                .bind_framebuffer(glow::FRAMEBUFFER, this.framebuffer);
            this.gl.cull_face(this.cull_face);
            this.gl.use_program(this.program);
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
                    eprintln!("{}:{} No shader bound", file!(), line!());
                    None
                },
                |program| self.gl.get_uniform_location(program, name),
            )
        }
    }

    pub fn set_program<V: Vertex>(&mut self, program: &ShaderProgram<V>) {
        // if self.program != Some(program.program) {
        self.program = Some(program.program);
        unsafe { self.gl.use_program(self.program) };
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
