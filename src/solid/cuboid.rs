use crate::{core::Pose, kernel::fxx, math::Range3};

// represents a cuboid in 3D space
pub struct Cuboid {
    pub pose: Pose,
    pub size: Range3,
}

impl Cuboid {
    pub fn new(pose: Pose, size: Range3) -> Self {
        Self { pose, size }
    }

    pub fn new_cube(pose: Pose, radius: fxx) -> Self {
        Self::new(pose, Range3::from_radius(radius))
    }
}
