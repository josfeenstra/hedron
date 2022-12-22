// some vector math, I don't know where else to put it
// Anytime a function askes for a normal, you MUST provide an actually normalized vector!
use glam::{Vec3, Vec2};
use std::f32::consts::TAU;

/// A simple but awesome operation to cheaply 'flatten' a point onto a plane.
pub fn project_point_to_plane(center: Vec3, normal: Vec3, point: Vec3) -> Vec3 {
    let center_to_point = point - center;
    let distance =  center_to_point.dot(normal);
    point - distance * normal
}

/// Given a plane defined by a `center` and `normal`, and `points` surrounding the origin of this plane,
/// Return the indices of the point "before" and "after" a certain `sample` point. 
pub fn get_vectors_between(center: Vec3, normal: Vec3, points: Vec<Vec3>, sample: Vec3) -> Option<(usize, usize)> {
    
    match points.len() {
        0 => return None,
        1 => return Some((0, 0)),
        _ => {},
    };

    let sample = project_point_to_plane(center, normal, sample);

    let mut left_way_hit = TAU;
    let mut left_id = 0;
    let mut right_way_hit = TAU;
    let mut right_id = 0;

    // this only makes sense using an image
    for (i, point) in points.iter().enumerate() {
        let point = project_point_to_plane(center, normal, *point);
        
        // one angle takes the short route, the other the long route. 
        let mut left_angle = (sample - center).angle_between(point - center);
        let mut right_angle = TAU - left_angle;
        
        // TODO check which sign is which
        // based on this choice, (1,4) or (4,1) will be returned
        let sign = sample.cross(point).dot(normal) > 0.0;
        if sign {
            (left_angle, right_angle) = (right_angle, left_angle); 
        } 

        if left_angle < left_way_hit {
            left_way_hit = left_angle;
            left_id = i;
        }
        if right_angle < right_way_hit {
            right_way_hit = right_angle;
            right_id = i;
        }
    }
    
    Some((right_id,left_id))
}