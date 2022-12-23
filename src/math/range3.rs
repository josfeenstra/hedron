use crate::math::quick;
use glam::Vec3;
use std::ops::Range;

/// A two dimentional range.
/// Can also be interpreted as an axis-aligned rectangle
pub struct Range3 {
    pub x: Range<f32>,
    pub y: Range<f32>,
    pub z: Range<f32>,
}

impl Range3 {
    pub const UNIT: Self = Self::from_vecs(Vec3::ZERO, Vec3::ONE);

    #[inline]
    pub const fn new(x: Range<f32>, y: Range<f32>, z: Range<f32>) -> Self {
        Self { x, y, z }
    }

    pub fn from_radius(r: f32) -> Self {
        Self::new(-r..r, -r..r, -r..r)
    }

    #[inline]
    pub const fn from_vecs(from: Vec3, to: Vec3) -> Self {
        Self::new(from.x..to.x, from.y..to.y, from.z..to.z)
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
