use std::ptr::null;

use image::{ColorType, DynamicImage};
use ogl33::{GL_CLAMP_TO_BORDER, GL_CLAMP_TO_EDGE, GL_LINEAR, GL_LINEAR_MIPMAP_LINEAR, GL_LINEAR_MIPMAP_NEAREST, GL_MIRRORED_REPEAT, GL_NEAREST, GL_NEAREST_MIPMAP_LINEAR, GL_NEAREST_MIPMAP_NEAREST, GL_R16, GL_R8, GL_RED, GL_REPEAT, GL_RG, GL_RG16, GL_RG8, GL_RGB, GL_RGB16, GL_RGB8, GL_RGBA, GL_RGBA16, GL_RGBA8, GL_TEXTURE0, GL_TEXTURE_2D, GL_TEXTURE_CUBE_MAP, GL_TEXTURE_CUBE_MAP_POSITIVE_X, GL_TEXTURE_MAG_FILTER, GL_TEXTURE_MIN_FILTER, GL_TEXTURE_WRAP_R, GL_TEXTURE_WRAP_S, GL_TEXTURE_WRAP_T, GL_UNSIGNED_BYTE, GL_UNSIGNED_SHORT, glActiveTexture, glBindTexture, GLenum, glGenerateMipmap, glGenTextures, GLint, glTexImage2D, glTexParameteri, glTexSubImage2D, GLuint};

pub struct Texture {
    id: GLuint,
    texture_type: GLenum,
}

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
    pub fn new(texture_type: TextureType) -> Option<Self> {
        let mut texture = 0;

        unsafe {
            glGenTextures(1, &mut texture);
        }

        if texture == 0 {
            None
        } else {
            Some(Self {
                id: texture,
                texture_type: texture_type as GLenum,
            })
        }
    }

    pub fn bind(&self) {
        unsafe {
            glBindTexture(self.texture_type, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            glBindTexture(self.texture_type, 0);
        }
    }

    pub fn set_wrap(&self, coordinate: WrapCoordinate, param: WrapParam) {
        self.bind();

        unsafe {
            glTexParameteri(self.texture_type, coordinate as GLenum, param as GLint);
        }
    }

    pub fn set_min_filter(&self, param: MinFilterParam) {
        self.bind();

        unsafe {
            glTexParameteri(self.texture_type, GL_TEXTURE_MIN_FILTER, param as GLint);
        }
    }

    pub fn set_mag_filter(&self, param: MagFilterParam) {
        self.bind();

        unsafe {
            glTexParameteri(self.texture_type, GL_TEXTURE_MAG_FILTER, param as GLint);
        }
    }

    pub fn load_from_image_path(&self, image_path: &str, generate_mipmap: bool) {
        self.bind();

        let image_buffer = {
            let mut f = std::fs::File::open(image_path).unwrap();
            let mut bytes = vec![];
            std::io::Read::read_to_end(&mut f, &mut bytes).unwrap();

            image::load_from_memory(&bytes).unwrap()
        };
        let color = image_buffer.color();

        let (internal_format, pixel_format, data_type) = get_gl_image_params_from_color(color);
        unsafe {
            glTexImage2D(self.texture_type, 0, internal_format as GLint, image_buffer.width().try_into().unwrap(), image_buffer.height().try_into().unwrap(), 0, pixel_format, data_type, image_buffer.as_bytes().as_ptr().cast());
            if generate_mipmap {
                glGenerateMipmap(self.texture_type);
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

    pub fn load_empty(&self, width: u32, height: u32) {
        self.bind();

        unsafe {
            glTexImage2D(self.texture_type, 0, GL_R8 as GLint, width.try_into().unwrap(), height.try_into().unwrap(), 0, GL_RED, GL_UNSIGNED_BYTE, null());
        }
    }

    pub fn set_active_texture(index: usize) {
        let gl_index = GL_TEXTURE0 + index as GLenum;

        unsafe {
            glActiveTexture(gl_index);
        }
    }

    pub fn upload_pixels<T>(&self, x_offset: u32, y_offset: u32, width: u32, height: u32, data_ptr: *const T) {
        self.bind();

        unsafe {
            glTexSubImage2D(self.texture_type, 0, x_offset.try_into().unwrap(), y_offset.try_into().unwrap(), width.try_into().unwrap(), height.try_into().unwrap(), GL_RED, GL_UNSIGNED_BYTE, data_ptr.cast())
        }
    }
}