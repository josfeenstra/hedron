use rand::seq::SliceRandom;

use super::Mesh;
use crate::kernel::{fxx, vec3, Vec3};
use crate::util::{iter_pairs, iter_triplets};
use crate::various::{get_smoothers_quad_to_square, get_smoothers_quad_to_square_at_length};
use crate::{
    core::PointBased,
    data::{Pool, Ptr},
    planar::Polygon,
    pts::Vectors,
};
use std::collections::HashSet;

pub type VertPtr = Ptr;
pub type EdgePtr = Ptr;
pub type FacePtr = Ptr;

/// A vertex of the graph
#[derive(Default, Debug)]
pub struct Vert {
    pub pos: Vec3,
    pub edge: Option<EdgePtr>, // I believe this Edge is always incoming
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
    /// the origin vertex
    pub next: EdgePtr, // edge always has a next
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

    /// read a string as an obj file format. If the string can't be interpreted, an empty Mesh will be returned
    /// TODO test this!
    pub fn from_obj_str(string: &str, flip_y_z: bool) -> Self {
        let mut hedron = Self::default();

        for line in string.lines().map(|l| l.trim()) {
            let parts: Vec<_> = line.split_whitespace().collect();
            if parts.len() == 0 {
                continue;
            }
            match parts[0] {
                "#" => continue,
                "v" => {
                    let num = parts
                        .into_iter()
                        .skip(1)
                        .filter_map(|s| s.parse::<fxx>().ok())
                        .collect::<Vec<_>>();
                    // mesh.verts.push()
                    if num.len() != 3 {
                        println!("mistake in obj vertices...");
                        continue;
                    }
                    if flip_y_z {
                        hedron.add_vert(Vec3::new(num[0], num[2], num[1]));
                    } else {
                        hedron.add_vert(Vec3::new(num[0], num[1], num[2]));
                    };
                }
                "f" => {
                    let num = parts
                        .into_iter()
                        .skip(1)
                        .filter_map(|s| s.parse::<usize>().ok())
                        .collect::<Vec<_>>();
                    for (a, b, c) in iter_triplets(&num) {
                        // to insert the face properly, we need some normal. Luckely, we can extract that
                        let va = hedron.vert(*a).pos;
                        let vb = hedron.vert(*b).pos;
                        let vc = hedron.vert(*c).pos;

                        let normal = (vb - va).cross(vc - va);
                        hedron.add_edge(*a, *b, normal, normal);
                    }
                    // mesh.tri.
                    // mesh.verts.push(Vec3::new(num[0], num[1], num[2]));
                }
                _ => continue,
            };
        }

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

        for (ia, ib, ic) in mesh.get_triangles().into_iter() {
            
            let (a, b, c) = (
                hedron.verts.get(ia).expect("the mesh pointers should work"),
                hedron.verts.get(ib).expect("the mesh pointers should work"),
                hedron.verts.get(ic).expect("the mesh pointers should work"),
            );
            // get triangle normal
            
            // assume counter clockwise CHECK THIS rotation
            let face_normal = (b.pos - a.pos).cross(c.pos - a.pos).normalize();
            
            
            // add half edges in accordance with this normal
            for [from, to] in vec![[ia, ib], [ib, ic], [ic, ia]] {
                // println!("adding edge between: {from} and {to}");
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

    /// convert to polygon faces
    pub fn all_cww_loops_as_polygons(&self) -> Vec<Polygon> {
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
        self.get_loops()
            .iter()
            .map(|lp| {
                Polygon::new(
                    lp.iter()
                        .map(|ep| self.vert(self.edge(*ep).from).pos)
                        .collect(),
                )
            })
            .filter(|p| p.signed_area() < 0.0)
            .collect()
    }

    ///
    pub fn all_unique_edges(&self) -> Vec<EdgePtr> {
        self.edges
            .iter_enum()
            .filter_map(|(i, e)| if e.twin < i { Some(i) } else { None })
            .collect()
    }

    pub fn all_verts(&self) -> Vec<Vec3> {
        self.verts.iter().map(|v| v.pos).collect()
    }

    pub fn iter_verts_mut(&self) -> Vec<Vec3> {
        todo!();
    }

    pub fn edge_verts(&self, ep: EdgePtr) -> (Vec3, Vec3) {
        let e = self.edge(ep);
        let t = self.edge(e.twin);
        (self.vert(e.from).pos, self.vert(t.from).pos)
    }

    pub fn get_all_debug_lines(&self) -> Vec<Vec3> {
        todo!()
    }

    /////////////////////////////////////////////////////////////// Transactions

    // this is not a very rusty way of doing things, but come on, I need some progress :)

    #[inline]
    pub fn edge(&self, ep: EdgePtr) -> &HalfEdge {
        self.edges.get(ep).expect("edge ptr not found!")
    }

    #[inline]
    pub fn vert(&self, vp: VertPtr) -> &Vert {
        self.verts.get(vp).expect("vert ptr not found!")
    }

    #[inline]
    pub fn mut_edge(&mut self, ep: EdgePtr) -> &mut HalfEdge {
        self.edges.get_mut(ep).expect("edge ptr not found!")
    }

    #[inline]
    pub fn mut_vert(&mut self, vp: VertPtr) -> &mut Vert {
        self.verts.get_mut(vp).expect("vert ptr not found!")
    }

    pub fn add_vert(&mut self, pos: Vec3) -> VertPtr {
        self.verts.push(Vert { pos, edge: None }) as VertPtr
    }



    /// NOTE: A - B is a different half edge than B - A.
    fn has_half_edge(&self, start: VertPtr, end: VertPtr) -> bool {
        // println!("edges around {start} are {:?}", self.get_disk(start));
        self.get_disk(start).iter().any(|ep| self.edge(*ep).from == end) // these are checked twice as many times as needed
    }

    /// delete an half-edge and its twin
    pub fn delete_edge(&mut self, ep: EdgePtr) {
        let twin = self.edge(ep).twin;
        let ep_next = self.edge(ep).next;
        let twin_next = self.edge(twin).next;

        let vert_a = self.edge(twin).from;
        let vert_b = self.edge(ep).from;

        // FIRST take care of faces
        if self.edge(ep).face != None {
            todo!();
        } 
        if self.edge(twin).face != None {
            todo!();
        } 

        // SECOND: take care of vertex ptrs
        if ep_next == twin {
            self.mut_vert(vert_a).edge = None;  // vert a will become a dangling vertex 
        } else {
            self.mut_vert(vert_a).edge = Some(ep_next); // vert must not point to the edge about to be deleted
        }
        if twin_next == ep {
            self.mut_vert(vert_b).edge = None; // vert a will become a dangling vertex 
        } else {
            self.mut_vert(vert_b).edge = Some(twin_next); // vert must not point to the edge about to be deleted
        }

        // THIRD: take care of edge ptrs: find the previous edge, and edit the disk
        let mut a_replaced = false;
        let mut b_replaced = false;
        for disk_edge in self.get_disk(vert_a) {
            if self.edge(disk_edge).next == twin { 
                a_replaced = true;
                self.mut_edge(disk_edge).next = ep_next;
                break;
            }
        }
        for disk_edge in self.get_disk(vert_b) {
            if self.edge(disk_edge).next == ep { 
                b_replaced = true;
                self.mut_edge(disk_edge).next = twin_next;
                break;
            }
        }

        if !a_replaced {
            println!("a did not get replaced!");
        }
        if !b_replaced {
            println!("b did not get replaced!");
        }

        // LASTLY: actually delete them
        self.edges.delete(ep);
        self.edges.delete(twin);
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
            // println!("Already exists!");
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

    fn get_vert_neighbors(&self, vp: VertPtr) -> Vec<VertPtr> {
        let vert = self.vert(vp);
        let disk_edges: Vec<EdgePtr> = self.get_disk(vp);
        let neighbors: Vec<VertPtr> = disk_edges
            .iter()
            .skip(1)
            .step_by(2)
            // .map(|u| u.clone())
            .map(|ep| self.edge(*ep).from)
            .collect();
        neighbors
    }

    // from the disk of edges surrounding `vp`, get the two 'neighboring edges', based on some sample vector
    fn get_disk_neighbors_edges(
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
            .map(|ep| self.vert(self.edge(*ep).from).pos)
            .collect();
        // let nb_vecs: Vec<Vec3> = neighbors.map(|nb| nb - vert.pos).collect();

        // println!("disk v{vp}: edges: {inc_disk_edges:?} vecs: {neighbors:?} sample: {sample}");

        // based on disk ordering, figure out which two incoming edges are in between the addition
        // NOTE: (1, 4) is not the same as (4, 1)
        let between_ids: (usize, usize) =
            Vectors::get_between(vert.pos, normal, neighbors, sample)?;
        let between = (inc_disk_edges[between_ids.0], inc_disk_edges[between_ids.1]);

        // println!("between: {between:?}");

        // we want to return the incoming and connected outgoing edge based
        if self.edge(self.edge(between.0).next).twin == between.1 {
            // normal
            Some((between.0, self.edge(between.0).next))
        } else {
            println!("ERR: something went wrong in halfedge disk ordering: between: {between:?} is incorrect");
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
            let (_, to) = self.edge_verts(ep);
            let Some((ep_nb_inwards, ep_nb_outwards)) = self.get_disk_neighbors_edges(vp, to, normal) else {
                return;
            };
            // println!("in: {ep_nb_inwards} out: {ep_nb_outwards}");

            self.mut_edge(ep_inwards).next = ep_nb_outwards;
            self.mut_edge(ep_nb_inwards).next = ep_outwards;
        }
    }

    /////////////////////////////////////////////////////////////// Movement

    /////////////////////////////////////////////////////////////// Modelling

    /// split an edge at normalized parameter t, starting from the starting vert of the given edge.
    pub fn split_edge(&mut self, ep: EdgePtr, t: fxx) -> (VertPtr, EdgePtr, EdgePtr) {
        let ep_top = ep;
        let ep_top_next = self.edge(ep_top).next;
        let ep_top_face = self.edge(ep_top).face;
        let ep_bottom = self.edge(ep_top).twin;
        let ep_bottom_next = self.edge(ep_bottom).next;
        let ep_bottom_face = self.edge(ep_bottom).face;

        let vp_start = self.edge(ep).from;
        let vp_end = self.edge(self.edge(ep).next).from;

        // create the new material
        let vp_new = self.add_vert(self.vert(vp_start).pos.lerp(self.vert(vp_end).pos, t));
        let ep_top_extended = self.edges.push(HalfEdge {
            from: vp_new,
            next: ep_top_next,
            twin: ep_bottom,
            face: ep_top_face,
        });
        let ep_bottom_extended = self.edges.push(HalfEdge {
            from: vp_new,
            next: ep_bottom_next,
            twin: ep_top,
            face: ep_bottom_face,
        });

        // set correct next and twin edge
        let e_top = self.mut_edge(ep_top);
        e_top.next = ep_top_extended;
        e_top.twin = ep_bottom_extended;
        let e_bottom = self.mut_edge(ep_bottom);
        e_bottom.next = ep_bottom_extended;
        e_bottom.twin = ep_top_extended;

        // make sure the vertex points to one of its two incoming edges
        // TODO sketchy, for some reason, this has to point to an outgoing edge.
        // TODO formalize this, check if this is the case everywhere
        self.mut_vert(vp_new).edge = Some(ep_top_extended);

        // return new material
        (vp_new, ep_top_extended, ep_bottom_extended)
    }

    pub fn split_face(&mut self, ep: FacePtr, t: fxx) {
        todo!()
    }

    /// subdivide by creating quads from all polyhedrons
    pub fn quad_divide(&mut self) {
        // get center points, normal, and original start edges for all loops
        let faces_data: Vec<(Vec3, Vec3, usize)> = self
            .get_loops()
            .iter()
            .filter_map(|edges| {
                let first_edge = edges[0];
                let verts = self.edges_to_verts(&edges);
                let center = Vectors::average(&verts);
                let signed_area = Polygon::new(verts).signed_area();
                if signed_area > 0.0 {
                    None
                } else {
                    let normal = Vec3::Z; // TODO CREATE A GOOD NORMAL!
                                          // NOTE: WE ALSO NEED A GOOD NORMAL FOR THE SIGNED AREA TEST.
                                          // println!("{:?}", (center, normal, first_edge));
                    Some((center, normal, first_edge))
                }
            })
            .collect();

        // subdivide all edges
        for edge in self.all_unique_edges() {
            self.split_edge(edge, 0.5);
        }

        // then build new faces
        for (i, (center, normal, first_edge)) in faces_data.into_iter().enumerate() {
            let vp = self.add_vert(center);
            for edge in self.get_loop(first_edge).iter().skip(1).step_by(2) {
                // NOTE: we can do this faster and without disk thingies, if we just carefully edit pointers
                self.add_edge(vp, self.edge(*edge).from, normal, normal); // TODO FIX EDGE ORDERING MISTAKES :)
                                                                          // break;
            }
        }
    }

    /// cap closed planar holes by creating faces at these holes.
    pub fn cap_ccw_holes(&mut self) {
        let loops = self.get_loops();
        for lp in loops {
            let pts: Vec<Vec3> = self.edges_to_verts(&lp);
            let area = Polygon::new(pts).signed_area();
            if area > 0.0 {
                continue;
            }
            todo!("add edges")
        }
    }

    pub fn edges_to_verts(&self, edges: &Vec<EdgePtr>) -> Vec<Vec3> {
        edges
            .iter()
            .map(|edgeptr| self.vert(self.edge(*edgeptr).from).pos)
            .collect()
    }

    pub fn all_face_loops() {}
}

impl PointBased for Polyhedron {
    fn mutate_points<'a>(&'a mut self) -> Vec<&'a mut Vec3> {
        self.verts.iter_mut().map(|v| &mut v.pos).collect()
    }
}

/// Here I put very specific polyhedron operations
impl Polyhedron {

    /// return the number of iterations upon exhaustion, or None if max_iterations was reached 
    pub fn make_random_quads(&mut self) -> Option<usize> {
        let mut rng = rand::thread_rng(); 
        for i in 0..1_000_000 {
            let edges = self.edges.all_ids();
            let edges_between_triangles: Vec<_> = edges.into_iter().filter(|ep| {
                let twin = self.edge(*ep).twin;
                // `ep < twin` to filter out half the half edges
                *ep < twin && self.get_loop(*ep).len() == 3 && self.get_loop(twin).len() == 3    
            }).collect();

            let Some(ep) = edges_between_triangles.choose(&mut rng) else {
                return Some(i);
            };
            self.delete_edge(*ep);
        }
        None
    }

    pub fn quad_smooth_planar_partition(&mut self, normal: Vec3, length: fxx) {
        
        let loops = self.get_loops();
        let polygons = loops.iter().map(|lp| 
            Polygon::new(lp.iter()
                .map(|ep| self.vert(self.edge(*ep).from).pos)
                .collect()
            )).collect::<Vec<_>>();
        let bad_apples = polygons.iter().map(|p| p.signed_area() > 0.0);

        // TODO uphold edge length by pushing out elastically on neighboring verts
        for vp in self.verts.all_ids() {
            let nbs_pos = self.get_vert_neighbors(vp).iter().map(|nb| self.vert(*nb).pos).collect();
            let avg = Vectors::average(&nbs_pos);
            self.mut_vert(vp).pos = avg;
        }


        // for ((lp, polygon), bad_apple) in loops.iter().zip(polygons.iter()).zip(bad_apples) {
        //     if bad_apple { continue; }
        //     let quad: [Vec3; 4] = polygon.verts.clone().try_into().expect("not a quad!");

        //     let smoothers = get_smoothers_quad_to_square(&quad, normal);
        //     // let smoothers = get_smoothers_quad_to_square_at_length(&quad, normal, length);
        //     for (edge, smoother) in lp.iter().zip(smoothers) {
        //         self.mut_vert(self.edge(*edge).from).pos += smoother;
        //     } 
        // }
        
    }
}

#[cfg(test)]
mod tests {
    use crate::kernel::{vec3, Vec3};
    use crate::solid::{Polyhedron, VertPtr};

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
