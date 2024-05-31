use ogl33::{GL_ARRAY_BUFFER, GL_STATIC_DRAW, glBindBuffer, glBufferData, glGenBuffers, GLuint};

pub struct VertexBufferObject(pub GLuint);

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

    pub fn load_data<T>(size: usize, data_ptr: *const T) {
        unsafe {
            glBufferData(GL_ARRAY_BUFFER, size.try_into().unwrap(), data_ptr.cast(), GL_STATIC_DRAW);
        }
    }
}