use crate::kernel::Vec3;


/// a cube-like thing with 8 corners
/// 3D equivalent of a quad
pub struct Octoid {
    pub verts: [Vec3; 8]
}

impl Octoid {
    pub fn new(verts: [Vec3; 8]) -> Self {
        Self { verts }
    }
}
