use std::hash::{Hash, Hasher};

use ultraviolet::{Vec2, Vec3};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vertex {
    position: Vec3,
    tex_coord: Vec2,
    // TODO: Normals, bi-tangent, tangent etc.
}

impl Vertex {
    pub fn new(position: Vec3, tex_coord: Vec2) -> Self {
        Self {
            position,
            tex_coord,
        }
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
            && self.tex_coord == other.tex_coord
    }
}

impl Eq for Vertex {}

impl Hash for Vertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position[0].to_bits().hash(state);
        self.position[1].to_bits().hash(state);
        self.position[2].to_bits().hash(state);
        self.tex_coord[0].to_bits().hash(state);
        self.tex_coord[1].to_bits().hash(state);
    }
}