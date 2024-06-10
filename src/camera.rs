use std::ops::Add;

use ultraviolet::{Mat4, Vec3};

pub const YAW: f32 = -90.0;
pub const PITCH: f32 = 0.0;
const SENSITIVITY: f32 = 0.1;
const ZOOM: f32 = 70.0;

pub struct Camera {
    pub position: Vec3,
    pub front: Vec3,
    pub up: Vec3,
    pub right: Vec3,
    pub world_up: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub mouse_sensitivity: f32,
    pub zoom: f32,
    pub height: f32,
}


impl Camera {
    pub fn from_vec3(position: Vec3, up: Vec3, yaw: f32, pitch: f32) -> Self {
        let mut camera = Self {
            position,
            world_up: up,
            yaw,
            pitch,
            front: Vec3::new(0.0, 0.0, -1.0),
            mouse_sensitivity: SENSITIVITY,
            zoom: ZOOM,
            up: Vec3::default(),
            right: Vec3::default(),
            height: position.y,
        };
        camera.update_camera_vectors();

        camera
    }

    pub fn get_view_matrix(&self, world_position: Vec3) -> Mat4 {
        let position = self.position + world_position;

        Mat4::look_at(position, position.add(self.front), self.up)
    }

    pub fn process_mouse_movement(&mut self, x_offset: f32, y_offset: f32, constrain_pitch: bool) {
        self.yaw += x_offset * self.mouse_sensitivity;
        self.pitch += y_offset * self.mouse_sensitivity;

        if constrain_pitch {
            if self.pitch > 89.0 {
                self.pitch = 89.0;
            }
            if self.pitch < -89.0 {
                self.pitch = -89.0;
            }
        }

        self.update_camera_vectors();
    }

    pub fn process_mouse_scroll(&mut self, y_offset: f32) {
        self.zoom -= y_offset;

        if self.zoom < 1.0 {
            self.zoom = 1.0;
        }
        if self.zoom > 45.0 {
            self.zoom = 45.0;
        }
    }

    fn update_camera_vectors(&mut self) {
        self.front.x = self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
        self.front.y = self.pitch.to_radians().sin();
        self.front.z = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();
        self.front.normalize();

        self.right = self.front.cross(self.world_up);
        self.right.normalize();

        self.up = self.right.cross(self.front);
        self.up.normalize();
    }
}