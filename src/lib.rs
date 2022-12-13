#![allow(dead_code, unused_variables)]

pub mod algos;
pub mod core;
pub mod data;
pub mod math;
pub mod render;
pub mod smoothing;
pub mod util;

pub mod pts;
pub mod lines;
pub mod planar;
pub mod srf;
pub mod solid;
pub mod raster;

#[cfg(test)]
mod tests {
    use crate::{math::elastic_out};

    #[test]
    fn elastics() {
        for i in 0..101 {
            let t = (i as f32) / 100.0;
            println!("{}: {}", t, elastic_out(t));
        }
    }
}
