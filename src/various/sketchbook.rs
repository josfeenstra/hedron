use glam::IVec3;
use crate::{kernel::{Vec3, ivec3_to_vec3, vec3}, solid::{Octoid}, math::Range3};

// integer points
pub fn mask_to_bounding_boxes(from_cells: Vec<IVec3>) -> Vec<Range3> {
    from_cells
        .into_iter()
        .map(|v| ivec3_to_vec3(v))
        .map(|c| Range3::new(c - vec3(1.0, 1.0, 0.0), c + vec3(1.0, 1.0, 1.0)))
        .collect::<Vec<_>>()
}

/// translate vertices 
pub fn from_mask_to_world(
    vertices: Vec<Vec3>, 
    bounding_boxes: Vec<Range3>, 
    targets: Vec<Octoid>,
) -> Vec<Vec3> {

    assert!(bounding_boxes.len() > 0);
    assert!(bounding_boxes.len() == targets.len());


    let mut results = Vec::new();
    for vert in vertices {

        // figure out to which box the vertex corresponds with
        let min_index = bounding_boxes
            .iter()
            .map(|bb|bb.center().distance(vert))
            .enumerate()
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(index, _)| index)
            .expect("no bounding boxes!");

        // take that box, and translate
        let bb = &bounding_boxes[min_index];
        let target = &targets[min_index];
        let norm = bb.normalize(vert);
        results.push(target.tri_lerp(norm))
    }
    
    results
}


