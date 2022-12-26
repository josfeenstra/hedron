use glam::Vec3;
// I like to split away these intersection / geometric predicates stuff from the main code

// simple 2x2
#[inline]
pub fn det(a: f32, b: f32, c: f32, d: f32) -> f32 {
    a * d - b * c
}

/// yay wikipedia
#[rustfmt::skip]
#[inline]
pub fn line_line_2d(
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x3: f32,
    y3: f32,
    x4: f32,
    y4: f32,
) -> (f32, f32) {
    // TODO this can be optimized for minimum gains
    let px = det(det(x1, y1, x2, y2), det(x1, 1., x2, 1.), det(x3, y3, x4, y4), det(x3, 1., x4, 1.)) / 
             det(det(x1, 1., x2, 1.), det(y1, 1., y2, 1.), det(x3, 1., x4, 1.), det(y3, 1., y4, 1.));
    let py = det(det(x1, y1, x2, y2), det(y1, 1., y2, 1.), det(x3, y3, x4, y4), det(y3, 1., y4, 1.)) / 
             det(det(x1, 1., x2, 1.), det(y1, 1., y2, 1.), det(x3, 1., x4, 1.), det(y3, 1., y4, 1.));

    (px, py)
}
// https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
// NOTE : this must be implemented at some point: line segments, parameter T, and in generalized bezier matrix parameter space stuffs

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

