use ultraviolet::Vec3;

use crate::math::rotation::Rotation;

pub struct Node3D {
    pub world_position: Vec3,
    pub scale: Vec3,
    pub rotation: Rotation,
    // TODO: Rotation, transform ???
}