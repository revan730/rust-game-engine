use std::ffi::CString;
use std::fmt::{Display, Formatter};
use std::fs;

use ogl33::{GL_COMPILE_STATUS, GL_FALSE, GL_FRAGMENT_SHADER, GL_LINK_STATUS, GL_TRUE, GL_VERTEX_SHADER, glAttachShader, glCompileShader, glCreateProgram, glCreateShader, glDeleteShader, glGetProgramInfoLog, glGetProgramiv, glGetShaderInfoLog, glGetShaderiv, glGetUniformLocation, glLinkProgram, glShaderSource, GLuint, glUniform1f, glUniform1i, glUniform3f, glUniformMatrix4fv, glUseProgram};
use ultraviolet::Mat4;

use crate::shader::SourceType::{Fragment, Program, Vertex};

pub struct Shader {
    pub program_id: GLuint,
}

enum SourceType {
    Vertex,
    Fragment,
    Program,
}

impl Display for SourceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Vertex => write!(f, "VERTEX SHADER"),
            Fragment => write!(f, "FRAGMENT SHADER"),
            Program => write!(f, "SHADER PROGRAM")
        }
    }
}

impl Shader {
    pub fn from_files(vertex_path: &str, fragment_path: &str) -> Self {
        let vertex_data = match fs::read(vertex_path) {
            Ok(data) => data,
            Err(e) => panic!("Couldn't find vertex shader file: {e:?}")
        };

        let fragment_data = match fs::read(fragment_path) {
            Ok(data) => data,
            Err(e) => panic!("Couldn't find fragment shader file: {e:?}")
        };

        unsafe {
            let vertex = glCreateShader(GL_VERTEX_SHADER);
            glShaderSource(vertex, 1, &(vertex_data.as_ptr().cast()), &(vertex_data.len().try_into().unwrap()));
            glCompileShader(vertex);
            check_compile_errors(vertex, Vertex);

            let fragment = glCreateShader(GL_FRAGMENT_SHADER);
            glShaderSource(fragment, 1, &(fragment_data.as_ptr().cast()), &(fragment_data.len().try_into().unwrap()));
            glCompileShader(fragment);
            check_compile_errors(fragment, Fragment);

            let program_id = glCreateProgram();
            glAttachShader(program_id, vertex);
            glAttachShader(program_id, fragment);
            glLinkProgram(program_id);
            check_compile_errors(program_id, Program);

            glDeleteShader(vertex);
            glDeleteShader(fragment);

            Self {
                program_id
            }
        }
    }

    pub fn bind(&self) {
        unsafe {
            glUseProgram(self.program_id);
        }
    }

    pub fn set_bool(&self, name: &str, value: bool) {
        unsafe {
            glUniform1i(glGetUniformLocation(self.program_id, CString::new(name).unwrap().as_ptr().cast()), value.into());
        }
    }

    pub fn set_int(&self, name: &str, value: i64) {
        unsafe {
            glUniform1i(glGetUniformLocation(self.program_id, CString::new(name).unwrap().as_ptr().cast()), value.try_into().unwrap());
        }
    }

    pub fn set_float(&self, name: &str, value: f32) {
        unsafe {
            glUniform1f(glGetUniformLocation(self.program_id, CString::new(name).unwrap().as_ptr().cast()), value);
        }
    }

    pub fn set_vec3(&self, name: &str, x: f32, y: f32, z: f32) {
        unsafe {
            glUniform3f(glGetUniformLocation(self.program_id, CString::new(name).unwrap().as_ptr().cast()), x, y, z);
        }
    }

    pub fn set_mat4(&self, name: &str, mat: Mat4) {
        unsafe {
            glUniformMatrix4fv(glGetUniformLocation(self.program_id, CString::new(name).unwrap().as_ptr().cast()), 1, GL_FALSE, mat.as_ptr().cast());
        }
    }
}

unsafe fn check_compile_errors(id: GLuint, source_type: SourceType) {
    let mut success = 0;
    let mut buf = Vec::<u8>::with_capacity(1024);
    let mut log_len = 0_i32;

    match source_type {
        SourceType::Vertex | SourceType::Fragment => {
            glGetShaderiv(id, GL_COMPILE_STATUS, &mut success);
            if success != i32::from(GL_TRUE) {
                glGetShaderInfoLog(id, 1024, &mut log_len, buf.as_mut_ptr().cast());
                buf.set_len(log_len.try_into().unwrap());
                panic!("Failed to compile {source_type}: {}", String::from_utf8_lossy(&buf));
            }
        }
        SourceType::Program => {
            glGetProgramiv(id, GL_LINK_STATUS, &mut success);
            if success != i32::from(GL_TRUE) {
                glGetProgramInfoLog(id, 1024, &mut log_len, buf.as_mut_ptr().cast());
                buf.set_len(log_len.try_into().unwrap());
                panic!("Failed to compile {source_type}: {}", String::from_utf8_lossy(&buf));
            }
        }
    }
}