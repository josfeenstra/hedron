use glam::{UVec3};
use rand::Rng;
use rand::distributions::Uniform;

use crate::kernel::{fxx, Vec3, uvec3_to_vec3, vec3};
use crate::{util};
use std::ops::Range;

use super::{Range1, Shaper};

/// A 3D range, or axis-aligned box
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

    pub fn splat(r: Range<fxx>) -> Self {
        Self { x: r.clone(), y: r.clone(), z: r }
    }

    pub fn from_radius(r: fxx) -> Self {
        Self::from_ranges(-r..r, -r..r, -r..r)
    }

    pub fn includes(&self, t: Vec3) -> bool {
        !(t.x < self.x.start || t.x > self.x.end ||
          t.y < self.y.start || t.y > self.y.end || 
          t.z < self.z.start || t.z > self.z.end)
    }

    pub fn expand_to(&mut self, t: Vec3) {
        self.x.expand_to(t.x);
        self.y.expand_to(t.y);
        self.z.expand_to(t.z);
    }

    pub fn center(&self) -> Vec3 {
        self.lerp(Vec3::new(0.5,0.5,0.5))
    }
    
    pub fn corners(&self) -> [Vec3; 8] {
        let mut i = 0;
        let arr: [Vec3; 8] = [0; 8].map(|_| {
            let x = if i     % 2 == 0 { self.x.start } else { self.x.end }; 
            let y = if (i/2) % 2 == 0 { self.y.start } else { self.y.end }; 
            let z = if (i/4) % 2 == 0 { self.z.start } else { self.z.end }; 
            i += 1;
            vec3(x, y, z)
        });
        arr
    }

    pub fn normalize(&self, t: Vec3) -> Vec3 {
        Vec3::new(
            self.x.normalize(t.x),
            self.y.normalize(t.y),
            self.z.normalize(t.z),
        )
    }

    /// linearly interpolate
    pub fn lerp(&self, t: Vec3) -> Vec3 {
        Vec3::new(
            self.x.lerp(t.x),
            self.y.lerp(t.y),
            self.z.lerp(t.z),
        )
    }

    pub fn lerp_shaped(&self, t: Vec3, shapers: (Shaper, Shaper, Shaper)) -> Vec3 {
        self.lerp(Vec3::new(shapers.0.eval(t.x), shapers.1.eval(t.y), shapers.2.eval(t.z)))
    }

    /// remap from self to other
    pub fn remap(&self, other: &Self, t: Vec3, clamp: bool) -> Vec3 {
        Vec3::new(
            self.x.remap(&other.x, t.x, clamp),
            self.x.remap(&other.y, t.y, clamp),
            self.x.remap(&other.z, t.z, clamp),
        )
    }

    pub fn spawn<RNG: Rng>(&self, rng: &mut RNG, count: usize) -> Vec<Vec3> {
        let mut points = Vec::new();
        let ux = Uniform::from(self.x.clone());
        let uy = Uniform::from(self.y.clone());
        let uz = Uniform::from(self.z.clone());
        for _ in 0..count {
            points.push(Vec3::new(rng.sample(ux), rng.sample(uy), rng.sample(uz)));
        }
        points
    }

    /// iterate through this space
    /// hmmm... this approach is more stable, floating point wise
    pub fn iter(&self, n_times: UVec3) -> impl Iterator<Item = Vec3> + '_ {
        let fcount: Vec3 = uvec3_to_vec3(n_times) + - Vec3::ONE;
        util::iter_xyz_u(n_times).map(move |u| self.lerp(uvec3_to_vec3(u) / fcount))
    }

    /// same as above. 
    /// Benchmark the fastest approach
    pub fn iter_n_times(&self, x_steps: usize, y_steps: usize, z_steps: usize) -> impl Iterator<Item = Vec3> + '_ {
        self.z.iter_n_times(z_steps)
            .flat_map(move |z| self.x.iter_n_times(y_steps)
            .flat_map(move |y| self.x.iter_n_times(x_steps)
            .map(move |x| vec3(x, y, z))
        ))
    }

    pub fn iter_by_delta(&self, delta: Vec3) -> impl Iterator<Item = Vec3> + '_ {
        self.z.iter_by_delta(delta.z)
            .flat_map(move |z| self.x.iter_by_delta(delta.y)
            .flat_map(move |y| self.x.iter_by_delta(delta.x)
            .map(move |x| vec3(x, y, z))
        ))
    }

}
