use std::ptr::null;

use image::{ColorType, DynamicImage};
use ogl33::{GL_CLAMP_TO_BORDER, GL_CLAMP_TO_EDGE, GL_LINEAR, GL_LINEAR_MIPMAP_LINEAR, GL_LINEAR_MIPMAP_NEAREST, GL_MIRRORED_REPEAT, GL_NEAREST, GL_NEAREST_MIPMAP_LINEAR, GL_NEAREST_MIPMAP_NEAREST, GL_R16, GL_R8, GL_RED, GL_REPEAT, GL_RG, GL_RG16, GL_RG8, GL_RGB, GL_RGB16, GL_RGB8, GL_RGBA, GL_RGBA16, GL_RGBA8, GL_TEXTURE0, GL_TEXTURE_2D, GL_TEXTURE_CUBE_MAP, GL_TEXTURE_CUBE_MAP_POSITIVE_X, GL_TEXTURE_MAG_FILTER, GL_TEXTURE_MIN_FILTER, GL_TEXTURE_WRAP_R, GL_TEXTURE_WRAP_S, GL_TEXTURE_WRAP_T, GL_UNSIGNED_BYTE, GL_UNSIGNED_SHORT, glActiveTexture, glBindTexture, GLenum, glGenerateMipmap, glGenTextures, GLint, glTexImage2D, glTexParameteri, glTexSubImage2D, GLuint};

pub struct Texture(pub GLuint);

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TextureType {
    Texture2d = GL_TEXTURE_2D,
    CubeMap = GL_TEXTURE_CUBE_MAP,
    TextureCubeMapPositiveX = GL_TEXTURE_CUBE_MAP_POSITIVE_X,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum WrapCoordinate {
    S = GL_TEXTURE_WRAP_S,
    T = GL_TEXTURE_WRAP_T,
    R = GL_TEXTURE_WRAP_R,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum WrapParam {
    Repeat = GL_REPEAT,
    ClampToEdge = GL_CLAMP_TO_EDGE,
    ClampToBorder = GL_CLAMP_TO_BORDER,
    MirroredRepeat = GL_MIRRORED_REPEAT,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum MinFilterParam {
    Nearest = GL_NEAREST,
    Linear = GL_LINEAR,
    NearestMipmapNearest = GL_NEAREST_MIPMAP_NEAREST,
    LinearMipmapNearest = GL_LINEAR_MIPMAP_NEAREST,
    NearestMipmapLinear = GL_NEAREST_MIPMAP_LINEAR,
    LinearMipmapLinear = GL_LINEAR_MIPMAP_LINEAR,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum MagFilterParam {
    Nearest = GL_NEAREST,
    Linear = GL_LINEAR,
}

// TODO: I don't like this API implementation
// I want texture to store it's type so i don't have to provide it for each operation
// I want methods to be self-contained, for example, so i don't have to remember to bind texture before setting it's parameter, parameter setter should make sure it's set
// Current implementation is too OpenGLy-state machine like

fn load_image_file(path: &str) -> DynamicImage {
    let mut f = std::fs::File::open(path).unwrap();
    let mut bytes = vec![];
    std::io::Read::read_to_end(&mut f, &mut bytes).unwrap();

    image::load_from_memory(&bytes).unwrap()
}

fn get_gl_image_params_from_color(color: ColorType) -> (GLenum, GLenum, GLenum) {
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
    (internal_format, pixel_format, data_type)
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

    pub fn bind(&self, texture_type: TextureType) {
        unsafe {
            glBindTexture(texture_type as GLenum, self.0);
        }
    }

    pub fn unbind(texture_type: TextureType) {
        unsafe {
            glBindTexture(texture_type as GLenum, 0);
        }
    }

    pub fn set_wrap(texture_type: TextureType, coordinate: WrapCoordinate, param: WrapParam) {
        unsafe {
            glTexParameteri(texture_type as GLenum, coordinate as GLenum, param as GLint);
        }
    }

    pub fn set_min_filter(texture_type: TextureType, param: MinFilterParam) {
        unsafe {
            glTexParameteri(texture_type as GLenum, GL_TEXTURE_MIN_FILTER, param as GLint);
        }
    }

    pub fn set_mag_filter(texture_type: TextureType, param: MagFilterParam) {
        unsafe {
            glTexParameteri(texture_type as GLenum, GL_TEXTURE_MAG_FILTER, param as GLint);
        }
    }

    // TODO: Handle reading from file
    pub fn load_from_image_buffer(texture_type: TextureType, image_buffer: DynamicImage, generate_mipmap: bool) {
        let color = image_buffer.color();

        let (internal_format, pixel_format, data_type) = get_gl_image_params_from_color(color);
        unsafe {
            glTexImage2D(texture_type as GLenum, 0, internal_format as GLint, image_buffer.width().try_into().unwrap(), image_buffer.height().try_into().unwrap(), 0, pixel_format, data_type, image_buffer.as_bytes().as_ptr().cast());
            if generate_mipmap {
                glGenerateMipmap(texture_type as GLenum);
            }
        }
    }

    pub fn load_cube_map_from_paths(paths: [&str; 6]) {
        for (i, path) in paths.iter().enumerate() {
            let image_buffer = load_image_file(path);
            let (internal_format, pixel_format, data_type) = get_gl_image_params_from_color(image_buffer.color());

            unsafe {
                glTexImage2D(TextureType::TextureCubeMapPositiveX as GLenum + i as GLenum, 0, internal_format as GLint, image_buffer.width().try_into().unwrap(), image_buffer.height().try_into().unwrap(), 0, pixel_format, data_type, image_buffer.as_bytes().as_ptr().cast());
            }
        }
    }

    pub fn load_empty(texture_type: TextureType, width: u32, height: u32) {
        unsafe {
            glTexImage2D(texture_type as GLenum, 0, GL_R8 as GLint, width.try_into().unwrap(), height.try_into().unwrap(), 0, GL_RED, GL_UNSIGNED_BYTE, null());
        }
    }

    pub fn set_active_texture(index: usize) {
        let gl_index = GL_TEXTURE0 + index as GLenum;

        unsafe {
            glActiveTexture(gl_index);
        }
    }

    pub fn upload_pixels<T>(&self, texture_type: TextureType, x_offset: u32, y_offset: u32, width: u32, height: u32, data_ptr: *const T) {
        unsafe {
            glTexSubImage2D(texture_type as GLenum, 0, x_offset.try_into().unwrap(), y_offset.try_into().unwrap(), width.try_into().unwrap(), height.try_into().unwrap(), GL_RED, GL_UNSIGNED_BYTE, data_ptr.cast())
        }
    }
}