use core::fmt;
use std::{error::Error, marker::PhantomData};

use glow::{HasContext, NativeVertexArray};

use crate::{vertex::Vertex, Context};

#[derive(Clone, Copy)]
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

pub struct Mesh<V: Vertex> {
    vao: NativeVertexArray,
    count: i32,
    primitive: MeshPrimitive,
    phantom: PhantomData<V>,
}

impl<V: Vertex> Mesh<V> {
    pub fn new(
        ctx: &Context,
        vertices: &[V],
        indicies: &[u16],
        primitive: MeshPrimitive,
    ) -> anyhow::Result<Self> {
        if indicies.iter().any(|&i| i >= vertices.len() as u16) {
            Err(MeshError("indicies out of bounds".to_string()))?;
        }
        let vao = unsafe {
            let gl = &ctx.gl;
            let vao = gl.create_vertex_array().map_err(MeshError)?;
            let vbo = gl.create_buffer().map_err(MeshError)?;
            let ibo = gl.create_buffer().map_err(MeshError)?;
            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(vertices),
                glow::STATIC_DRAW,
            );
            V::set_layout(gl);
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ibo));
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(indicies),
                glow::STATIC_DRAW,
            );
            // // FIXME uncommenting this causes a segfault.
            // // The index buffer should be unbound,
            // // so it won't be changed accidentally.
            // gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
            gl.bind_vertex_array(None);
            vao
        };
        Ok(Self {
            vao,
            count: indicies.len() as i32,
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
    /// Should only be called if a shader with matching vertex layout is being
    /// used.
    unsafe fn draw_mesh<V: Vertex>(&self, mesh: &Mesh<V>);
}

impl DrawMesh for Context {
    unsafe fn draw_mesh<V: Vertex>(&self, mesh: &Mesh<V>) {
        self.gl.bind_vertex_array(Some(mesh.vao));
        self.gl.draw_elements(
            mesh.primitive as u32,
            mesh.count,
            glow::UNSIGNED_SHORT,
            0,
        );
        self.gl.bind_vertex_array(None);
    }
}
