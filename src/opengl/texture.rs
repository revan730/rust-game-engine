use std::ptr::null;

use image::{ColorType, DynamicImage};
use ogl33::{GL_CLAMP_TO_BORDER, GL_CLAMP_TO_EDGE, GL_LINEAR, GL_LINEAR_MIPMAP_LINEAR, GL_LINEAR_MIPMAP_NEAREST, GL_MIRRORED_REPEAT, GL_NEAREST, GL_NEAREST_MIPMAP_LINEAR, GL_NEAREST_MIPMAP_NEAREST, GL_R16, GL_R8, GL_RED, GL_REPEAT, GL_RG, GL_RG16, GL_RG8, GL_RGB, GL_RGB16, GL_RGB8, GL_RGBA, GL_RGBA16, GL_RGBA8, GL_TEXTURE0, GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_TEXTURE_MIN_FILTER, GL_TEXTURE_WRAP_S, GL_TEXTURE_WRAP_T, GL_UNSIGNED_BYTE, GL_UNSIGNED_SHORT, glActiveTexture, glBindTexture, GLenum, glGenerateMipmap, glGenTextures, GLint, glTexImage2D, glTexParameteri, glTexSubImage2D, GLuint};

pub struct Texture(pub GLuint);

#[repr(u32)]
pub enum WrapCoordinate {
    S = GL_TEXTURE_WRAP_S,
    T = GL_TEXTURE_WRAP_T,
}

#[repr(u32)]
pub enum WrapParam {
    Repeat = GL_REPEAT,
    ClampToEdge = GL_CLAMP_TO_EDGE,
    ClampToBorder = GL_CLAMP_TO_BORDER,
    MirroredRepeat = GL_MIRRORED_REPEAT,
}

#[repr(u32)]
pub enum MinFilterParam {
    Nearest = GL_NEAREST,
    Linear = GL_LINEAR,
    NearestMipmapNearest = GL_NEAREST_MIPMAP_NEAREST,
    LinearMipmapNearest = GL_LINEAR_MIPMAP_NEAREST,
    NearestMipmapLinear = GL_NEAREST_MIPMAP_LINEAR,
    LinearMipmapLinear = GL_LINEAR_MIPMAP_LINEAR,
}

#[repr(u32)]
pub enum MagFilterParam {
    Nearest = GL_NEAREST,
    Linear = GL_LINEAR,
}

impl Texture {
    pub fn new() -> Option<Self> {
        let mut texture = 0;

        unsafe {
            glGenTextures(1, &mut texture);
        }

        if texture == 0 {
            None
        } else {
            Some(Self(texture))
        }
    }

    pub fn bind(&self) {
        unsafe {
            // TODO: Support other texture types in the future
            glBindTexture(GL_TEXTURE_2D, self.0);
        }
    }

    pub fn unbind() {
        unsafe {
            glBindTexture(GL_TEXTURE_2D, 0);
        }
    }

    pub fn set_wrap(coordinate: WrapCoordinate, param: WrapParam) {
        unsafe {
            glTexParameteri(GL_TEXTURE_2D, coordinate as GLenum, param as GLint);
        }
    }

    pub fn set_min_filter(param: MinFilterParam) {
        unsafe {
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, param as GLint);
        }
    }

    pub fn set_mag_filter(param: MagFilterParam) {
        unsafe {
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, param as GLint);
        }
    }

    // TODO: Handle reading from file
    pub fn load_from_image_buffer(image_buffer: DynamicImage, generate_mipmap: bool) {
        let color = image_buffer.color();

        let (internal_format, pixel_format, data_type) = match color {
            ColorType::L8 => (GL_R8, GL_RED, GL_UNSIGNED_BYTE),
            ColorType::L16 => (GL_R16, GL_RED, GL_UNSIGNED_SHORT),
            ColorType::La8 => (GL_RG8, GL_RG, GL_UNSIGNED_BYTE),
            ColorType::La16 => (GL_RG16, GL_RG, GL_UNSIGNED_SHORT),
            ColorType::Rgb8 => (GL_RGB8, GL_RGB, GL_UNSIGNED_BYTE),
            ColorType::Rgb16 => (GL_RGB16, GL_RGB, GL_UNSIGNED_SHORT),
            ColorType::Rgba8 => (GL_RGBA8, GL_RGBA, GL_UNSIGNED_BYTE),
            ColorType::Rgba16 => (GL_RGBA16, GL_RGBA, GL_UNSIGNED_SHORT),
            _ => panic!("Unsupported color type"),
        };
        unsafe {
            glTexImage2D(GL_TEXTURE_2D, 0, internal_format as GLint, image_buffer.width().try_into().unwrap(), image_buffer.height().try_into().unwrap(), 0, pixel_format, data_type, image_buffer.as_bytes().as_ptr().cast());
            if generate_mipmap {
                glGenerateMipmap(GL_TEXTURE_2D);
            }
        }
    }

    pub fn load_empty(width: u32, height: u32) {
        unsafe {
            glTexImage2D(GL_TEXTURE_2D, 0, GL_R8 as GLint, width.try_into().unwrap(), height.try_into().unwrap(), 0, GL_RED, GL_UNSIGNED_BYTE, null());
        }
    }

    pub fn set_active_texture(index: usize) {
        let gl_index = GL_TEXTURE0 + index as GLenum;

        unsafe {
            glActiveTexture(gl_index);
        }
    }

    pub fn upload_pixels<T>(&self, x_offset: u32, y_offset: u32, width: u32, height: u32, data_ptr: *const T) {
        unsafe {
            glTexSubImage2D(GL_TEXTURE_2D, 0, x_offset.try_into().unwrap(), y_offset.try_into().unwrap(), width.try_into().unwrap(), height.try_into().unwrap(), GL_RED, GL_UNSIGNED_BYTE, data_ptr.cast())
        }
    }
}