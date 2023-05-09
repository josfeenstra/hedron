use crate::kernel::Vec3;

/// A list of points that will have a line drawn between each consecutive points
#[derive(Debug, Clone)]
pub struct Polyline {
    pub verts: Vec<Vec3>,
}

impl Polyline {
    pub fn new(verts: Vec<Vec3>) -> Self {
        Self { verts }
    }
}
