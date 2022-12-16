use glam::Vec3;

// NOTE: these are not robust geometric predicates.
// At a later stage, and when that becomes relevant, I will incorporate the `robust` crate.

// UNTESTED
pub fn line_hits_triangle(l1: Vec3, l2: Vec3, p1: Vec3, p2: Vec3, p3: Vec3) -> bool {
    // first, test it like a plane
    if !line_hits_plane(l1, l2, p1, p2, p3) {
        return false;
    }
    if !line_hits_plane(p1, p2, l1, l2, p3) {
        return false;
    }
    if !line_hits_plane(p2, p3, l1, l2, p1) {
        return false;
    }
    if !line_hits_plane(p3, p1, l1, l2, p2) {
        return false;
    }
    true
}

// UNTESTED
pub fn line_hits_plane(l1: Vec3, l2: Vec3, p1: Vec3, p2: Vec3, p3: Vec3) -> bool {
    let left = signed_volume(p1, p2, p3, l1);
    let right = signed_volume(p1, p2, p3, l2);

    // we ignore the null case (the case where the line touches the plane)
    // left.abs() < f32::EPSILON || right.abs() < f32::EPSILON
    if (left > 0.0 && right > 0.0) || (left < 0.0 && right < 0.0) {
        false
    } else {
        true
    }
}

// UNTESTED
pub fn signed_volume(a: Vec3, b: Vec3, c: Vec3, d: Vec3) -> f32 {
    (1.0 / 6.0) * (a - d).dot((c - d).cross(b - d))
}
