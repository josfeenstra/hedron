use super::Mesh;
use crate::{
    data::{Pool, Ptr},
    math,
    planar::Polygon,
};
use glam::{vec3, Vec3};
use std::collections::HashSet;

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
    pub next: EdgePtr,         // edge always has a next
    pub twin: EdgePtr, // in our case, edge always has a twin. optional twins comes later TODO
    pub face: Option<FacePtr>, // not every loop is filled
}

#[derive(Default, Debug)]
pub struct Face {
    pub edge: EdgePtr,
}

/// A polyhedron model.
/// Implemented as a half edge mesh.  
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
        hedron.add_edge(a, b, UP, UP);
        hedron.add_edge(b, c, UP, UP);
        hedron.add_edge(c, d, UP, UP);
        hedron.add_edge(d, a, UP, UP);

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
                hedron.add_edge(from, to, face_normal, face_normal);
            }
        }

        hedron
    }

    /////////////////////////////////////////////////////////////// Debugging

    pub fn print_structure(&self) {
        fn fmt<T: std::fmt::Display>(v: Option<T>) -> String {
            v.map_or(".".to_owned(), |v| v.to_string())
        }

        println!("POLYHEDRON: ");
        println!("   ----------- VERTS ------------");
        println!("      vp | edge | pos   ");
        for (vp, vert) in self.verts.iter_enum() {
            println!(
                "  [{:w$}] | {:w$} | {:w$}",
                vp,
                fmt(vert.edge),
                vert.pos,
                w = 4
            );
        }

        println!("   ----------- EDGES ------------");
        println!("      ep | from | twin | next | face ");
        for (ep, edge) in self.edges.iter_enum() {
            println!(
                "  [{:w$}] | {:w$} | {:w$} | {:w$} | {:w$} ",
                ep,
                edge.from,
                edge.twin,
                edge.next,
                fmt(edge.face),
                w = 4
            );
        }

        println!("   ----------- FACES ------------");
        println!("      fp | edge |");
        for (fp, face) in self.faces.iter_enum() {
            println!("  ({}) | {}", fp, face.edge);
        }

        println!("   ----------- loops ------------");
        for (i, my_loop) in self.get_loops().iter().enumerate() {
            print!("loop: ");
            for edge in my_loop {
                print!(" {} ,", edge)
            }
            print!("\n")
        }
    }

    /////////////////////////////////////////////////////////////// Getting Geometry

    pub fn get_loops_as_faces(&self) -> Vec<Polygon> {
        // traverse all faces, construct polygon faces from them

        // TODO this is what we need now!!!
        // THE DATA STRUCTURE APPEARS CORRECT.
        // NOW WE NEED TO CREATE A HEDRON FROM A MESH, THEN CONVERT THE HEDRON TO POLYGONS,
        // AND THEN SCALE THE POLYGONS SO SEE IF WE ARE DOING THE RIGHT KIND OF THINGS!
        // [x] BUILD POLYGON MODEL
        // [ ] TRIANGULATE POLYGON / SIMPLE TRIANGULATE POLYGON
        // [ ] MESH -> HEDRON
        // [ ] HEDRON -> POLYGONS -> MESHES

        // sorry for this insane statement :)
        self.get_loops().iter().map(|lp| {
            Polygon::new(
                lp.iter()
                    .map(|ep| self.vert(self.edge(*ep).from).pos)
                    .collect(),
            )
        }).collect()
    }

    pub fn claim_all_verts(&self) -> Vec<Vec3> {
        self.verts.iter().map(|v| v.pos).collect()
    }

    pub fn iter_verts_mut(&self) -> Vec<Vec3> {
        
    }

    fn get_edge_verts(&self, ep: EdgePtr) -> (Vec3, Vec3) {
        let e = self.edge(ep);
        let t = self.edge(e.twin);
        (self.vert(e.from).pos, self.vert(t.from).pos)
    }

    pub fn get_all_debug_lines(&self) -> Vec<Vec3> {
        todo!()
    }

    /////////////////////////////////////////////////////////////// Transactions

    // this is not a very rusty way of doing things, but come on, I need some progress :)

    fn edge(&self, ep: EdgePtr) -> &HalfEdge {
        self.edges.get(ep).expect("edge ptr not found!")
    }

    fn vert(&self, vp: VertPtr) -> &Vert {
        self.verts.get(vp).expect("vert ptr not found!")
    }

    fn mut_edge(&mut self, ep: EdgePtr) -> &mut HalfEdge {
        self.edges.get_mut(ep).expect("edge ptr not found!")
    }

    fn mut_vert(&mut self, vp: VertPtr) -> &mut Vert {
        self.verts.get_mut(vp).expect("vert ptr not found!")
    }

    fn add_vert(&mut self, pos: Vec3) -> VertPtr {
        self.verts.push(Vert { pos, edge: None }) as VertPtr
    }

    /// NOTE: A - B is a different half edge than B - A.
    fn has_half_edge(&self, from: VertPtr, to: VertPtr) -> bool {
        false
    }

    fn delete_edge(&mut self, edge: EdgePtr) {
        // remove references to this edge

        self.edges.delete(edge)
    }

    /////////////////////////////////////////////////////////////// Complex Transactions & Traversals

    /// get all loops in the half-edge structure
    pub fn get_loops(&self) -> Vec<Vec<EdgePtr>> {
        let mut loops = Vec::new();

        let mut passed = HashSet::<usize>::new();
        for (ep, edge) in self.edges.iter_enum() {
            if passed.contains(&ep) {
                continue;
            }
            let my_loop = self.get_loop(ep);

            // store the loop so we dont do it again
            for ep in my_loop.iter() {
                if !passed.insert(*ep) {
                    println!("WARN: loops appear to have overlapped...")
                }
            }
            loops.push(my_loop);
        }

        loops
    }

    /// get the loop asociated with a certain half-edge, by continuously following 'next'
    /// this half-edge will appear as the first edge
    pub fn get_loop(&self, ep: EdgePtr) -> Vec<EdgePtr> {
        let mut my_loop = Vec::new();
        let mut cursor = ep;
        my_loop.push(cursor);
        for _ in 0..self.edges.len() {
            // loops will never be longer than the number of half edges
            cursor = self.edge(cursor).next;
            if cursor == ep {
                return my_loop;
            }
            my_loop.push(cursor);
        }
        println!("WARN: prevented an infinite loop in 'get_loop'...");
        my_loop
    }

    /// add two half edges between a and b, order doesnt matter.
    /// We need a normal to determine disk ordering, when inserting
    /// these half edges in the network of existing half edges
    pub fn add_edge(
        &mut self,
        a: VertPtr,
        b: VertPtr,
        a_normal: Vec3,
        b_normal: Vec3,
    ) -> Option<()> {
        let Some((from_a, from_b)) = self.add_dangling_twins(a, b) else {
            return None;
        };
        self.add_edge_to_vertex(from_a, a_normal);
        self.add_edge_to_vertex(from_b, b_normal);

        Some(())
    }

    /// Add an edge using the positive z axis
    pub fn add_planar_edge(&mut self, a: VertPtr, b: VertPtr) -> Option<()> {
        self.add_edge(a, b, Vec3::Z, Vec3::Z)
    }

    // build the edge twins themselves
    // initialize them fully correct, but only pointing to each other
    // does not fire when this edge already exists
    fn add_dangling_twins(&mut self, a: VertPtr, b: VertPtr) -> Option<(EdgePtr, EdgePtr)> {
        // TODO add cases were one of the two does already exist
        // for now, quit if any one exist
        if self.has_half_edge(a, b) || self.has_half_edge(b, a) {
            return None;
        }

        // we only know the 'from_b array' after adding both, so start as 0
        let from_a: EdgePtr = self.edges.push(HalfEdge {
            from: a,
            next: 0,
            twin: 0,
            face: None,
        });
        let from_b = self.edges.push(HalfEdge {
            from: b,
            next: from_a,
            twin: from_a,
            face: None,
        });
        let a = self.mut_edge(from_a);
        a.twin = from_b;
        a.next = from_b;

        Some((from_a, from_b))
    }

    // from the disk of edges surrounding `vp`, get the two 'neighboring edges', based on some sample vector
    fn get_disk_neighbors(
        &self,
        vp: VertPtr,
        sample: Vec3,
        normal: Vec3,
    ) -> Option<(EdgePtr, EdgePtr)> {
        let vert = self.vert(vp);
        let disk_edges: Vec<EdgePtr> = self.get_disk(vp);
        let inc_disk_edges: Vec<EdgePtr> = disk_edges
            .iter()
            .skip(1)
            .step_by(2)
            .map(|u| u.clone())
            .collect();
        let neighbors = inc_disk_edges
            .iter()
            .map(|ep| self.vert(self.edge(*ep).from).pos);
        let nb_vecs: Vec<Vec3> = neighbors.map(|nb| nb - vert.pos).collect();

        // based on disk ordering, figure out which two incoming edges are in between the addition
        let between_ids: (usize, usize) =
            math::get_vectors_between(vert.pos, normal, nb_vecs, sample)?;
        let between = (inc_disk_edges[between_ids.0], inc_disk_edges[between_ids.1]);

        // we want to return the incoming and connected outgoing edge based
        if self.edge(self.edge(between.0).next).twin == between.1 {
            // normal
            Some((between.0, self.edge(between.0).next))
        } else if self.edge(self.edge(between.1).next).twin == between.0 {
            // reversed
            println!("WARN: technically not correct! this means 'get vectors in between' needs to be flipped");
            Some((between.1, self.edge(between.1).next))
        } else {
            println!("ERR: something went wrong in halfedge disk ordering...");
            None
        }
    }

    /// get the edges, lined as a disk around a vertex.
    /// starts with the outgoing edge the vertex points to
    /// With the half-edge loops being counter clockwise, disk ordering is always clockwise
    fn get_disk(&self, vp: VertPtr) -> Vec<EdgePtr> {
        let Some(start) = self.vert(vp).edge else {
            return Vec::new();
        };
        let mut disk = Vec::new();
        let mut cursor = start;
        loop {
            let outc = self.edge(cursor);
            let inc = self.edge(outc.twin);
            disk.push(cursor);
            disk.push(outc.twin);
            cursor = inc.next;
            if cursor == start {
                break;
            }
        }
        disk
    }

    /// set the `next` property of a given edge by adding the edge to the conceptual 'disk' of a vertex
    /// set the `vert` property of a vertex to the first added edge
    fn add_edge_to_vertex(&mut self, ep: EdgePtr, normal: Vec3) {
        // pointer business
        let ep_outwards = ep;
        let vp = self.edge(ep).from;
        let ep_inwards = self.edge(ep).twin;
        let v = self.mut_vert(vp);
        if v.edge.is_none() {
            // we are the first edge to be added to this vertex
            v.edge = Some(ep);
            self.mut_edge(ep_inwards).next = ep;
        } else if let Some(disk_start) = v.edge {
            // ep_outwards
            let (from, to) = self.get_edge_verts(ep);
            let incoming = to - from;
            let Some((ep_nb_inwards, ep_nb_outwards)) = self.get_disk_neighbors(vp, incoming, normal) else {
                return;
            };

            self.mut_edge(ep_inwards).next = ep_nb_outwards;
            self.mut_edge(ep_nb_inwards).next = ep_outwards;
        }
    }

    /////////////////////////////////////////////////////////////// Movement

    /////////////////////////////////////////////////////////////// Modelling

    fn divide_edge(&mut self, ep: EdgePtr, t: f32) {
        todo!()
    }

    fn divide_face(&mut self, ep: FacePtr, t: f32) {
        todo!()
    }
    
    fn subdivide(&mut self) {

        // halfway_pt = subdivide every edge by creating a new point halfway 
        // - subdivide, store faceptr on the left and right

        // face_pt = add a point at the center of every face / loop
        // - add pts, store with the faceptr

        // edge = create new edges between every halfway point, and the two adjacent face points
        // - draw lines between 
        todo!()
    }

    /// cap closed planar holes by creating faces at these holes.
    fn cap(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::solid::{Polyhedron, VertPtr};
    use glam::{vec3, Vec3};

    #[test]
    fn polyhedron() {
        let mut ph = Polyhedron::new();
        let a = ph.add_vert(vec3(1.0, 0.0, 0.0));
        let b = ph.add_vert(vec3(1.0, 1.0, 0.0));
        let c = ph.add_vert(vec3(0.0, 0.0, 1.0));
        assert_eq!(a, 0);
        assert_eq!(b, 1);
        assert_eq!(c, 2);

        const UP: Vec3 = Vec3::Z;
        let p = ph.add_planar_edge(a, b);
        let p = ph.add_planar_edge(b, c);
        let p = ph.add_planar_edge(c, a);

        ph.print_structure();

        let mut hedron = Polyhedron::new();

        let a: VertPtr = hedron.add_vert(vec3(0., 0., 0.));
        let b: VertPtr = hedron.add_vert(vec3(1., 0., 0.));
        let c: VertPtr = hedron.add_vert(vec3(1., 1., 0.));
        let d: VertPtr = hedron.add_vert(vec3(0., 1., 0.));

        hedron.add_planar_edge(a, b);
        hedron.add_planar_edge(b, c);
        hedron.add_planar_edge(c, d);
        hedron.add_planar_edge(d, a);
        hedron.add_planar_edge(a, c);

        hedron.print_structure();
    }
}
