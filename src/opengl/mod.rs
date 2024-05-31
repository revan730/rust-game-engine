use beryllium::video::GlWindow;
use bitmask::bitmask;
use ogl33::{GL_COLOR_BUFFER_BIT, GL_DEPTH_BUFFER_BIT, GL_DEPTH_TEST, GL_LINES, GL_POINTS, GL_STENCIL_BUFFER_BIT, GL_TRIANGLES, glClear, glClearColor, glDrawArrays, glEnable, GLenum, GLint, GLsizei, load_gl_with};

pub mod vertex_array_object;
pub mod vertex_buffer_object;
pub mod texture;

bitmask! {
    pub mask ClearBitMask: u32 where flags ClearBitFlags {
        ColorBuffer = GL_COLOR_BUFFER_BIT,
        DepthBuffer = GL_DEPTH_BUFFER_BIT,
        StencilBuffer = GL_STENCIL_BUFFER_BIT,
    }
}

#[repr(u32)]
pub enum Primitive {
    Triangles = GL_TRIANGLES,
    Lines = GL_LINES,
    Points = GL_POINTS,
}

#[repr(u32)]
pub enum Capability {
    DepthTest = GL_DEPTH_TEST,
    // TODO: Add others
}

pub fn load_gl(gl_window: &GlWindow) {
    unsafe {
        load_gl_with(|f_name| gl_window.get_proc_address(f_name.cast()));
    }
}

pub fn enable(capability: Capability) {
    unsafe {
        glEnable(capability as GLenum);
    }
}

pub fn clear_color(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe {
        glClearColor(red, green, blue, alpha);
    }
}

pub fn clear(bit_mask: ClearBitMask) {
    unsafe {
        glClear(*bit_mask);
    }
}

pub fn draw_arrays(primitive: Primitive, start: usize, count: usize) {
    unsafe {
        glDrawArrays(primitive as GLenum, start as GLint, count as GLsizei);
    }
}