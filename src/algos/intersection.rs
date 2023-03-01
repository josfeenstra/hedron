use crate::{kernel::{fxx, Vec3}};
// I like to split away these intersection / geometric predicates stuff from the main code

// simple 2x2
#[inline]
pub fn det(a: fxx, b: fxx, c: fxx, d: fxx) -> fxx {
    a * d - b * c
}

/// yay wikipedia
#[rustfmt::skip]
#[inline]
pub fn line_line_2d(
    x1: fxx,
    y1: fxx,
    x2: fxx,
    y2: fxx,
    x3: fxx,
    y3: fxx,
    x4: fxx,
    y4: fxx,
) -> (fxx, fxx) {
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
    // left.abs() < fxx::EPSILON || right.abs() < fxx::EPSILON
    if (left > 0.0 && right > 0.0) || (left < 0.0 && right < 0.0) {
        false
    } else {
        true
    }
}

pub fn signed_volume(a: Vec3, b: Vec3, c: Vec3, d: Vec3) -> fxx {
    (1.0 / 6.0) * (a - d).dot((c - d).cross(b - d))
}

/// don't know if this works
pub fn line_x_plane(a: Vec3, b: Vec3, plane_pos: Vec3, plane_normal: Vec3) -> Option<fxx> {

    let ba = a - b;
    let cross = plane_normal;
    let top = cross.dot(a - plane_pos);
    let bot = ba.dot(cross);
    if bot == 0.0 {
        return None;
    }
    let t = top / bot;
    Some(t)    
}


#[cfg(test)]
mod test {
    use crate::kernel::{vec3};
    use super::line_x_plane;

    #[test]
    fn test() {
        assert_eq!(line_x_plane(
            vec3(-1.0, -1.0, -1.0), 
            vec3(1.0, 1.0, 1.0), 
            vec3(0.0, 0.0, 0.0), 
            vec3(0.0, 0.0, 1.0)
        ), Some(0.5));

        let res = line_x_plane(
            vec3(1.0, -1.0, -2.0), 
            vec3(-1.0, 1.0, 3.0), 
            vec3(0.0, 0.0, 0.0), 
            vec3(1.0, 0.0, 0.0)
        );  
        dbg!(res);

    }

}