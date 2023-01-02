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
    pub const UNIT: Self = Self::new(Vec2::ZERO, Vec2::ONE);

    #[inline]
    pub const fn new(start: Vec2, end: Vec2) -> Self {
        Self::from_ranges(start.x..end.x, start.y..end.y)
    }

    #[inline]
    pub const fn from_ranges(x: Range<fxx>, y: Range<fxx>) -> Self {
        Self { x, y }
    }

    pub fn from_radius(r: fxx) -> Self {
        Self::from_ranges(-r..r, -r..r)
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

    // Produces a continuous random generator.
    // This code works, but we have to clone thread rng. not nice if we are dealing with a seeded random
    // requires `with_chunks::2<>` to properly fix things 
    pub fn gen<'a, RNG: Rng + Clone>(&'a self, rng: &'a mut RNG) -> impl Iterator<Item = Vec2> + 'a {
        // let vec = rng.gen::<Vec2>();
        let dist_x = rng.clone().sample_iter(Uniform::new(self.x.start, self.x.end));
        let dist_y = rng.sample_iter(Uniform::new(self.y.start, self.y.end));
        dist_x.zip(dist_y).map(|(x, y)| Vec2::new(x, y))
    }

    pub fn spawn<RNG: Rng>(&self, rng: &mut RNG, count: usize) -> Vec<Vec2> {
        let mut points = Vec::new();
        let ux = Uniform::from(self.x.clone());
        let uy = Uniform::from(self.y.clone());
        for i in 0..count {
            points.push(Vec2::new(rng.sample(ux), rng.sample(uy)));
        }
        points
    }
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