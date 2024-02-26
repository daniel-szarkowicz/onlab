use bytemuck::{Pod, Zeroable};
use glow::{HasContext, NativeProgram};
use std::{cmp::Ordering, error::Error, fmt};

/// # Safety
/// These vertices will be uploaded byte-for-byte to the GPU,
/// sometimes padding will be neccesary to conform with OpenGL's alignment.
///
/// The `set_layout` and `validate_layout` functions should match the layout
/// of the struct. The `validate_layout` function can accept layouts that
/// do not exactly match the layout of the struct, for example accepting
/// a vec4 instead of a vec3 for a position vector.
pub unsafe trait Vertex: Pod {
    /// # Safety
    /// Should only be called when a `glow::NativeVertexArray` with matching
    /// vertex layout is bound.
    unsafe fn set_layout(gl: &glow::Context);

    /// Should only be called after the shader was linked
    fn validate_layout(
        gl: &glow::Context,
        program: NativeProgram,
    ) -> Result<(), ShaderValidationError>;
}

#[derive(Debug)]
pub enum ShaderValidationError {
    TooFewAttributes,
    TooManyAttributes,
    TypeMismatch { location: u32, attr_name: String },
}

impl fmt::Display for ShaderValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ShaderValidationError::{
            TooFewAttributes, TooManyAttributes, TypeMismatch,
        };
        match self {
            TooFewAttributes => write!(f, "Shader has too few attibutes!"),
            TooManyAttributes => write!(f, "Shader has too many attributes!"),
            TypeMismatch {
                location,
                attr_name,
            } => write!(
                f,
                "Shader attribute `{attr_name}` has the wrong type (location={location})",
            ),
        }
    }
}

impl Error for ShaderValidationError {}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct PNVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

unsafe impl Vertex for PNVertex {
    #[allow(clippy::cast_possible_wrap)]
    unsafe fn set_layout(gl: &glow::Context) {
        unsafe {
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(
                0,
                3,
                glow::FLOAT,
                false,
                std::mem::size_of::<Self>() as i32,
                0,
            );
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(
                1,
                3,
                glow::FLOAT,
                false,
                std::mem::size_of::<Self>() as i32,
                std::mem::size_of::<[f32; 3]>() as i32,
            );
        }
    }

    fn validate_layout(
        gl: &glow::Context,
        program: NativeProgram,
    ) -> Result<(), ShaderValidationError> {
        use ShaderValidationError::{
            TooFewAttributes, TooManyAttributes, TypeMismatch,
        };
        unsafe {
            let attr_count = gl.get_active_attributes(program);
            match attr_count.cmp(&2) {
                Ordering::Less => Err(TooFewAttributes)?,
                Ordering::Greater => Err(TooManyAttributes)?,
                Ordering::Equal => {}
            }
            let attr = gl
                .get_active_attribute(program, 0)
                .expect("Attribute cannot be None here.");
            if ![glow::FLOAT_VEC3, glow::FLOAT_VEC4].contains(&attr.atype) {
                Err(TypeMismatch {
                    location: 0,
                    attr_name: attr.name,
                })?;
            }
            let attr = gl
                .get_active_attribute(program, 1)
                .expect("Attribute cannot be None here.");
            if ![glow::FLOAT_VEC3].contains(&attr.atype) {
                Err(TypeMismatch {
                    location: 1,
                    attr_name: attr.name,
                })?;
            }
            Ok(())
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct PVertex {
    pub position: [f32; 3],
}

unsafe impl Vertex for PVertex {
    unsafe fn set_layout(gl: &glow::Context) {
        unsafe {
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(
                0,
                3,
                glow::FLOAT,
                false,
                std::mem::size_of::<Self>() as i32,
                0,
            );
        }
    }

    fn validate_layout(
        gl: &glow::Context,
        program: NativeProgram,
    ) -> Result<(), ShaderValidationError> {
        use ShaderValidationError::*;
        unsafe {
            let attr_count = gl.get_active_attributes(program);
            match attr_count.cmp(&1) {
                Ordering::Less => Err(TooFewAttributes)?,
                Ordering::Greater => Err(TooManyAttributes)?,
                Ordering::Equal => {}
            }
            let attr = gl
                .get_active_attribute(program, 0)
                .expect("Attribute cannot be None here.");
            if ![glow::FLOAT_VEC3, glow::FLOAT_VEC4].contains(&attr.atype) {
                Err(TypeMismatch {
                    location: 0,
                    attr_name: attr.name,
                })?;
            }
            Ok(())
        }
    }
}
