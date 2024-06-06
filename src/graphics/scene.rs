use ultraviolet::Mat4;

use crate::graphics::skybox::Skybox;
use crate::graphics::static_body_3d::StaticBody3D;
use crate::shader::Shader;

pub struct Scene {
    static_bodies: Vec<StaticBody3D>,
    skybox: Option<Skybox>,
    // TODO: Gui?
    // TODO: lights
    // TODO: particles
}

impl Scene {
    pub fn new(static_bodies: Vec<StaticBody3D>, skybox: Option<Skybox>) -> Self {
        Self {
            static_bodies,
            skybox,
        }
    }
    pub fn update(delta_time: f32) {
        // TODO: Update particles, lights, dynamic meshes (entities)
        todo!()
    }

    pub fn draw(&self, shader_program: &Shader, camera_view: Mat4, projection: Mat4) {
        // TODO: Not sure if we need to pass shader from the outside or shaders will be loaded into scene
        for body in &self.static_bodies {
            body.draw(shader_program);
        }

        if self.skybox.as_ref().is_some() {
            self.skybox.as_ref().unwrap().draw(camera_view, projection);
        }
    }
}