#[derive(Copy, Clone, Default)]
pub struct AABBBoundingBox {
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
    pub z_min: f32,
    pub z_max: f32,
}

impl AABBBoundingBox {
    pub fn collides_with(&self, other: AABBBoundingBox) -> bool {
        self.x_min <= other.x_max && self.x_max >= other.x_min && self.y_min <= other.y_max && self.y_max >= other.y_min && self.z_min <= other.z_max && self.z_max >= other.z_min
    }
}