use ultraviolet::Mat4;

pub struct Rotation {
    pub angle_x: f32,
    pub angle_y: f32,
    pub angle_z: f32,
}

impl Rotation {
    pub fn rotation_matrix(&self) -> Mat4 {
        Mat4::from_rotation_x(self.angle_x) * Mat4::from_rotation_y(self.angle_y) * Mat4::from_rotation_z(self.angle_z)
    }
}