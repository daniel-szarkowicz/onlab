use glow::{HasContext, NativeFramebuffer, NativeTexture};

pub const SHADOW_WIDTH: i32 = 2048;
pub const SHADOW_HEIGHT: i32 = SHADOW_WIDTH;

pub fn create_buffer(gl: &glow::Context) -> (NativeFramebuffer, NativeTexture) {
    #[allow(clippy::cast_possible_wrap)]
    unsafe {
        let depth_framebuffer = gl.create_framebuffer().unwrap();
        let depth_map = gl.create_texture().unwrap();
        gl.bind_texture(glow::TEXTURE_2D, Some(depth_map));
        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            glow::DEPTH_COMPONENT as i32,
            SHADOW_WIDTH,
            SHADOW_HEIGHT,
            0,
            glow::DEPTH_COMPONENT,
            glow::FLOAT,
            None,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_MIN_FILTER,
            glow::NEAREST as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_MAG_FILTER,
            glow::NEAREST as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_WRAP_S,
            glow::REPEAT as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_WRAP_T,
            glow::REPEAT as i32,
        );
        gl.bind_framebuffer(glow::FRAMEBUFFER, Some(depth_framebuffer));
        gl.framebuffer_texture_2d(
            glow::FRAMEBUFFER,
            glow::DEPTH_ATTACHMENT,
            glow::TEXTURE_2D,
            Some(depth_map),
            0,
        );
        gl.draw_buffer(glow::NONE);
        gl.read_buffer(glow::NONE);
        gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        (depth_framebuffer, depth_map)
    }
}
