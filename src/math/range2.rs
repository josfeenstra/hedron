use std::ops::Range;
use glam::Vec2;
use crate::math::quick;

/// A two dimentional range. 
/// Can also be interpreted as an axis-aligned rectangle
pub struct Range2 {
    pub x: Range<f32>,
    pub y: Range<f32>,
    }

impl Range2 {

    pub const UNIT: Self = Self::from_vecs(Vec2::ZERO, Vec2::ONE);

    #[inline]
    pub const fn new(x: Range<f32>, y: Range<f32>) -> Self {
        Self { x, y }
    }
    
    pub fn from_radius(r: f32) -> Self {
        Self::new(-r..r, -r..r)
    }
    
    #[inline]
    pub const fn from_vecs(from: Vec2, to: Vec2) -> Self {
        Self::new(from.x..to.x, from.y..to.y)
    }

    pub fn normalize(&self, t: Vec2) -> Vec2 {
        Vec2::new(quick::normalize(t.x, &self.x), quick::normalize(t.y, &self.y))
    }
    
    pub fn lerp(&self, t: Vec2) -> Vec2 {
        Vec2::new(quick::interpolate(t.x, &self.x), quick::interpolate(t.y, &self.y))
    }
    
    /// remap from self to other
    pub fn remap(&self, other: &Self, t: Vec2, clamp: bool) -> Vec2 {
        Vec2::new(quick::remap(t.x, &self.x, &other.x, clamp), quick::remap(t.y, &self.y, &other.y, clamp))
    }
}
