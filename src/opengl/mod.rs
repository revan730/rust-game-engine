use beryllium::video::GlWindow;
use bitmask::bitmask;
use ogl33::{GL_BLEND, GL_COLOR_BUFFER_BIT, GL_DEPTH_BUFFER_BIT, GL_DEPTH_TEST, GL_LINES, GL_ONE_MINUS_SRC_ALPHA, GL_POINTS, GL_SRC_ALPHA, GL_STENCIL_BUFFER_BIT, GL_TRIANGLES, GL_UNPACK_ALIGNMENT, GL_UNSIGNED_BYTE, GL_UNSIGNED_INT, GL_UNSIGNED_SHORT, glBlendFunc, glClear, glClearColor, glDrawArrays, glDrawElements, glEnable, GLenum, GLint, glPixelStorei, GLsizei, load_gl_with};

pub mod vertex_array_object;
pub mod vertex_buffer_object;
pub mod texture;
pub mod element_buffer_object;

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
    Blending = GL_BLEND,
    // TODO: Add others
}

#[repr(u32)]
pub enum ElementType {
    UnsignedByte = GL_UNSIGNED_BYTE,
    UnsignedShort = GL_UNSIGNED_SHORT,
    UnsignedInt = GL_UNSIGNED_INT,
}

#[repr(u32)]
pub enum BlendFactor {
    SrcAlpha = GL_SRC_ALPHA,
    OneMinusSrcAlpha = GL_ONE_MINUS_SRC_ALPHA,
}

#[repr(u32)]
pub enum UnpackAlignment {
    One = 1,
    Two = 2,
    Four = 4,
    Eight = 8,
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

pub fn draw_elements(primitive: Primitive, count: usize, element_type: ElementType) {
    unsafe {
        glDrawElements(primitive as GLenum, count as GLsizei, element_type as GLenum, std::ptr::null());
    }
}

pub fn blend_func(s_factor: BlendFactor, d_factor: BlendFactor) {
    unsafe {
        glBlendFunc(s_factor as GLenum, d_factor as GLenum);
    }
}

pub fn pixel_store_unpack_alignment(alignment: UnpackAlignment) {
    unsafe {
        glPixelStorei(GL_UNPACK_ALIGNMENT, alignment as GLint);
    }
}