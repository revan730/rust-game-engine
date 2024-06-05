use crate::graphics::static_body_3d::StaticBody3D;
use crate::shader::Shader;

pub struct Scene {
    static_bodies: Vec<StaticBody3D>,
    // TODO: Gui?
    // TODO: lights
    // TODO: particles
}

// TODO: StaticBody3D. It will have a model, and Node3D
// TODO: Node3D. It will have a transform, rotation and translation in scene (world) space

impl Scene {
    pub fn new(static_bodies: Vec<StaticBody3D>) -> Self {
        Self {
            static_bodies,
        }
    }
    pub fn update(delta_time: f32) {
        // TODO: Update particles, lights, dynamic meshes (entities)
        todo!()
    }

    pub fn draw(&self, shader_program: &Shader) {
        // TODO: Not sure if we need to pass shader from the outside or shaders will be loaded into scene
        for body in &self.static_bodies {
            body.draw(shader_program);
        }
    }
}