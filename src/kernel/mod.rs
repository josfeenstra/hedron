// The Floating-point precision is globally configurable using the "f32" feature

#[cfg(feature = "f32")]
#[allow(clippy::module_inception)]
pub mod kernel {
    #[allow(non_camel_case_types)]
    pub type fxx = f32;
    pub use std::f32 as floats;

    pub type Vec2 = bevy_math::Vec2;
    pub type Vec3 = bevy_math::Vec3;
    pub type Vec4 = bevy_math::Vec4;
    pub type Mat3 = bevy_math::Mat3;
    pub type Mat4 = bevy_math::Mat4;
    pub type Quat = bevy_math::Quat;
    pub type Affine3 = bevy_math::Affine3A;

    pub const TAU: f32 = std::f32::consts::TAU;
    pub const PI: f32 = std::f32::consts::PI;
    pub const FRAC_PI_2: f32 = std::f32::consts::FRAC_PI_2;
    pub const EPSILON: f32 = std::f32::EPSILON;

    pub const SQRT_OF_3: f32 = 1.7320508075688772;

    pub const INFINITY: f32 = std::f32::INFINITY;
    pub const NEG_INFINITY: f32 = std::f32::NEG_INFINITY;

    pub use bevy_math::vec2;
    pub use bevy_math::vec3;

    pub fn uvec3_to_vec3(some: bevy_math::UVec3) -> Vec3 {
        some.as_vec3()
    }

    pub fn uvec2_to_vec2(some: bevy_math::UVec2) -> Vec2 {
        some.as_vec2()
    }

    pub fn ivec3_to_vec3(some: bevy_math::IVec3) -> Vec3 {
        some.as_vec3()
    }

    pub fn ivec2_to_vec2(some: bevy_math::IVec2) -> Vec2 {
        some.as_vec2()
    }

    pub fn as_vec2<I: Into<Vec2>>(some: I) -> Vec2 {
        some.into()
    }

    pub fn as_vec3<I: Into<Vec3>>(some: I) -> Vec3 {
        some.into()
    }
}

#[cfg(not(feature = "f32"))]
#[allow(clippy::module_inception)]
pub mod kernel {
    #[allow(non_camel_case_types)]
    pub type fxx = f64;
    pub use std::f64 as floats;

    pub type Vec2 = bevy_math::DVec2;
    pub type Vec3 = bevy_math::DVec3;
    pub type Vec4 = bevy_math::DVec4;
    pub type Mat3 = bevy_math::DMat3;
    pub type Mat4 = bevy_math::DMat4;
    pub type Quat = bevy_math::DQuat;
    pub type Affine3 = bevy_math::DAffine3;

    pub const TAU: f64 = std::f64::consts::TAU;
    pub const PI: f64 = std::f64::consts::PI;
    pub const FRAC_PI_2: f64 = std::f64::consts::FRAC_PI_2;
    pub const EPSILON: f64 = std::f64::EPSILON;
    pub const SQRT_OF_3: f64 = 1.7320508075688772;

    pub const INFINITY: f64 = std::f64::INFINITY;
    pub const NEG_INFINITY: f64 = std::f64::NEG_INFINITY;

    pub use bevy_math::dvec2 as vec2;
    pub use bevy_math::dvec3 as vec3;

    pub fn uvec3_to_vec3(some: bevy_math::UVec3) -> Vec3 {
        some.as_dvec3()
    }

    pub fn uvec2_to_vec2(some: bevy_math::UVec2) -> Vec2 {
        some.as_dvec2()
    }

    pub fn ivec3_to_vec3(some: bevy_math::IVec3) -> Vec3 {
        some.as_dvec3()
    }

    pub fn ivec2_to_vec2(some: bevy_math::IVec2) -> Vec2 {
        some.as_dvec2()
    }

    pub fn as_mat4(mat: bevy_math::Mat4) -> Mat4 {
        mat.as_dmat4()
    }

    pub fn as_vec2<I: Into<Vec2>>(some: I) -> Vec2 {
        some.into()
    }

    pub fn as_vec3<I: Into<Vec3>>(some: I) -> Vec3 {
        some.into()
    }
}

pub use kernel::*;
