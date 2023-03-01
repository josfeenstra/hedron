use crate::{data::Grid2, kernel::Vec3};

/// a rectangular bezier surface
pub struct BiSurface {
    pub cps: Grid2<Vec3>
}