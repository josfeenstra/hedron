use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rand::rngs::ThreadRng;
use rand::Rng;

use crate::kernel::{fxx, Vec2};
use crate::math::quick;
use std::ops::Range;

/// A two dimentional range.
/// Can also be interpreted as an axis-aligned rectangle
pub struct Range2 {
    pub x: Range<fxx>,
    pub y: Range<fxx>,
}

impl Range2 {
    pub const UNIT: Self = Self::from_vecs(Vec2::ZERO, Vec2::ONE);

    #[inline]
    pub const fn new(x: Range<fxx>, y: Range<fxx>) -> Self {
        Self { x, y }
    }

    pub fn from_radius(r: fxx) -> Self {
        Self::new(-r..r, -r..r)
    }

    #[inline]
    pub const fn from_vecs(from: Vec2, to: Vec2) -> Self {
        Self::new(from.x..to.x, from.y..to.y)
    }

    pub fn normalize(&self, t: Vec2) -> Vec2 {
        Vec2::new(
            quick::normalize(t.x, &self.x),
            quick::normalize(t.y, &self.y),
        )
    }

    pub fn lerp(&self, t: Vec2) -> Vec2 {
        Vec2::new(
            quick::interpolate(t.x, &self.x),
            quick::interpolate(t.y, &self.y),
        )
    }

    /// remap from self to other
    pub fn remap(&self, other: &Self, t: Vec2, clamp: bool) -> Vec2 {
        Vec2::new(
            quick::remap(t.x, &self.x, &other.x, clamp),
            quick::remap(t.y, &self.y, &other.y, clamp),
        )
    }

    // this code works, but then we have to clone thread rng. not nice if we are dealing with a seeded random


    pub fn spawn<'a>(&'a self, rng: &'a mut ThreadRng) -> impl Iterator<Item = Vec2> + 'a {
        // let vec = rng.gen::<Vec2>();
        let dist_x = rng.clone().sample_iter(Uniform::new(self.x.start, self.x.end));
        let dist_y = rng.sample_iter(Uniform::new(self.y.start, self.y.end));
        dist_x.zip(dist_y).map(|(x, y)| Vec2::new(x, y))
    }
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;
    use rand_seeder::Seeder;
    use super::Range2;

    #[test]
    fn test_random_sampling() {
        let rect = Range2::new(-0.5 .. 0.5,-0.5 .. 0.5);
        let mut rng: SeedableRng = Seeder::from("stripy zebra").make_rng::<SeedableRng>();
        // for v in rect.spawn(&mut rng).take(10) {
        //     println!("{v}");
        // }
    }
}