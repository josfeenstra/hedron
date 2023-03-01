// #![allow(dead_code, unused_variables,  unused_imports)]

pub mod algos;
pub mod core;
pub mod data;
pub mod kernel;
pub mod math;
pub mod util;
pub mod various;

pub mod lines;
pub mod planar;
pub mod pts;
pub mod raster;
pub mod solid;
pub mod srf;

#[cfg(feature = "bevy")]
pub mod render;

#[cfg(feature = "bevy")]
pub mod smoothing;

#[cfg(test)]
mod tests {
    use crate::{kernel::fxx, math::elastic_out};

    #[test]
    fn elastics() {
        for i in 0..101 {
            let t = (i as fxx) / 100.0;
            println!("{}: {}", t, elastic_out(t));
        }
    }
}
