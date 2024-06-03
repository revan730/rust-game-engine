use ogl33::{GL_ELEMENT_ARRAY_BUFFER, GL_STATIC_DRAW, glBindBuffer, glBufferData, glGenBuffers, GLuint};

pub struct ElementBufferObject(pub GLuint);

impl ElementBufferObject {
    pub fn new() -> Option<Self> {
        let mut ebo = 0;

        unsafe {
            glGenBuffers(1, &mut ebo);
        }

        if ebo == 0 {
            None
        } else {
            Some(Self(ebo))
        }
    }

    pub fn bind(&self) {
        unsafe {
            glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, self.0);
        }
    }

    pub fn unbind() {
        unsafe {
            glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    pub fn load_data<T>(size: usize, data_ptr: *const T) {
        unsafe {
            glBufferData(GL_ELEMENT_ARRAY_BUFFER, size.try_into().unwrap(), data_ptr.cast(), GL_STATIC_DRAW);
        }
    }
}