use core::fmt;
use std::{error::Error, marker::PhantomData};

use glow::{HasContext, NativeVertexArray};

use crate::{vertex::Vertex, Context};

pub struct VertexArray<V: Vertex> {
    vao: NativeVertexArray,
    count: i32,
    phantom: PhantomData<V>,
}

impl<V: Vertex> VertexArray<V> {
    pub fn new(ctx: &Context, vertices: &[V]) -> anyhow::Result<Self> {
        let vao = unsafe {
            let gl = &ctx.gl;
            let vao = gl.create_vertex_array().map_err(VertexArrayError)?;
            let vbo = gl.create_buffer().map_err(VertexArrayError)?;
            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(vertices),
                glow::STATIC_DRAW,
            );
            V::set_layout(gl);
            gl.bind_vertex_array(None);
            vao
        };
        Ok(Self {
            vao,
            count: vertices.len() as i32,
            phantom: PhantomData,
        })
    }
}

#[derive(Debug)]
struct VertexArrayError(String);

impl fmt::Display for VertexArrayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for VertexArrayError {}

pub trait DrawVertexArray {
    /// Should only be called if a shader with matching vertex layout is being
    /// used.
    unsafe fn draw_vertex_array<V: Vertex>(
        &self,
        mode: u32,
        vertex_array: &VertexArray<V>,
    );

    /// Should only be called if a shader with matching vertex layout is being
    /// used.
    unsafe fn draw_triangles<V: Vertex>(&self, vertex_array: &VertexArray<V>) {
        self.draw_vertex_array(glow::TRIANGLES, vertex_array);
    }
}

impl DrawVertexArray for Context {
    unsafe fn draw_vertex_array<V: Vertex>(
        &self,
        mode: u32,
        vertex_array: &VertexArray<V>,
    ) {
        self.gl.bind_vertex_array(Some(vertex_array.vao));
        self.gl.draw_arrays(mode, 0, vertex_array.count);
        self.gl.bind_vertex_array(None);
    }
}

// native vertex array = &[u8] + layout
// my vertex array = &[impl Vertex] + Vertex::set_layout()
