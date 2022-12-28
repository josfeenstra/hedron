use glam::UVec3;

use crate::kernel::{fxx, Vec3};
use crate::{math::quick, util};
use std::ops::Range;

/// A two dimentional range.
/// Can also be interpreted as an axis-aligned rectangle
pub struct Range3 {
    pub x: Range<fxx>,
    pub y: Range<fxx>,
    pub z: Range<fxx>,
}

impl Range3 {
    pub const UNIT: Self = Self::from_vecs(Vec3::ZERO, Vec3::ONE);

    #[inline]
    pub const fn new(x: Range<fxx>, y: Range<fxx>, z: Range<fxx>) -> Self {
        Self { x, y, z }
    }

    pub fn from_radius(r: fxx) -> Self {
        Self::new(-r..r, -r..r, -r..r)
    }

    #[inline]
    pub const fn from_vecs(from: Vec3, to: Vec3) -> Self {
        Self::new(from.x..to.x, from.y..to.y, from.z..to.z)
    }

    /// iterate through this space
    pub fn iter<'a>(&'a self, count: UVec3) -> impl Iterator<Item = Vec3> + 'a {
        let fcount = count.as_dvec3();
        util::iter_xyz_u(count).map(move |u| self.lerp(u.as_dvec3() / fcount))
    }

    /// normalize
    pub fn norm(&self, t: Vec3) -> Vec3 {
        Vec3::new(
            quick::normalize(t.x, &self.x),
            quick::normalize(t.y, &self.y),
            quick::normalize(t.z, &self.z),
        )
    }

    /// linearly interpolate
    pub fn lerp(&self, t: Vec3) -> Vec3 {
        Vec3::new(
            quick::interpolate(t.x, &self.x),
            quick::interpolate(t.y, &self.y),
            quick::interpolate(t.z, &self.z),
        )
    }

    /// remap from self to other
    pub fn remap(&self, other: &Self, t: Vec3, clamp: bool) -> Vec3 {
        Vec3::new(
            quick::remap(t.x, &self.x, &other.x, clamp),
            quick::remap(t.y, &self.y, &other.y, clamp),
            quick::remap(t.z, &self.z, &other.z, clamp),
        )
    }
}
