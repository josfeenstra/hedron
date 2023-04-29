pub mod algos;
pub mod core;
pub mod data;
pub mod kernel;
pub mod lines;
pub mod math;
pub mod planar;
pub mod pts;
pub mod raster;
pub mod solid;
pub mod srf;
pub mod util;
pub mod various;

#[cfg(feature = "bevy")]
pub mod render;

#[cfg(feature = "bevy")]
pub mod smoothing;

pub mod prelude {
    pub use crate::algos::*;
    pub use crate::core::*;
    pub use crate::data::*;
    pub use crate::kernel::*;
    pub use crate::lines::*;
    pub use crate::math::*;
    pub use crate::planar::*;
    pub use crate::pts::*;
    pub use crate::raster::*;
    pub use crate::solid::*;
    pub use crate::srf::*;
    pub use crate::util::*;
    pub use crate::various::*;

    #[cfg(feature = "bevy")]
    pub use crate::render::*;

    #[cfg(feature = "bevy")]
    pub use crate::smoothing::*;
}

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
