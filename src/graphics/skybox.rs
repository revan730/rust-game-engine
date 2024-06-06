use std::mem::size_of;

use ultraviolet::Mat4;

use crate::opengl;
use crate::opengl::{DepthFunc, gl_depth_func, Primitive};
use crate::opengl::texture::{MagFilterParam, MinFilterParam, Texture, TextureType, WrapCoordinate, WrapParam};
use crate::opengl::vertex_array_object::VertexArrayObject;
use crate::opengl::vertex_array_object::VertexAttribType::Float;
use crate::opengl::vertex_buffer_object::{BufferUsage, VertexBufferObject};
use crate::shader::Shader;

type SkyboxVertex = [f32; 3];

const SKYBOX_VERTICES: [SkyboxVertex; 36] = [
    // 1
    [-1.0, 1.0, -1.0],
    [-1.0, -1.0, -1.0],
    [1.0, -1.0, -1.0],
    [1.0, -1.0, -1.0],
    [1.0, 1.0, -1.0],
    [-1.0, 1.0, -1.0],
    // 2
    [-1.0, -1.0, 1.0],
    [-1.0, -1.0, -1.0],
    [-1.0, 1.0, -1.0],
    [-1.0, 1.0, -1.0],
    [-1.0, 1.0, 1.0],
    [-1.0, -1.0, 1.0],
    // 3
    [1.0, -1.0, -1.0],
    [1.0, -1.0, 1.0],
    [1.0, 1.0, 1.0],
    [1.0, 1.0, 1.0],
    [1.0, 1.0, -1.0],
    [1.0, -1.0, -1.0],
    // 4
    [-1.0, -1.0, 1.0],
    [-1.0, 1.0, 1.0],
    [1.0, 1.0, 1.0],
    [1.0, 1.0, 1.0],
    [1.0, -1.0, 1.0],
    [-1.0, -1.0, 1.0],
    // 5
    [-1.0, 1.0, -1.0],
    [1.0, 1.0, -1.0],
    [1.0, 1.0, 1.0],
    [1.0, 1.0, 1.0],
    [-1.0, 1.0, 1.0],
    [-1.0, 1.0, -1.0],
    // 6
    [-1.0, -1.0, -1.0],
    [-1.0, -1.0, 1.0],
    [1.0, -1.0, -1.0],
    [1.0, -1.0, -1.0],
    [-1.0, -1.0, 1.0],
    [1.0, -1.0, 1.0],
];

pub struct Skybox {
    texture: Texture,
    shader_program: Shader,
    vao: VertexArrayObject,
    vbo: VertexBufferObject,
}

impl Skybox {
    pub fn new_from_image_paths(shader_program: Shader, paths: [&str; 6]) -> Self {
        let texture = Texture::new(TextureType::CubeMap).expect("Failed to allocate texture for skybox");
        texture.bind();
        Texture::load_cube_map_from_paths(paths);

        texture.set_min_filter(MinFilterParam::Linear);
        texture.set_mag_filter(MagFilterParam::Linear);
        texture.set_wrap(WrapCoordinate::S, WrapParam::ClampToEdge);
        texture.set_wrap(WrapCoordinate::T, WrapParam::ClampToEdge);
        texture.set_wrap(WrapCoordinate::R, WrapParam::ClampToEdge);

        let vao = VertexArrayObject::new().expect("Failed to allocate vertex array object for skybox");
        let vbo = VertexBufferObject::new().expect("Failed to allocate vertex buffer object for skybox");
        vao.bind();
        vbo.bind();

        VertexBufferObject::load_data(SKYBOX_VERTICES.len() * size_of::<SkyboxVertex>(), SKYBOX_VERTICES.as_ptr(), BufferUsage::StaticDraw);

        VertexArrayObject::set_vertex_attribute(0, 3, Float, false, size_of::<SkyboxVertex>(), 0);

        shader_program.bind();
        shader_program.set_int("skybox", 0);

        Self {
            texture,
            shader_program,
            vao,
            vbo,
        }
    }

    pub fn draw(&self, camera_view: Mat4, projection: Mat4) {
        gl_depth_func(DepthFunc::LEqual);

        self.shader_program.bind();
        let mut view = camera_view;
        view.cols[3].x = 0.0;
        view.cols[3].y = 0.0;
        view.cols[3].z = 0.0;
        self.shader_program.set_mat4("view", view);
        self.shader_program.set_mat4("projection", projection);

        self.vao.bind();
        Texture::set_active_texture(0);
        self.texture.bind();
        opengl::draw_arrays(Primitive::Triangles, 0, SKYBOX_VERTICES.len());
        VertexArrayObject::unbind();
        gl_depth_func(DepthFunc::Less);
    }
}