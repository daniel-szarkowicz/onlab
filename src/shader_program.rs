use core::fmt;
use std::{error::Error, fs, marker::PhantomData, path::Path};

use glow::{HasContext, NativeProgram};

use anyhow::Result;

use crate::{vertex::Vertex, Context};

#[derive(Debug)]
pub struct ShaderProgram<V: Vertex> {
    // HACK make in private
    pub program: NativeProgram,
    phantom: PhantomData<V>,
}

unsafe fn load_shader(
    gl: &glow::Context,
    file: impl AsRef<Path>,
    typ: u32,
) -> Result<glow::NativeShader> {
    unsafe {
        let shader = gl.create_shader(typ).map_err(ShaderProgramError)?;
        let f_name = file.as_ref().to_string_lossy().to_string();
        gl.shader_source(shader, &fs::read_to_string(file)?);
        gl.compile_shader(shader);
        if !gl.get_shader_compile_status(shader) {
            Err(ShaderProgramError(format!(
                "Shader `{f_name}` was not compiled:\n{}",
                gl.get_shader_info_log(shader)
            )))?;
        }
        Ok(shader)
    }
}

impl<V: Vertex> ShaderProgram<V> {
    pub fn new(
        ctx: &Context,
        vertex_file: impl AsRef<Path>,
        fragment_file: impl AsRef<Path>,
    ) -> Result<Self> {
        let program = unsafe {
            let gl = &ctx.gl;
            let vertex = load_shader(gl, vertex_file, glow::VERTEX_SHADER)?;
            let fragment =
                load_shader(gl, fragment_file, glow::FRAGMENT_SHADER)?;

            let program = gl.create_program().map_err(ShaderProgramError)?;
            gl.attach_shader(program, vertex);
            gl.attach_shader(program, fragment);
            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                Err(ShaderProgramError(format!(
                    "Shader program was not linked:\n{}",
                    gl.get_program_info_log(program)
                )))?;
            }
            V::validate_layout(gl, program)?;
            program
        };
        Ok(Self {
            program,
            phantom: PhantomData,
        })
    }
}

#[derive(Debug)]
struct ShaderProgramError(String);

impl fmt::Display for ShaderProgramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ShaderProgramError {}

pub trait UseShaderProgram {
    /// # Safety
    /// All of the shader's uniforms should be set before calling a `draw`
    /// funciton.
    unsafe fn use_shader_program<V: Vertex>(&self, program: &ShaderProgram<V>);
}

impl UseShaderProgram for Context {
    unsafe fn use_shader_program<V: Vertex>(&self, program: &ShaderProgram<V>) {
        unsafe {
            self.gl.use_program(Some(program.program));
        }
    }
}
