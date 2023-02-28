use glam::UVec2;
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rand::rngs::ThreadRng;
use rand::Rng;

use crate::kernel::{fxx, Vec2, vec2, uvec2_to_vec2};
use crate::math::quick;
use crate::util;
use std::ops::Range;
use super::Range1;

/// A 2D range, or axis-aligned rectangle
#[derive(Debug)]
pub struct Range2 {
    pub x: Range<fxx>,
    pub y: Range<fxx>,
}

impl Range2 {
    pub const UNIT: Self = Self::new(Vec2::ZERO, Vec2::ONE);

    #[inline]
    pub const fn new(start: Vec2, end: Vec2) -> Self {
        Self::from_ranges(start.x..end.x, start.y..end.y)
    }

    #[inline]
    pub const fn from_ranges(x: Range<fxx>, y: Range<fxx>) -> Self {
        Self { x, y }
    }

    pub fn splat(x: Range<fxx>) -> Self {
        Self { x: x.clone(), y: x }
    }

    pub fn from_radius(r: fxx) -> Self {
        Self::from_ranges(-r..r, -r..r)
    }

    pub fn center(&self) -> Vec2 {
        self.lerp(Vec2::new(0.5,0.5))
    }
    
    pub fn includes(&self, t: Vec2) -> bool {
        !(t.x < self.x.start || t.x > self.x.end ||
          t.y < self.y.start || t.y > self.y.end)
    }

    pub fn expand_to(&mut self, t: Vec2) {
        self.x.expand_to(t.x);
        self.y.expand_to(t.y);
    }

    pub fn add(&mut self, rhs: Vec2) {
        self.x.start += rhs.x;
        self.x.end   += rhs.x;
        self.y.start += rhs.y;
        self.y.end   += rhs.y;
    }

    pub fn scale(&mut self, rhs: Vec2) {
        self.x.start *= rhs.x;
        self.x.end   *= rhs.x;
        self.y.start *= rhs.y;
        self.y.end   *= rhs.y;
    }

    pub fn scale_u(&mut self, scalar: fxx) {
        self.scale(vec2(scalar, scalar));
    }

    pub fn normalize(&self, t: Vec2) -> Vec2 {
        Vec2::new(
            self.x.normalize(t.x),
            self.y.normalize(t.y),
        )
    }

    pub fn lerp(&self, t: Vec2) -> Vec2 {
        Vec2::new(
            self.x.lerp(t.x),
            self.y.lerp(t.y),
        )
    }

    /// remap from self to other
    pub fn remap(&self, other: &Self, t: Vec2, clamp: bool) -> Vec2 {
        Vec2::new(
            self.x.remap(&other.x, t.x, clamp),
            self.y.remap(&other.y, t.y, clamp),
        )
    }

    // Produces a continuous random generator.
    // This code works, but we have to clone thread rng. not nice if we are dealing with a seeded random
    // requires `with_chunks::2<>` to properly fix things 
    pub fn gen<'a, RNG: Rng + Clone>(&self, rng: &'a mut RNG) -> impl Iterator<Item = Vec2> + 'a {
        // let vec = rng.gen::<Vec2>();
        let dist_x = rng.clone().sample_iter(Uniform::new(self.x.start, self.x.end));
        let dist_y = rng.sample_iter(Uniform::new(self.y.start, self.y.end));
        dist_x.zip(dist_y).map(|(x, y)| Vec2::new(x, y))
    }

    /// iterator-less
    pub fn spawn<RNG: Rng>(&self, rng: &mut RNG, count: usize) -> Vec<Vec2> {
        let mut points = Vec::new();
        let ux = Uniform::from(self.x.clone());
        let uy = Uniform::from(self.y.clone());
        for i in 0..count {
            points.push(Vec2::new(rng.sample(ux), rng.sample(uy)));
        }
        points
    }

    /// Explained in Range3 
    pub fn iter(&self, n_times: UVec2) -> impl Iterator<Item = Vec2> + '_ {
        let fcount: Vec2 = uvec2_to_vec2(n_times) - Vec2::ONE;
        util::iter_xy_u(n_times).map(move |u| self.lerp(uvec2_to_vec2(u) / fcount))
    }

    /// duplicate, but different implementation. Well see which one sticks...
    pub fn iter_n_times(&self, times: UVec2) -> impl Iterator<Item = Vec2> + '_ {
        self.y.iter_n_times(times.y as usize)
            .flat_map(move |y| self.x.iter_n_times(times.x as usize)
            .map(move |x| vec2(x, y))
        )
    }

    pub fn iter_by_delta(&self, delta: Vec2) -> impl Iterator<Item = Vec2> + '_ {
        self.y.iter_by_delta(delta.y)
            .flat_map(move |y| self.x.iter_by_delta(delta.x)
            .map(move |x| vec2(x, y))
        )
    }

    // fn iter_by_delta(&self, delta: fxx) -> Self::Iter {
    //     Box::new(iter_by_delta(self.start, self.end, delta))
    // }
}

#[cfg(test)]
mod tests {
    use crate::kernel::vec2;
    use rand::prelude::*;
    use rand_pcg::Pcg64;
    use rand_seeder::Seeder;
    use super::Range2;

    #[test]
    fn test_random_sampling() {
        let rect = Range2::new(vec2(0.5, -0.5), vec2(10.,15.));
        let mut rng: Pcg64 = Seeder::from("stripy zebra").make_rng();
        // let mut rng = rand::thread_rng();
        println!("gen:");
        for v in rect.gen(&mut rng).take(10) {
            println!("{v}");
        }

        println!("spawn:");
        for v in rect.spawn(&mut rng, 10) {
            println!("{v}");
        }
    }
}