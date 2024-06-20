use ultraviolet::{Mat4, Vec3};

use crate::camera::Camera;
use crate::graphics::node_3d::Node3D;
use crate::math::aabb_bouding_box::AABBBoundingBox;

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
    bounding_box: AABBBoundingBox,
    vertical_velocity: f32,
    // TODO: RigidBody
}

impl PlayerCharacter {
    pub fn new(node3d: Node3D, camera: Camera, height: f32) -> Self {
        let width = 0.5;
        let bounding_box = AABBBoundingBox { x_min: -(width / 2.0), x_max: width / 2.0, y_max: height, y_min: 0.0, z_min: -(width / 2.0), z_max: width / 2.0 };

        Self {
            node3d,
            camera,
            movement_speed: SPEED,
            bounding_box,
            vertical_velocity: 0.0,
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

    pub fn check_collision(&self, bounding_box: AABBBoundingBox) -> bool {
        let bounding_box_translated = self.get_bounding_box_translated();
        bounding_box_translated.collides_with(bounding_box)
    }

    fn get_bounding_box_translated(&self) -> AABBBoundingBox {
        let mut bounding_box = self.bounding_box;
        let half_width = (bounding_box.x_max - bounding_box.x_min) / 2.0;
        let half_height = (bounding_box.y_max - bounding_box.y_min) / 2.0;
        let half_depth = (bounding_box.z_max - bounding_box.z_min) / 2.0;

        bounding_box.x_min = self.node3d.world_position.x - half_width;
        bounding_box.x_max = self.node3d.world_position.x + half_width;

        bounding_box.y_min = self.node3d.world_position.y - half_height;
        bounding_box.y_max = self.node3d.world_position.y + half_height;

        bounding_box.z_min = self.node3d.world_position.z - half_depth;
        bounding_box.z_max = self.node3d.world_position.z + half_depth;

        bounding_box
    }

    pub fn add_vertical_velocity(&mut self, velocity: f32) {
        self.vertical_velocity += velocity;
        if self.vertical_velocity <= -55.0 {
            // terminal velocity
            self.vertical_velocity = -55.0;
        }

        /*if self.vertical_velocity > 5.0 {
            self.vertical_velocity = 5.0;
        }*/
    }

    pub fn get_vertical_velocity(&self) -> f32 {
        self.vertical_velocity
    }

    pub fn reset_vertical_velocity(&mut self) {
        self.vertical_velocity = 0.0;
    }

    pub fn get_half_height(&self) -> f32 {
        (self.bounding_box.y_max - self.bounding_box.y_min) / 2.0
    }

    pub fn get_right_direction(&self) -> Vec3 {
        self.camera.right
    }
}