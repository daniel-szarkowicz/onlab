use bytemuck::{Pod, Zeroable};
use glow::HasContext;

pub unsafe trait Vertex: Pod {
    unsafe fn set_layout(gl: &glow::Context);
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct PNVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

unsafe impl Vertex for PNVertex {
    /// Should only be called when a glow::NativeVertexArray with matching
    /// vertex layout is bound.
    unsafe fn set_layout(gl: &glow::Context) {
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(
            0,
            3,
            glow::FLOAT,
            false,
            std::mem::size_of::<PNVertex>() as i32,
            0,
        );
        gl.enable_vertex_attrib_array(1);
        gl.vertex_attrib_pointer_f32(
            1,
            3,
            glow::FLOAT,
            false,
            2 * std::mem::size_of::<PNVertex>() as i32,
            std::mem::size_of::<[f32; 3]>() as i32,
        );
    }
}
