use bevy::{prelude::*, render::mesh::VertexAttributeValues};

use crate::pts::Vectors;

// this removes and returns vertices from a mesh
pub fn extract_vertices(mesh: &mut Mesh) -> Option<Vec<Vec3>> {
    let data = mesh.remove_attribute(Mesh::ATTRIBUTE_POSITION)?;
    if let VertexAttributeValues::Float32x3(point_data) = data {
        Some(Vectors::from_vec_of_arrays(point_data))
    } else {
        None
    }
}

pub fn set_vertices(mesh: &mut Mesh, pts: Vectors) {
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, Into::<Vec<[f32; 3]>>::into(pts))
}

// flip mesh normals by changing triangle orders
pub fn flip_normals(mesh: &mut Mesh) -> Option<()> {
    fn swap_ids<T: Copy>(vec: &mut Vec<T>) {
        for i in (1..vec.len()).step_by(3) {
            let temp = vec[i - 1];
            vec[i - 1] = vec[i];
            vec[i] = temp;
        }
    }

    let ids = mesh.indices_mut()?;

    match ids {
        bevy::render::mesh::Indices::U16(shorts) => swap_ids(shorts),
        bevy::render::mesh::Indices::U32(longs) => swap_ids(longs),
    }

    Some(())
}
