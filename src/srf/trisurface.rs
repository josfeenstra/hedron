use crate::{data::Grid2, kernel::Vec3};

/// a triangular bezier surface, using baricentric coordinates
pub struct TriSurface {
    pub cps: Grid2<Vec3>
}