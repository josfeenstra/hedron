use glam::UVec3;
use rand::Rng;
use rand::distributions::Uniform;

use crate::kernel::{fxx, Vec3, uvec3_to_vec3};
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
    pub const UNIT: Self = Self::new(Vec3::ZERO, Vec3::ONE);

    #[inline]
    pub const fn new(from: Vec3, to: Vec3) -> Self {
        Self::from_ranges(from.x..to.x, from.y..to.y, from.z..to.z)
    }

    #[inline]
    pub const fn from_ranges(x: Range<fxx>, y: Range<fxx>, z: Range<fxx>) -> Self {
        Self { x, y, z }
    }

    pub fn from_radius(r: fxx) -> Self {
        Self::from_ranges(-r..r, -r..r, -r..r)
    }


    /// iterate through this space
    pub fn iter<'a>(&'a self, count: UVec3) -> impl Iterator<Item = Vec3> + 'a {
        let fcount: Vec3 = uvec3_to_vec3(count);
        util::iter_xyz_u(count).map(move |u| self.lerp(uvec3_to_vec3(u) / fcount))
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

    pub fn spawn<RNG: Rng>(&self, rng: &mut RNG, count: usize) -> Vec<Vec3> {
        let mut points = Vec::new();
        let ux = Uniform::from(self.x.clone());
        let uy = Uniform::from(self.y.clone());
        let uz = Uniform::from(self.z.clone());
        for i in 0..count {
            points.push(Vec3::new(rng.sample(ux), rng.sample(uy), rng.sample(uz)));
        }
        points
    }
}
