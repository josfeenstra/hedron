use super::Mesh;
use crate::data::{Pool, Ptr};
use anyhow::Result;
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
// <===== ()
//     
// [next] /\     |
//       /||\    |
//        ||     |
// [face] || - [twin]
//        ||     |
//        ||    \|/
//        ||     V
// 
//       (TO)
// ```
#[derive(Default, Debug)]
pub struct HalfEdge {
    pub from: VertPtr,
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
        hedron.add_edge_twins(a, b, &UP, &UP);
        hedron.add_edge_twins(b, c, &UP, &UP);
        hedron.add_edge_twins(c, d, &UP, &UP);
        hedron.add_edge_twins(d, a, &UP, &UP);

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
                hedron.add_edge_twins(from, to, &face_normal, &face_normal);
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

    pub fn get_all_debug_lines(&self) -> Vec<Vec3> {
        todo!()
    }

    /////////////////////////////////////////////////////////////// Transactions
    
    // this is not a very rusty way of doing things, but come on, I need some progress :)

    fn edge(&mut self, ep: EdgePtr) -> &mut HalfEdge {
        self.edges.get_mut(ep).expect("edge ptr not found!")
    }
    
    fn vert(&mut self, vp: VertPtr) -> &mut Vert {
        self.verts.get_mut(vp).expect("vert ptr not found!")
    }


    fn add_vert(&mut self, pos: Vec3) -> VertPtr {
        self.verts.push(Vert { pos, edge: None }) as VertPtr
    }

    /// NOTE: A - B is a different half edge than B - A.
    fn has_half_edge(&self, from: VertPtr, to: VertPtr) -> bool {
        false
    }

    /// add two half edges between a and b, order doesnt matter.
    /// We need a normal to determine disk ordering, when inserting
    /// these half edges in the network of existing half edges
    pub fn add_edge_twins(
        &mut self,
        a: VertPtr,
        b: VertPtr,
        a_normal: &Vec3,
        b_normal: &Vec3,
    ) -> Option<()> {
        // TODO add cases were one of the two does already exist
        // for now, quit if any one exist
        if self.has_half_edge(a, b) || self.has_half_edge(b, a) {
            return None;
        }

        // build the edge twins
        let from_a = self.edges.push(HalfEdge {
            from: a,
            next: None,
            twin: None,
            face: None,
        });
        let from_b = self.edges.push(HalfEdge {
            from: b,
            next: None,
            twin: None,
            face: None,
        });

        // set twin pointers
        self.edges.get_mut(from_a)?.twin = Some(from_b);
        self.edges.get_mut(from_b)?.twin = Some(from_a);

        self.add_edge_to_vertex(from_a, a_normal);
        self.add_edge_to_vertex(from_b, b_normal);

        Some(())
    }

    fn delete_edge(&mut self, edge: EdgePtr) {
        // remove references to this edge

        self.edges.delete(edge)
    }

    /// TODO make the signature (vp: VertPtr, incoming vec, normal)
    /// To show that we really dont do anything with the incoming edge pointer other than query things
    fn get_disk_neighbors(&self, ep: EdgePtr, normal: &Vec3) -> Option<(EdgePtr, EdgePtr)> {
        let edge = self.edges.get(ep)?;
        let vert = self.verts.get(edge.from)?;
        let disk_edges = self.get_disk(edge.from)?;

        // TODO incoming vert

        let incoming_edges = disk_edges
            .iter()
            .skip(1)
            .step_by(2)
            .map(|ep| self.edges.get(*ep).unwrap().twin.unwrap());

        let neighbors = incoming_edges.map(|vp| self.verts.get(vp).unwrap().pos);

        let nb_vecs = neighbors.map(|nb| nb - vert.pos);

        // TODO do the actual ordering steps

        // based on ordering, figure out which two edges need to be retuned (incoming and its neighbors)

        todo!();
    }

    /// get the edges, lined as a disk around a vertex.
    /// starts with the outgoing edge the vertex points to
    fn get_disk(&self, vp: VertPtr) -> Option<Vec<EdgePtr>> {
        let start = self.verts.get(vp)?.edge?;
        let mut disk = Vec::new();
        let mut cursor = start;
        loop {
            let outc = self.edges.get(cursor)?;
            let inc = self.edges.get(outc.twin?)?;
            disk.push(cursor);
            disk.push(outc.twin?);
            cursor = inc.next?;
            if cursor == start {
                break;
            }
        }
        Some(disk)
    }

    /// set the `next` property of a given edge by adding the edge to the conceptual 'disk' of a vertex
    /// set the `vert` property of a vertex to the first added edge
    fn add_edge_to_vertex(&mut self, ep: EdgePtr, normal: &Vec3) {
        // pointer business
        let ep_outwards = ep;
        let vp = self.edge(ep).from;
        let ep_inwards = self.edge(ep).twin.expect("should have twin");
        let v = self.vert(vp);
        if v.edge.is_none() {
            // we are the first edge to be added to this vertex
            v.edge = Some(ep);
            self.edge(ep_inwards).next = Some(ep);
        } else if let Some(disk_start) = v.edge {
            let (ep_nb_inwards, ep_nb_outwards) = self.get_disk_neighbors(ep_outwards, normal).unwrap();
            
            self.edge(ep_inwards).next = Some(ep_nb_outwards);
            self.edge(ep_nb_inwards).next = Some(ep_outwards);
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
        let p = ph.add_edge_twins(a, b, &UP, &UP);
        let p = ph.add_edge_twins(b, c, &UP, &UP);
        let p = ph.add_edge_twins(c, a, &UP, &UP);

        println!("{:?}", ph);
    }
}
