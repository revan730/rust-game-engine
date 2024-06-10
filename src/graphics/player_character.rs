use ultraviolet::{Mat4, Vec3};

use crate::camera::Camera;
use crate::graphics::node_3d::Node3D;

const SPEED: f32 = 20.5;

#[derive(Debug)]
pub enum MovementDirection {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
}

pub struct PlayerCharacter {
    node3d: Node3D,
    camera: Camera,
    movement_speed: f32,
    // TODO: RigidBody
}

impl PlayerCharacter {
    pub fn new(node3d: Node3D, camera: Camera) -> Self {
        Self {
            node3d,
            camera,
            movement_speed: SPEED,
        }
    }

    pub fn get_camera_view_matrix(&self) -> Mat4 {
        self.camera.get_view_matrix(self.node3d.world_position)
    }

    pub fn get_look_direction(&self) -> Vec3 {
        self.camera.front
    }

    pub fn process_keyboard(&mut self, direction: MovementDirection, dt: f32) {
        let velocity = self.movement_speed * dt;

        match direction {
            MovementDirection::Forward => self.node3d.world_position += self.camera.front * velocity,
            MovementDirection::Backward => self.node3d.world_position -= self.camera.front * velocity,
            MovementDirection::Left => self.node3d.world_position -= self.camera.right * velocity,
            MovementDirection::Right => self.node3d.world_position += self.camera.right * velocity,
            _ => (),
            //CameraMovement::Up => self.height += velocity,
            //CameraMovement::Down => self.height -= velocity,
        }

        // TODO: Reenable ?
        //self.node3d.world_position.y = self.height; // Keeps camera from flying so that the y position can only be changed with up/down movement keys
        self.node3d.world_position.y = 1.6;
    }

    pub fn process_mouse_movement(&mut self, x_offset: f32, y_offset: f32, constrain_pitch: bool) {
        self.camera.process_mouse_movement(x_offset, y_offset, constrain_pitch);
    }

    pub fn get_camera_zoom(&self) -> f32 {
        self.camera.zoom
    }

    pub fn get_position(&self) -> Vec3 {
        self.node3d.world_position
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.node3d.world_position.x = x;
        self.node3d.world_position.y = y;
        self.node3d.world_position.z = z;
    }
}