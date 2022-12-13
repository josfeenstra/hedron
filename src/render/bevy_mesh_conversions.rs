use bevy::{prelude::*, render::render_resource::PrimitiveTopology};

use crate::{
    core::Pose,
    lines::{Bezier, LineList, LineStrip},
    planar::Polygon,
    pts::Points,
};

// make sure we can easily translate hedron types to bevy types
impl From<Points> for Mesh {
    fn from(points: Points) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::PointList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, points.to_vec_of_arrays());
        mesh
    }
}

impl From<Bezier> for Mesh {
    fn from(val: Bezier) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
        let mut vertices = vec![];
        for vec in val.to_polyline(100) {
            vertices.push(vec.to_array());
        }
        if let Some(last) = val.verts.last() {
            vertices.push(last.to_array())
        }
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh
    }
}

impl From<Transform> for Pose {
    fn from(tf: Transform) -> Self {
        Self {
            pos: tf.translation,
            rot: tf.rotation,
            scale: tf.scale,
        }
    }
}

impl From<Pose> for Transform {
    fn from(pose: Pose) -> Self {
        Transform {
            translation: pose.pos,
            rotation: pose.rot,
            scale: pose.scale,
        }
    }
}

impl From<LineList> for Mesh {
    fn from(line: LineList) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::LineList);
        let mut vertices = vec![];
        for vec in line.verts {
            vertices.push(vec.to_array());
        }
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh
    }
}

impl From<LineStrip> for Mesh {
    fn from(line: LineStrip) -> Self {
        let mut vertices = vec![];
        let mut normals = vec![];
        for pos in line.verts {
            vertices.push(pos.to_array());
            normals.push(Vec3::ZERO.to_array());
        }

        // This tells wgpu that the positions are a list of points
        // where a line will be drawn between each consecutive point
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        // Normals are currently required by bevy, but they aren't used by the [`LineMaterial`]
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh
    }
}

impl From<Polygon> for Mesh {
    fn from(p: Polygon) -> Self {
        Mesh::from(LineStrip::from(p))
    }
}
