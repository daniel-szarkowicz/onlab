use core::fmt;
use std::{error::Error, marker::PhantomData};

use glow::{HasContext, NativeVertexArray};

use crate::{vertex::Vertex, Context};

#[derive(Clone, Copy, Debug)]
#[allow(clippy::cast_possible_wrap)]
pub enum MeshPrimitive {
    Points = glow::POINTS as isize,
    LineSprip = glow::LINE_STRIP as isize,
    LineLoop = glow::LINE_LOOP as isize,
    Lines = glow::LINES as isize,
    LineStripAdjacency = glow::LINE_STRIP_ADJACENCY as isize,
    LinesAdjacency = glow::LINES_ADJACENCY as isize,
    TriangleStrip = glow::TRIANGLE_STRIP as isize,
    TriangleFan = glow::TRIANGLE_FAN as isize,
    Triangles = glow::TRIANGLES as isize,
    TriangleStripAdjacency = glow::TRIANGLE_STRIP_ADJACENCY as isize,
    TrianglesAdjacency = glow::TRIANGLES_ADJACENCY as isize,
}

#[derive(Debug)]
pub struct Mesh<V: Vertex> {
    vertex_array: NativeVertexArray,
    count: i32,
    primitive: MeshPrimitive,
    phantom: PhantomData<V>,
}

impl<V: Vertex> Mesh<V> {
    pub fn new(
        ctx: &Context,
        vertices: &[V],
        indices: &[u16],
        primitive: MeshPrimitive,
    ) -> anyhow::Result<Self> {
        for i in indices {
            if *i as usize >= vertices.len() {
                Err(MeshError(format!(
                    "Indicies out of bounds. Index: {i}/{}",
                    vertices.len()
                )))?;
            }
        }
        let vertex_array = unsafe {
            let gl = &ctx.gl;
            let vertex_array = gl.create_vertex_array().map_err(MeshError)?;
            let vertex_buffer = gl.create_buffer().map_err(MeshError)?;
            let index_buffer = gl.create_buffer().map_err(MeshError)?;
            gl.bind_vertex_array(Some(vertex_array));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(vertices),
                glow::STATIC_DRAW,
            );
            V::set_layout(gl);
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(index_buffer));
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(indices),
                glow::STATIC_DRAW,
            );
            gl.bind_vertex_array(None);
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
            vertex_array
        };
        Ok(Self {
            vertex_array,
            count: indices.len().try_into()?,
            primitive,
            phantom: PhantomData,
        })
    }
}

#[derive(Debug)]
struct MeshError(String);

impl fmt::Display for MeshError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for MeshError {}

pub trait DrawMesh {
    /// # Safety
    /// Should only be called if a shader with matching vertex layout is being
    /// used.
    unsafe fn draw_mesh<V: Vertex>(&self, mesh: &Mesh<V>);
}

impl DrawMesh for Context {
    unsafe fn draw_mesh<V: Vertex>(&self, mesh: &Mesh<V>) {
        unsafe {
            self.gl.bind_vertex_array(Some(mesh.vertex_array));
            self.gl.draw_elements(
                mesh.primitive as u32,
                mesh.count,
                glow::UNSIGNED_SHORT,
                0,
            );
            self.gl.bind_vertex_array(None);
        }
    }
}
