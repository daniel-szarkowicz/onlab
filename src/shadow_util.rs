use glow::{HasContext, NativeFramebuffer, NativeTexture};

use crate::light;

pub fn create_buffer(gl: &glow::Context) -> (NativeFramebuffer, NativeTexture) {
    #[allow(clippy::cast_possible_wrap)]
    unsafe {
        let framebuffer = gl.create_framebuffer().unwrap();
        let texture = gl.create_texture().unwrap();
        gl.bind_texture(glow::TEXTURE_2D_ARRAY, Some(texture));
        gl.tex_image_3d(
            glow::TEXTURE_2D_ARRAY,
            0,
            glow::R32F as i32,
            light::SHADOW_WIDTH,
            light::SHADOW_HEIGHT,
            light::SHADOW_LAYERS,
            0,
            glow::RED,
            glow::FLOAT,
            None,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D_ARRAY,
            glow::TEXTURE_MIN_FILTER,
            glow::LINEAR as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D_ARRAY,
            glow::TEXTURE_MAG_FILTER,
            glow::LINEAR as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D_ARRAY,
            glow::TEXTURE_WRAP_S,
            glow::CLAMP_TO_EDGE as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D_ARRAY,
            glow::TEXTURE_WRAP_T,
            glow::CLAMP_TO_EDGE as i32,
        );
        gl.bind_framebuffer(glow::FRAMEBUFFER, Some(framebuffer));
        gl.framebuffer_texture(
            glow::FRAMEBUFFER,
            glow::COLOR_ATTACHMENT0,
            Some(texture),
            0,
        );
        gl.draw_buffer(glow::COLOR_ATTACHMENT0);
        gl.read_buffer(glow::NONE);
        gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        (framebuffer, texture)
    }
}
