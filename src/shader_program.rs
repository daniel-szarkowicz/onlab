use core::fmt;
use std::{error::Error, fs, marker::PhantomData, path::Path};

use glow::{HasContext, NativeProgram};

use anyhow::Result;

use crate::{vertex::Vertex, Context};

pub struct ShaderProgram<V: Vertex> {
    // HACK make in private
    pub program: NativeProgram,
    phantom: PhantomData<V>,
}

impl<V: Vertex> ShaderProgram<V> {
    pub fn new(
        ctx: &Context,
        vertex_file: impl AsRef<Path>,
        fragment_file: impl AsRef<Path>,
    ) -> Result<Self> {
        let program = unsafe {
            let gl = &ctx.gl;
            let program = gl.create_program().map_err(ShaderProgramError)?;
            let vertex = gl
                .create_shader(glow::VERTEX_SHADER)
                .map_err(ShaderProgramError)?;
            gl.shader_source(vertex, &fs::read_to_string(vertex_file)?);
            gl.compile_shader(vertex);
            gl.attach_shader(program, vertex);

            let fragment = gl
                .create_shader(glow::FRAGMENT_SHADER)
                .map_err(ShaderProgramError)?;
            gl.shader_source(fragment, &fs::read_to_string(fragment_file)?);
            gl.compile_shader(fragment);
            gl.attach_shader(program, fragment);
            gl.link_program(program);
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
    unsafe fn use_shader_program<V: Vertex>(&self, program: &ShaderProgram<V>);
}

impl UseShaderProgram for Context {
    unsafe fn use_shader_program<V: Vertex>(&self, program: &ShaderProgram<V>) {
        self.gl.use_program(Some(program.program))
    }
}
