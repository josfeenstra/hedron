use crate::{core::Pose, math::Range3};

// represents a cuboid in 3D space
pub struct Cuboid {
    pose: Pose,
    size: Range3,
}

impl Cuboid {
    pub fn new(pose: Pose, size: Range3) -> Self {
        Self { pose, size }
    }

    pub fn new_cube(pose: Pose, radius: f32) -> Self {
        Self::new(pose, Range3::from_radius(radius))
    }
}
