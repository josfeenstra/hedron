use bevy::{
    prelude::*,
    render::mesh::Mesh as BevyMesh,
    render::mesh::VertexAttributeValues,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

use crate::{
    core::Pose,
    lines::{Bezier, LineList, Polyline},
    planar::Polygon,
    pts::Vectors,
    solid::{Mesh as HedronMesh, Polyhedron},
};

// make sure we can easily translate hedron types to bevy types

impl From<HedronMesh> for BevyMesh {
    fn from(hmesh: HedronMesh) -> Self {
        let mut mesh = BevyMesh::new(PrimitiveTopology::TriangleList);
        let verts = Vectors::new(hmesh.verts.clone()).to_vec_of_arrays();
        let uvs = hmesh
            .uvs
            .iter()
            .map(|v| v.to_array())
            .collect::<Vec<[f32; 2]>>();
        let ids = hmesh.tri.iter().map(|v| *v as u32).collect();

        mesh.insert_attribute(BevyMesh::ATTRIBUTE_POSITION, verts);

        match hmesh.normals {
            crate::prelude::Normals::Face(_f_normals) => {
                warn!("ignoring face normals...");
                // let normals = Vectors::new(_f_normals).to_vec_of_arrays();
                // if !normals.is_empty() {
                //     mesh.insert_attribute(BevyMesh::ATTRIBUTE_NORMAL, normals);
                // }
            }
            crate::prelude::Normals::Vertex(v_normals) => {
                let normals = Vectors::new(v_normals).to_vec_of_arrays();
                if !normals.is_empty() {
                    mesh.insert_attribute(BevyMesh::ATTRIBUTE_NORMAL, normals);
                }
            }
            crate::prelude::Normals::None => {
                // dont do anything
            }
        }

        if !uvs.is_empty() {
            mesh.insert_attribute(BevyMesh::ATTRIBUTE_UV_0, uvs);
        }
        mesh.set_indices(Some(Indices::U32(ids)));
        mesh
    }
}

// TODO translate all of these into hedron mesh, and translate that to a bevy mesh.
// These shouldnt exist here
// On second thought, the PrimitiveTopology property makes this translation fine i guess

impl From<Vectors> for BevyMesh {
    fn from(points: Vectors) -> Self {
        let mut mesh = BevyMesh::new(PrimitiveTopology::PointList);
        mesh.insert_attribute(BevyMesh::ATTRIBUTE_POSITION, points.to_vec_of_arrays());
        mesh
    }
}

impl From<Bezier> for BevyMesh {
    fn from(val: Bezier) -> Self {
        let mut mesh = BevyMesh::new(PrimitiveTopology::LineStrip);
        let mut vertices = vec![];
        for vec in val.to_polyline(100) {
            vertices.push(vec.to_array());
        }
        if let Some(last) = val.verts.last() {
            vertices.push(last.to_array())
        }
        mesh.insert_attribute(BevyMesh::ATTRIBUTE_POSITION, vertices);
        mesh
    }
}

impl From<Transform> for Pose {
    fn from(tf: Transform) -> Self {
        let _scale = Vec3::ZERO;
        // scale: tf.scale,
        Self {
            pos: tf.translation,
            rot: tf.rotation,
            // scale,
        }
    }
}

impl From<Pose> for Transform {
    fn from(pose: Pose) -> Self {
        // pose.scale
        Transform {
            translation: pose.pos,
            rotation: pose.rot,
            scale: Vec3::ZERO,
        }
    }
}

impl From<LineList> for BevyMesh {
    fn from(line: LineList) -> Self {
        let mut mesh = BevyMesh::new(PrimitiveTopology::LineList);
        let mut vertices = vec![];
        for vec in line.verts {
            vertices.push(vec.to_array());
        }
        mesh.insert_attribute(BevyMesh::ATTRIBUTE_POSITION, vertices.clone());
        mesh.insert_attribute(BevyMesh::ATTRIBUTE_NORMAL, vertices); // this makes no sense, but it makes errors go away in the current prepass stuff
        mesh
    }
}

impl From<Polyline> for BevyMesh {
    fn from(polyline: Polyline) -> Self {
        let mut vertices = vec![];
        let mut normals = vec![];
        for pos in polyline.get_verts() {
            vertices.push(pos.to_array());
            normals.push(Vec3::ZERO.to_array());
        }

        // This tells wgpu that the positions are a list of points
        // where a line will be drawn between each consecutive point
        let mut mesh = BevyMesh::new(PrimitiveTopology::LineStrip);

        mesh.insert_attribute(BevyMesh::ATTRIBUTE_POSITION, vertices);
        // Normals are currently required by bevy, but they aren't used by the [`LineMaterial`]
        mesh.insert_attribute(BevyMesh::ATTRIBUTE_NORMAL, normals);
        mesh
    }
}

impl From<Polygon> for Mesh {
    fn from(p: Polygon) -> Self {
        HedronMesh::from(p).into()
    }
}

impl From<Polyhedron> for Mesh {
    fn from(p: Polyhedron) -> Self {
        HedronMesh::from_join(
            p.all_cww_loops_as_polygons()
                .into_iter()
                .map(|pg| pg.offset(Vec3::Z, 0.02).triangulate_naive())
                .collect(),
        )
        .into()
    }
}

pub fn borrow_verts(mesh: &BevyMesh) -> Option<&Vec<[f32; 3]>> {
    let verts = mesh.attribute(BevyMesh::ATTRIBUTE_POSITION)?;
    if let VertexAttributeValues::Float32x3(vector) = verts {
        Some(vector)
    } else {
        None
    }
}

pub fn borrow_normals(mesh: &BevyMesh) -> Option<&Vec<[f32; 3]>> {
    let verts = mesh.attribute(BevyMesh::ATTRIBUTE_POSITION)?;
    if let VertexAttributeValues::Float32x3(vector) = verts {
        Some(vector)
    } else {
        None
    }
}
