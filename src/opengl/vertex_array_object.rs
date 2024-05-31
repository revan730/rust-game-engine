use ogl33::{GL_FLOAT, glBindVertexArray, glEnableVertexAttribArray, GLenum, glGenVertexArrays, GLint, GLuint, glVertexAttribPointer};

pub struct VertexArrayObject(pub GLuint);

#[repr(u32)]
pub enum VertexAttribType {
    Float = GL_FLOAT,
}

impl VertexArrayObject {
    pub fn new() -> Option<Self> {
        let mut vao = 0;

        unsafe {
            glGenVertexArrays(1, &mut vao);
        }

        if vao == 0 {
            None
        } else {
            Some(Self(vao))
        }
    }

    pub fn bind(&self) {
        unsafe {
            glBindVertexArray(self.0);
        }
    }

    pub fn unbind() {
        unsafe {
            glBindVertexArray(0);
        }
    }

    pub fn set_vertex_attribute(index: usize, size: usize, attrib_type: VertexAttribType, normalized: bool, stride: usize, offset: usize) {
        unsafe {
            glVertexAttribPointer(index as GLuint, size as GLint, attrib_type as GLenum, normalized.try_into().unwrap(), stride.try_into().unwrap(), offset as *const _);
            glEnableVertexAttribArray(index as GLuint);
        }
    }
}