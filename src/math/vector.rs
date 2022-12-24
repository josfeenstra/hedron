// some vector math, I don't know where else to put it
// Anytime a function askes for a normal, you MUST provide an actually normalized vector!
use glam::Vec3;
use std::f32::consts::TAU;

/// A simple but awesome operation to cheaply 'flatten' a point onto a plane.
pub fn project_point_to_plane(center: Vec3, normal: Vec3, point: Vec3) -> Vec3 {
    let center_to_point = point - center;
    let distance = center_to_point.dot(normal);
    point - distance * normal
}

/// Given a plane defined by a `center` and `normal`, and `points` surrounding the origin of this plane,
/// Return the indices of the point "before" and "after" a certain `sample` point.
pub fn get_vectors_between(
    center: Vec3,
    normal: Vec3,
    points: Vec<Vec3>,
    sample_point: Vec3,
) -> Option<(usize, usize)> {
    match points.len() {
        0 => return None,
        1 => return Some((0, 0)),
        _ => {}
    };

    let sample = project_point_to_plane(center, normal, sample_point);
    let to_sample = sample - center;

    let mut left_way_hit = TAU;
    let mut left_id = 0;
    let mut right_way_hit = TAU;
    let mut right_id = 0;

    // this only makes sense using an image
    for (i, point) in points.iter().enumerate() {
        let point = project_point_to_plane(center, normal, *point);
        let to_point = point - center;

        // one angle takes the short route, the other the long route.
        let mut right_angle = to_sample.angle_between(to_point);
        let mut left_angle = TAU - right_angle;

        // TODO check which sign is which
        // based on this choice, (1,4) or (4,1) will be returned
        // - FOR THE EDGE CASE angle == PI, this does not matter
        // 
        let sign = to_sample.cross(normal).dot(to_point) > 0.0;
        if sign {
            // println!("FLIPPERINO!");
            (left_angle, right_angle) = (right_angle, left_angle);
        }

        // println!("{i}, right: {right_angle}, left: {left_angle}");

        if left_angle < left_way_hit {
            left_way_hit = left_angle;
            left_id = i;
        }
        if right_angle < right_way_hit {
            right_way_hit = right_angle;
            right_id = i;
        }
    }
    // println!("best right, {right_id}, best left {left_id}");
    Some((right_id, left_id))
}

pub fn average(verts: &Vec<Vec3>) -> Vec3 {
    verts.iter().fold(Vec3::ZERO, |sum, item| sum + *item) / verts.len() as f32
}
