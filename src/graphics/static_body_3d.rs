use std::rc::Rc;

use ultraviolet::Mat4;

use crate::graphics::model::Model;
use crate::graphics::node_3d::Node3D;
use crate::math::aabb_bouding_box::AABBBoundingBox;
use crate::shader::Shader;

pub struct StaticBody3D {
    pub node3d: Node3D,
    pub model: Rc<Model>, // TODO: Replace with resource manager and get model through it ?
    pub bounding_box: AABBBoundingBox,
}

// TODO: Collision shape (model?)

impl StaticBody3D {
    pub fn draw(&self, shader_program: &Shader) {
        let mut model = Mat4::from_translation(self.node3d.world_position);
        model = model * self.node3d.rotation.rotation_matrix() * Mat4::from_nonuniform_scale(self.node3d.scale);

        shader_program.set_mat4("model", model);

        self.model.draw(shader_program);
    }
}