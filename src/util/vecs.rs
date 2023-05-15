use crate::kernel::*;

// more things for vec3, vec2

#[inline]
fn tolerance_equals(a: Vec3, b: Vec3, tolerance: fxx) -> bool {
    fxx::abs(a.x - b.x) < tolerance
        && fxx::abs(a.y - b.y) < tolerance
        && fxx::abs(a.z - b.z) < tolerance
}

#[inline]
pub fn epsilon_equals(a: Vec3, b: Vec3) -> bool {
    tolerance_equals(a, b, fxx::EPSILON)
}

#[inline]
pub fn roughly_equals(a: Vec3, b: Vec3) -> bool {
    tolerance_equals(a, b, VEC_TOLERANCE)
}
