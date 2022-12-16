use super::Mesh;
use crate::data::{Pool, Ptr};
use bevy_inspector_egui::egui::plot::Polygon;
use glam::{vec3, Vec3};

pub type VertPtr = Ptr;
pub type EdgePtr = Ptr;
pub type FacePtr = Ptr;

#[derive(Default, Debug)]
pub struct Vert {
    pub pos: Vec3,
    pub edge: Option<EdgePtr>,
}

// IF looking from ABOVE (using the normals provided during addition)
// IF looking to the edge going from BOTTOM to TOP
// THEN the face and next edge of this face is on the left side (leading to counter clockwise faces)
// AND THEN the edge twin is on the right side:
// ```
//       (TO)
// [next] /\     |
//       /||\    |
//        ||     |
//        || - [twin]
// [face] ||     |
//        ||    \|/
//        ||     V
// ```
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
/// Despite the name, the model can also be used as a planar partition.
/// It all depends on the normals used to determine the edge ordering around a vertex
#[derive(Default, Debug)]
pub struct Polyhedron {
    pub verts: Pool<Vert>, // disk operations should present a normal to orient around within the function itself. It should not be stored
    pub edges: Pool<HalfEdge>,
    pub faces: Pool<Face>, // TODO implement explicit faces later!
}

impl Polyhedron {
    // For various constructors

    pub fn new_grid() -> Self {
        let mut hedron = Polyhedron::new();

        let a: VertPtr = hedron.add_vert(vec3(0., 0., 0.));
        let b: VertPtr = hedron.add_vert(vec3(1., 0., 0.));
        let c: VertPtr = hedron.add_vert(vec3(1., 1., 0.));
        let d: VertPtr = hedron.add_vert(vec3(0., 1., 0.));

        const UP: Vec3 = Vec3::Z;
        hedron.add_edge_twins(a, b, &UP);
        hedron.add_edge_twins(b, c, &UP);
        hedron.add_edge_twins(c, d, &UP);
        hedron.add_edge_twins(d, a, &UP);

        hedron
    }
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
                hedron.verts.get(ia).expect("the mesh pointers should work"),
                hedron.verts.get(ib).expect("the mesh pointers should work"),
                hedron.verts.get(ic).expect("the mesh pointers should work"),
            );
            // get triangle normal

            // assume counter clockwise CHECK THIS rotation
            let face_normal = (b.pos - a.pos).cross(b.pos - a.pos);

            // add half edges in accordance with this normal
            for [from, to] in vec![[ia, ib], [ib, ic], [ic, ia]] {
                hedron.add_edge_twins(from, to, &face_normal);
            }
        }

        hedron
    }

    /////////////////////////////////////////////////////////////// Getting

    pub fn get_loops_as_faces(&self) -> Vec<Polygon> {
        // traverse all faces, construct polygon faces from them
        Vec::new()
    }

    pub fn get_all_verts(&self) -> Vec<Vec3> {
        self.verts.iter().map(|v| v.pos).collect()
    }

    pub fn get_all_debug_lines(&self) -> Vec<Vec3> {}

    /////////////////////////////////////////////////////////////// Transactions

    fn add_vert(&mut self, pos: Vec3) -> VertPtr {
        self.verts.push(Vert { pos, edge: None }) as VertPtr
    }

    /// NOTE: A - B is a different half edge than B - A.
    fn has_half_edge(&self, from: VertPtr, to: VertPtr) -> bool {
        false
    }

    pub fn add_edge_twins(&mut self, a: VertPtr, b: VertPtr, normal: &Vec3) -> Option<()> {
        // TODO add cases were one of the two does already exist
        // for now, quit if any one exist
        if self.has_half_edge(a, b) || self.has_half_edge(b, a) {
            return None;
        }

        // build the edge twins
        let to_a = self.edges.push(HalfEdge {
            to: a,
            next: None,
            twin: None,
            face: None,
        });
        let to_b = self.edges.push(HalfEdge {
            to: b,
            next: None,
            twin: None,
            face: None,
        });

        self.edges.get_mut(to_a)?.twin = Some(to_b);
        self.edges.get_mut(to_b)?.twin = Some(to_a);

        self.add_edge_to_disk(a, to_a)?;
        self.add_edge_to_disk(b, to_b)?;

        Some(())
    }

    fn delete_edge(&mut self, edge: EdgePtr) {
        // remove references to this edge

        self.edges.delete(edge)
    }

    /// set the `next` property of a given edge by adding the edge to the conceptual 'disk' of a vertex
    /// set the `vert` property of a vertex to the first added edge
    fn add_edge_to_disk(&mut self, vert_ptr: VertPtr, edge_ptr: EdgePtr) -> Option<()> {
        let vert = self.verts.get_mut(vert_ptr)?;
        let edge = self.edges.get_mut(edge_ptr)?;
        let twin_ptr = edge.twin.expect("we are always working with twins for now");
        let twin = self.edges.get_mut(twin_ptr)?;

        if vert.edge.is_none() {
            vert.edge = Some(edge_ptr);
            edge.next = Some(twin_ptr)
        } else if let Some(sm) = vert.edge {
        }

        if (v.edge == -1) {
            // set two pointers:
            v.edge = ei; // I am the vertex's first edge
            twin.next = ei; // that means my twin points back to me
        } else {
            let [ei_before, ei_after] = this.getDiskPositions(ei);
            let [e_before, e_after] = [this.getEdge(ei_before), this.getEdge(ei_after)];

            // set two pointers:
            this.getEdge(e_before.twin).next = ei;
            twin.next = this.getEdgeIndex(e_after);
        }
    }

    /////////////////////////////////////////////////////////////// Movement

    /////////////////////////////////////////////////////////////// Modelling

    /// cap closed planar holes by creating faces at these holes.
    fn cap(&mut self) {}
}

#[cfg(test)]
mod tests {
    use crate::solid::Polyhedron;
    use glam::{vec3, Vec3};

    #[test]
    fn hedron() {
        let mut ph = Polyhedron::new();
        let a = ph.add_vert(vec3(1.0, 0.0, 0.0));
        let b = ph.add_vert(vec3(1.0, 1.0, 0.0));
        let c = ph.add_vert(vec3(0.0, 0.0, 1.0));
        assert_eq!(a, 0);
        assert_eq!(b, 1);
        assert_eq!(c, 2);

        const UP: Vec3 = Vec3::Z;
        let p = ph.add_edge_twins(a, b, &UP);
        let p = ph.add_edge_twins(b, c, &UP);
        let p = ph.add_edge_twins(c, a, &UP);

        println!("{:?}", ph);
    }
}
