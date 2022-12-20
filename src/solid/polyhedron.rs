use super::Mesh;
use crate::{
    data::{Pool, Ptr},
    planar::Polygon,
};
use glam::Vec3;

pub type VertPtr = Ptr;
pub type EdgePtr = Ptr;
pub type FacePtr = usize;

#[derive(Default, Debug)]
pub struct Vert {
    pub pos: Vec3,
    pub edge: Option<EdgePtr>,
}

#[derive(Default, Debug)]
pub struct HalfEdge {
    pub to: VertPtr,
    pub next: Option<EdgePtr>,
    pub twin: Option<EdgePtr>,
    pub face: Option<FacePtr>,
}

#[derive(Default, Debug)]
pub struct Face {
    pub edge: EdgePtr,
}

/// A polyhedron model.
/// Desprite the name, the model can also be used as a planar partition.
/// It all depends on the normals used to determine the edge ordering around a vertex
#[derive(Default, Debug)]
pub struct Polyhedron {
    verts: Pool<Vert>, // disk operations should present a normal to orient around within the function itself. It should not be stored
    edges: Pool<HalfEdge>,
    faces: Pool<Face>, // TODO implement faces later!
}

impl Polyhedron {
    pub fn new() -> Self {
        Polyhedron::default()
    }

    /// The triangle orientation of the mesh must be consistent!
    pub fn from_mesh(mesh: &Mesh) -> Self {
        let mut hedron = Polyhedron::new();

        for vert in mesh.verts.iter() {
            hedron.add_vert(*vert);
        }

        for (ia, ib, ic) in mesh.get_triangles() {
            let (a, b, c) = (
                hedron.get_vert(ia).unwrap(),
                hedron.get_vert(ib).unwrap(),
                hedron.get_vert(ic).unwrap(),
            );
            // get triangle normal
            // normal

            // add edges in accordance with this normal

            // hedron.add_edge(a, b, normal);
        }

        hedron
    }

    fn get_faces(&self) -> Vec<Polygon> {
        // traverse all faces, construct polygon faces from them
        Vec::new()
    }

    /////////////////////////////////////////////////////////////// Transactions

    fn add_vert(&mut self, pos: Vec3) -> VertPtr {
        self.verts.push(Vert { pos, edge: None }) as VertPtr
    }

    fn get_vert(&self, vert: VertPtr) -> Option<&Vert> {
        self.verts.get(vert)
    }

    fn delete_vert(&mut self, vert: VertPtr) {
        // 1. delete all edges having to do with this vert
        // 2. delete the vert
        self.verts.delete(vert);
    }

    // NOTE: A - B is a different half edge than B - A.
    fn has_half_edge(&self, a: VertPtr, b: VertPtr) -> bool {
        false
    }

    fn add_half_edge(&mut self, a: VertPtr, b: VertPtr, normal: &Vec3) {
        // self.edges.push(HalfEdge { });
        // self.edges.push(HalfEdge { });
    }

    fn delete_edge(&mut self) {}

    /////////////////////////////////////////////////////////////// Movement
}

#[cfg(test)]
mod tests {
    use crate::solid::Polyhedron;
    use glam::{vec3, Vec3};

    #[test]
    fn hedron() {
        let mut hedron = Polyhedron::new();
        let a = hedron.add_vert(vec3(1.0, 0.0, 0.0));
        let b = hedron.add_vert(vec3(1.0, 1.0, 0.0));
        let c = hedron.add_vert(vec3(0.0, 0.0, 1.0));
        assert_eq!(a, 0);
        assert_eq!(b, 1);
        assert_eq!(c, 2);

        const UP: Vec3 = Vec3::Z;
        let p = hedron.add_half_edge(a, b, &UP);
        let p = hedron.add_half_edge(b, c, &UP);
        let p = hedron.add_half_edge(c, a, &UP);

        println!("{:?}", hedron);
    }
}
