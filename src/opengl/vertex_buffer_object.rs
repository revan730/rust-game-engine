use ogl33::{GL_ARRAY_BUFFER, GL_DYNAMIC_DRAW, GL_STATIC_DRAW, glBindBuffer, glBufferData, GLenum, glGenBuffers, GLuint};

pub struct VertexBufferObject(pub GLuint);

#[repr(u32)]
pub enum BufferUsage {
    StaticDraw = GL_STATIC_DRAW,
    DynamicDraw = GL_DYNAMIC_DRAW,
}

impl VertexBufferObject {
    pub fn new() -> Option<Self> {
        let mut vbo = 0;

        unsafe {
            glGenBuffers(1, &mut vbo);
        }

        if vbo == 0 {
            None
        } else {
            Some(Self(vbo))
        }
    }

    pub fn bind(&self) {
        unsafe {
            glBindBuffer(GL_ARRAY_BUFFER, self.0);
        }
    }

    pub fn unbind() {
        unsafe {
            glBindBuffer(GL_ARRAY_BUFFER, 0);
        }
    }

    pub fn load_data<T>(size: usize, data_ptr: *const T, usage: BufferUsage) {
        unsafe {
            glBufferData(GL_ARRAY_BUFFER, size.try_into().unwrap(), data_ptr.cast(), usage as GLenum);
        }
    }
}