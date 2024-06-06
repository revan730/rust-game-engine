use std::mem::size_of;

use crate::graphics::vertex::Vertex;
use crate::opengl::draw_elements;
use crate::opengl::element_buffer_object::ElementBufferObject;
use crate::opengl::ElementType::UnsignedInt;
use crate::opengl::Primitive::Triangles;
use crate::opengl::texture::Texture;
use crate::opengl::vertex_array_object::VertexArrayObject;
use crate::opengl::vertex_array_object::VertexAttribType::Float;
use crate::opengl::vertex_buffer_object::{BufferUsage, VertexBufferObject};
use crate::shader::Shader;

pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    textures: Vec<Texture>,
    vao: VertexArrayObject,
    vbo: VertexBufferObject,
    ebo: ElementBufferObject,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Self {
        let mut mesh = Self {
            vertices,
            indices,
            textures,
            vao: VertexArrayObject::new().unwrap(),
            vbo: VertexBufferObject::new().unwrap(),
            ebo: ElementBufferObject::new().unwrap(),
        };

        mesh.setup();

        mesh
    }

    pub fn draw(&self, shader: &Shader) {
        for (i, texture) in self.textures.iter().enumerate() {
            Texture::set_active_texture(i);
            // TODO: Handle different texture types

            shader.set_int(format!("texture{}", i + 1).to_owned().as_str(), i.try_into().unwrap());
            texture.bind();
        }

        self.vao.bind();

        draw_elements(Triangles, self.indices.len(), UnsignedInt);

        VertexArrayObject::unbind();
        Texture::set_active_texture(0);
    }

    fn setup(&self) {
        self.vao.bind();
        self.vbo.bind();

        VertexBufferObject::load_data(self.vertices.len() * size_of::<Vertex>(), self.vertices.as_ptr(), BufferUsage::StaticDraw);
        self.ebo.bind();
        ElementBufferObject::load_data(self.indices.len() * size_of::<u32>(), self.indices.as_ptr());

        VertexArrayObject::set_vertex_attribute(0, 3, Float, false, size_of::<Vertex>(), 0);
        VertexArrayObject::set_vertex_attribute(1, 3, Float, false, size_of::<Vertex>(), size_of::<[f32; 3]>());

        // TODO: Other attributes when appliable
    }
}