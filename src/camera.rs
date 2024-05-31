use std::ops::Add;

use ultraviolet::{Mat4, Vec3};

pub const YAW: f32 = -90.0;
pub const PITCH: f32 = 0.0;
const SPEED: f32 = 20.5;
const SENSITIVITY: f32 = 0.1;
const ZOOM: f32 = 45.0;

pub struct Camera {
    pub position: Vec3,
    pub front: Vec3,
    pub up: Vec3,
    pub right: Vec3,
    pub world_up: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
    pub zoom: f32,
}

#[derive(Debug)]
pub enum CameraMovement {
    Forward,
    Backward,
    Left,
    Right,
}

impl Camera {
    pub fn from_vec3(position: Vec3, up: Vec3, yaw: f32, pitch: f32) -> Self {
        let mut camera = Self {
            position: position,
            world_up: up,
            yaw: yaw,
            pitch: pitch,
            front: Vec3::new(0.0, 0.0, -1.0),
            movement_speed: SPEED,
            mouse_sensitivity: SENSITIVITY,
            zoom: ZOOM,
            up: Vec3::new(0.0, 0.0, 0.0),
            right: Vec3::new(0.0, 0.0, 0.0),
        };
        camera.update_camera_vectors();

        camera
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        Mat4::look_at(self.position, self.position.add(self.front), self.up)
    }

    pub fn process_keyboard(&mut self, direction: CameraMovement, dt: f32) {
        let velocity = self.movement_speed * dt;

        match direction {
            CameraMovement::Forward => self.position += self.front * velocity,
            CameraMovement::Backward => self.position -= self.front * velocity,
            CameraMovement::Left => self.position -= self.right * velocity,
            CameraMovement::Right => self.position += self.right * velocity,
        }

        self.position.y = 0.0;
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

        self.right = Vec3::from(self.front).cross(self.world_up);
        self.right.normalize();

        self.up = Vec3::from(self.right).cross(self.front);
        self.up.normalize();
    }
}