#![allow(unused_variables, dead_code)]

use super::{quad_to_tri, Octoid, Polyhedron, CUBE_FACES};
use crate::kernel::{fxx, kernel, vec2, vec3, Vec2, Vec3};
use crate::{prelude::*, util};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::iter::Rev;
use std::ops::Range;

#[derive(Debug, Clone, Default)]
pub enum Normals {
    #[default]
    None,
    Vertex(Vec<Vec3>),
    Face(Vec<Vec3>),
}

/// A dead simple, internal data structure to store meshes.
/// Can get confusing in conjunction with bevy's mesh
#[derive(Debug, Clone)]
pub struct Mesh {
    pub verts: Vec<Vec3>,
    pub tri: Vec<usize>, // TODO Index Enum
    pub uvs: Vec<Vec2>,
    pub normals: Normals,
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            verts: Default::default(),
            tri: Default::default(),
            uvs: Default::default(),
            normals: Normals::None,
        }
    }
}

impl Mesh {
    pub fn new(verts: Vec<Vec3>, tri: Vec<usize>, uvs: Vec<Vec2>, normals: Normals) -> Self {
        Self {
            verts,
            tri,
            uvs,
            normals,
        }
    }

    pub fn with_uvs(mut self, uvs: Vec<Vec2>) -> Self {
        self.uvs = uvs;
        self
    }

    pub fn with_uniform_uvs(mut self, uv: impl Into<Vec2>) -> Self {
        let uv = uv.into();
        self.uvs.clear();
        for i in 0..self.verts.len() {
            self.uvs.push(uv);
        }
        self
    }

    pub fn with_normals(mut self, normals: Normals) -> Self {
        self.normals = normals;
        self
    }

    /// bevy workaround
    pub fn with_dummy_normals(mut self) -> Self {
        self.normals = Normals::Vertex(self.verts.clone());
        self
    }

    pub fn double_sided(self) -> Self {
        Mesh::from_join(vec![self.clone().flip(), self])
    }

    pub fn append_normals(&mut self, other_normals: &mut Normals) {
        match (&mut self.normals, other_normals) {
            (Normals::Face(normals), Normals::Face(others)) => {
                normals.append(others);
            }
            (Normals::Vertex(normals), Normals::Vertex(others)) => {
                normals.append(others);
            }
            (Normals::None, Normals::None) => {
                //
            }
            _ => {
                println!("WARN: Skipping append normals with non-uniform normal type!");
            }
        }
    }

    pub fn calc_flat_face_normals(&self) -> Vec<Vec3> {
        self.iter_triangle_verts()
            .map(|(a, b, c)| Triangle::new(a, b, c).normal())
            .collect::<Vec<_>>()
    }

    pub fn calc_vertex_normals(&self) -> Vec<Vec3> {
        let flat_face_normals = self.calc_flat_face_normals();
        let mut buckets: Vec<Vec<Vec3>> = (0..self.verts.len()).map(|_| Vec::new()).collect();

        for (face_id, (ia, ib, ic)) in self.iter_triangles().enumerate() {
            for index in [ia, ib, ic] {
                buckets[index].push(flat_face_normals[face_id]);
            }
        }

        buckets
            .into_iter()
            .map(|bucket| Vectors::average(&bucket).normalize())
            .collect()
    }

    pub fn with_face_normals(mut self) -> Self {
        self.normals = Normals::Face(self.calc_flat_face_normals());
        self
    }

    pub fn with_vertex_normals(mut self) -> Self {
        self.normals = Normals::Vertex(self.calc_vertex_normals());
        self
    }

    pub fn count_triangles(&self) -> usize {
        self.tri.len() / 3
    }

    pub fn iter_triangles(&self) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
        (0..self.tri.len())
            .step_by(3)
            .map(|i| (self.tri[i], self.tri[i + 1], self.tri[i + 2]))
    }

    pub fn iter_triangle_verts(&self) -> impl Iterator<Item = (Vec3, Vec3, Vec3)> + '_ {
        self.iter_triangles()
            .map(|(a, b, c)| (self.verts[a], self.verts[b], self.verts[c]))
    }

    pub fn iter_edges(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.iter_triangles()
            .flat_map(|(a, b, c)| [(a, b), (b, c), (c, a)])
    }

    // pub fn get_triangles(&self) -> Vec<(usize, usize, usize)> {
    //     let mut data = Vec::new();
    //     assert!(self.tri.len() % 3 == 0);
    //     for i in (0..self.tri.len()).step_by(3) {
    //         data.push((self.tri[i], self.tri[i + 1], self.tri[i + 2]))
    //     }
    //     data
    // }

    // pub fn get_edges(&self) -> Vec<(usize, usize)> {
    //     let tri = self.get_triangles();
    //     let mut edges = Vec::new();
    //     for (a, b, c) in tri {
    //         edges.push((a, b));
    //         edges.push((b, c));
    //         edges.push((c, a));
    //     }
    //     edges
    // }

    /// an edge is naked if the same sequence cannot be found

    pub fn linearize(self) -> Self {
        todo!();
    }

    pub fn get_normals(&self) -> Option<&Vec<Vec3>> {
        match &self.normals {
            Normals::Face(normals) | Normals::Vertex(normals) => Some(normals),
            Normals::None => None,
        }
    }

    pub fn get_normals_mut(&mut self) -> Option<&mut Vec<Vec3>> {
        match &mut self.normals {
            Normals::Face(normals) | Normals::Vertex(normals) => Some(normals),
            Normals::None => None,
        }
    }

    pub fn to_uniform(&self) -> Self {
        let desouped = TriMesh::desoupify(&self.verts, 0.001);

        // let mut uvs = Vec::new();
        let mut verts = Vec::new();
        let mut ids = Vec::new();
        // let mut normals = Vec::new();

        // the desouped vertices point to the old vertex list
        // for a new, unique vertex, i will be equal to id
        // for all others, id must be replaced with this i
        let mut map = HashMap::new();
        let mut new_vert_index = 0;
        for (i, id) in desouped.iter().enumerate() {
            if let Some(index) = map.get(id) {
                ids.push(*index);
            } else {
                map.insert(id, new_vert_index);
                ids.push(new_vert_index);
                verts.push(self.verts[*id]);
                // uvs.push(self.uvs.get(i).unwrap().clone());
                // normals.push(self.normals.get(i).unwrap().clone());
                new_vert_index += 1;
            }
        }

        Self::new(verts, ids, Vec::new(), Normals::None)
    }
}

impl Mesh {
    // various constructors

    pub fn from_bi_surface(_srf: BiSurface, _u_segments: usize, _v_segments: usize) -> Mesh {
        // returns vertices & indices of a flat grid
        // let uPoints = u_segments + 1;
        // let vPoints = v_segments + 1;

        todo!();

        // let verts = MultiVector3.new(uPoints * vPoints);
        // let links = new IntMatrix(uSegments * vSegments * 2, 3);

        // // create all positions
        // for (let u = 0; u < uPoints; u++) {
        //     for (let v = 0; v < vPoints; v++) {
        //         let i = u * vPoints + v;
        //         verts.set(i, srf.pointAt(u / uSegments, v / vSegments));
        //     }
        // }

        // // create all indices
        // // a---c
        // // | \ |
        // // b---d
        // for (let u = 0; u < uSegments; u++) {
        //     for (let v = 0; v < vSegments; v++) {
        //         let start_index = 2 * (u * vSegments + v);
        //         let a = u * uPoints + v;
        //         let b = a + vPoints;
        //         let c = a + 1;
        //         let d = b + 1;

        //         links.setRow(start_index, [a, b, d]);
        //         links.setRow(start_index + 1, [c, a, d]);
        //     }
        // }
        // Self::new(verts, links)
    }

    pub fn from_tri_surface(srf: TriSurface, segments: usize) -> Mesh {
        todo!()
    }

    pub fn from_rectangle(rect: Rectangle3) -> Mesh {
        // let verts = rect.getCorners();

        // // we cant handle quads yet
        // let faces: number[] = [];
        // faces.push(...quadToTri(cubeFaces[0]));
        // return this.fromLists(verts, faces);
        todo!();
    }

    pub fn new_cube(size: fxx) -> Self {
        Self::from_range(Range3::from_radius(size))
    }

    /// Jup I stole this from bevy :)
    /// TODO: rewrite this once Mesh & TriMesh are integrated 
    pub fn from_range(sp: Range3) -> Self {
        // suppose Y-up right hand, and camera look from +z to -z
        let vertices = &[
            // Front
            ([sp.x.start, sp.y.start, sp.z.end], [0., 0., 1.0], [0., 0.]),
            ([sp.x.end, sp.y.start, sp.z.end], [0., 0., 1.0], [1.0, 0.]),
            ([sp.x.end, sp.y.end, sp.z.end], [0., 0., 1.0], [1.0, 1.0]),
            ([sp.x.start, sp.y.end, sp.z.end], [0., 0., 1.0], [0., 1.0]),
            // Back
            ([sp.x.start, sp.y.end, sp.z.start], [0., 0., -1.0], [1.0, 0.]),
            ([sp.x.end, sp.y.end, sp.z.start], [0., 0., -1.0], [0., 0.]),
            ([sp.x.end, sp.y.start, sp.z.start], [0., 0., -1.0], [0., 1.0]),
            ([sp.x.start, sp.y.start, sp.z.start], [0., 0., -1.0], [1.0, 1.0]),
            // Right
            ([sp.x.end, sp.y.start, sp.z.start], [1.0, 0., 0.], [0., 0.]),
            ([sp.x.end, sp.y.end, sp.z.start], [1.0, 0., 0.], [1.0, 0.]),
            ([sp.x.end, sp.y.end, sp.z.end], [1.0, 0., 0.], [1.0, 1.0]),
            ([sp.x.end, sp.y.start, sp.z.end], [1.0, 0., 0.], [0., 1.0]),
            // Left
            ([sp.x.start, sp.y.start, sp.z.end], [-1.0, 0., 0.], [1.0, 0.]),
            ([sp.x.start, sp.y.end, sp.z.end], [-1.0, 0., 0.], [0., 0.]),
            ([sp.x.start, sp.y.end, sp.z.start], [-1.0, 0., 0.], [0., 1.0]),
            ([sp.x.start, sp.y.start, sp.z.start], [-1.0, 0., 0.], [1.0, 1.0]),
            // Top
            ([sp.x.end, sp.y.end, sp.z.start], [0., 1.0, 0.], [1.0, 0.]),
            ([sp.x.start, sp.y.end, sp.z.start], [0., 1.0, 0.], [0., 0.]),
            ([sp.x.start, sp.y.end, sp.z.end], [0., 1.0, 0.], [0., 1.0]),
            ([sp.x.end, sp.y.end, sp.z.end], [0., 1.0, 0.], [1.0, 1.0]),
            // Bottom
            ([sp.x.end, sp.y.start, sp.z.end], [0., -1.0, 0.], [0., 0.]),
            ([sp.x.start, sp.y.start, sp.z.end], [0., -1.0, 0.], [1.0, 0.]),
            ([sp.x.start, sp.y.start, sp.z.start], [0., -1.0, 0.], [1.0, 1.0]),
            ([sp.x.end, sp.y.start, sp.z.start], [0., -1.0, 0.], [0., 1.0]),
        ];

        let verts: Vec<Vec3> = vertices.iter().map(|(p, _, _)| (*p).into()).collect();
        let normals: Vec<Vec3> = vertices.iter().map(|(_, n, _)| (*n).into()).collect();
        let uvs: Vec<Vec2> = vertices.iter().map(|(_, _, uv)| (*uv).into()).collect();

        let indices = vec![
            0, 1, 2, 2, 3, 0, // front
            4, 5, 6, 6, 7, 4, // back
            8, 9, 10, 10, 11, 8, // right
            12, 13, 14, 14, 15, 12, // left
            16, 17, 18, 18, 19, 16, // top
            20, 21, 22, 22, 23, 20, // bottom
        ];

        Self::new(verts, indices, uvs, Normals::Vertex(normals))
    }
    
    

    /// will only convert triangular faces
    /// cap / triangulate before running this
    pub fn from_polyhedron(graph: Polyhedron) -> Mesh {
        let mut mesh = Mesh::default();

        let remapper = graph.verts.get_refactor_mapping();
        for v in graph.verts.iter() {
            mesh.verts.push(v.pos);
        }

        for lp in graph.get_face_loops() {
            if lp.len() != 3 {
                continue;
            }

            for edge in lp {
                let id = remapper
                    .get(&graph.edge(edge).from)
                    .expect("not a valid vert id");
                mesh.tri.push(*id);
            }
        }

        let mut normals = vec![];
        for (i, v) in graph.verts.iter_enum() {
            let normal = graph.vertex_normal_using_faces(i).unwrap_or(Vec3::ZERO);
            normals.push(normal);
        }
        mesh.normals = Normals::Vertex(normals);
        mesh
    }

    pub fn new_triangle(verts: [Vec3; 3]) -> Self {
        Triangle::new(verts[0], verts[1], verts[2]).into()
    }

    pub fn from_triangle(triangle: Triangle) -> Self {
        triangle.into()
    }

    pub fn quad_from_flat_plane(scale: fxx) -> Self {
        let verts = [
            Vec3::new(-scale, -scale, 0.0),
            Vec3::new(-scale, scale, 0.0),
            Vec3::new(scale, scale, 0.0),
            Vec3::new(scale, -scale, 0.0),
        ];
        let normal = Vec3::Z;

        Self::new(
            verts.to_vec(),
            [0, 3, 2, 2, 1, 0].to_vec(),
            vec![],
            Normals::Vertex(vec![normal, normal, normal, normal]),
        )
    }

    pub fn quad_from_plane(plane: Plane, scale: fxx) -> Self {
        let verts = [
            Vec3::new(-scale, -scale, 0.0),
            Vec3::new(-scale, scale, 0.0),
            Vec3::new(scale, scale, 0.0),
            Vec3::new(scale, -scale, 0.0),
        ]
        .iter()
        .map(|v| plane.point_to_world(*v))
        .collect();

        let normal = plane.normal();

        Self::new(
            verts,
            [0, 1, 2, 2, 3, 0].to_vec(),
            vec![],
            Normals::Face(vec![normal, normal]),
        )
    }

    pub fn new_quad(verts: [Vec3; 4]) -> Self {
        Self::new(
            verts.to_vec(),
            [0, 1, 2, 0, 2, 3].to_vec(),
            vec![],
            Normals::None,
        )
    }

    pub fn new_oct(verts: [Vec3; 8]) -> Self {
        let ids = CUBE_FACES
            .into_iter()
            .map(quad_to_tri)
            .fold(Vec::new(), |mut tris, idset| {
                tris.extend(idset);
                tris
            });
        Self::new(verts.to_vec(), ids, vec![], Normals::None)
    }

    pub fn from_octoid(oct: Octoid) -> Self {
        Self::new_oct(oct.verts)
    }

    pub fn new_icosahedron(scale: fxx) -> Self {
        let graph = Polyhedron::new_icosahedron(scale);
        Self::from_polyhedron(graph)
    }

    // // TODO remove center. Just move afterwards
    // static newSphere(center: Vector3, radius: number, numRings: number, resolution: number): Mesh {
    //     // verts
    //     let vertCount = numRings * resolution + 2;
    //     let verts = MultiVector3.new(vertCount);
    //     let setVert = function (i: number, vector: Vector3) {
    //         verts.set(i, vector.scale(radius).add(center));
    //     };

    //     setVert(0, new Vector3(0, 0, 1));
    //     for (let ring = 0; ring < numRings; ring++) {
    //         for (let perRing = 0; perRing < resolution; perRing++) {
    //             let alpha = (Math.PI * (ring + 1)) / (numRings + 1);
    //             let beta = (2 * Math.PI * perRing) / resolution;

    //             let x = Math.sin(alpha) * Math.cos(beta);
    //             let y = Math.sin(alpha) * Math.sin(beta);
    //             let z = Math.cos(alpha);

    //             let index = 1 + ring * resolution + perRing;
    //             setVert(index, new Vector3(x, y, z));
    //         }
    //     }
    //     setVert(vertCount - 1, new Vector3(0, 0, -1));

    //     // faces
    //     let faceCount = resolution * numRings * 2;
    //     let links = new IntMatrix(faceCount, 3);
    //     links.fill(-1);
    //     let setFace = function (i: number, row: number[]) {
    //         links.setRow(i, row);
    //     };

    //     // faces top
    //     for (let i = 0; i < resolution; i++) {
    //         setFace(i, [0, i + 1, ((i + 1) % resolution) + 1]);
    //     }

    //     // faces middle
    //     // we are at this cursor
    //     // console.log("faces", faceCount);

    //     for (let ring = 0; ring < numRings - 1; ring++) {
    //         let vertCursor = resolution * ring + 1;
    //         let vertCursorBelow = vertCursor + resolution;

    //         for (let perRing = 0; perRing < resolution; perRing++) {
    //             let a = vertCursor + perRing;
    //             let b = vertCursor + ((perRing + 1) % resolution);

    //             let c = vertCursorBelow + perRing;
    //             let d = vertCursorBelow + ((perRing + 1) % resolution);

    //             let iFace = resolution + resolution * ring * 2 + perRing * 2;

    //             // console.log(iFace);
    //             setFace(iFace, [a, c, b]);
    //             setFace(iFace + 1, [c, d, b]);
    //         }
    //     }

    //     // faces bottom
    //     for (let i = 0; i < resolution; i++) {
    //         let iNext = (i + 1) % resolution;
    //         let last = vertCount - 1;

    //         let iFace = faceCount - resolution + i;

    //         let zero = vertCount - resolution - 1;
    //         let vertI = zero + i;
    //         let vertINext = zero + iNext;

    //         // console.log(iFace);
    //         // console.log("face", last, vertINext, vertI);

    //         setFace(iFace, [last, vertINext, vertI]);
    //     }

    //     return new Mesh(verts, links);
    // }

    // // TODO remove from & to, Just move the mesh afterwards
    // static newCylinder(from: Vector3, to: Vector3, radius: number, resolution: number): Mesh {
    //     let normal = to.subbed(from);

    //     let numVerts = resolution * 2 + 2;
    //     let numFaces = (numVerts - 2) * 2;
    //     let verts = MultiVector3.new(numVerts);

    //     // some dumb stuff
    //     let setVert = function (i: number, vector: Vector3) {
    //         verts.set(i, vector);
    //     };

    //     // planes to represent top & bottom
    //     let planeFrom = Plane.fromPN(from, normal);
    //     // console.log(planeFrom);

    //     let planeTo = Plane.fromPN(to, normal);
    //     // console.log(planeFrom);

    //     // verts 'from ring
    //     setVert(0, from);
    //     for (let i = 0; i < resolution; i++) {
    //         let v = new Vector3(
    //             Math.cos((Math.PI * 2 * i) / resolution),
    //             Math.sin((Math.PI * 2 * i) / resolution),
    //             0,
    //         ).scale(radius);

    //         v = planeFrom.matrix.multiplyVector(v);
    //         setVert(i + 1, v);
    //     }

    //     // verts 'to' ring
    //     let numVertsHalf = numVerts / 2;
    //     for (let i = 0; i < resolution; i++) {
    //         let v = new Vector3(
    //             Math.cos((Math.PI * 2 * i) / resolution),
    //             Math.sin((Math.PI * 2 * i) / resolution),
    //             0,
    //         ).scale(radius);

    //         v = planeTo.matrix.multiplyVector(v);
    //         setVert(numVertsHalf + i, v);
    //     }
    //     setVert(numVerts - 1, to);

    //     // start making links
    //     let links = new IntMatrix(numFaces, 3);
    //     links.fill(-1);
    //     let setFace = function (i: number, row: number[]) {
    //         links.setRow(i, row);
    //     };

    //     // set faces
    //     for (let i = 0; i < resolution; i++) {
    //         let a = 0;
    //         let b = 1 + i;
    //         let c = 1 + ((i + 1) % resolution);

    //         let d = numVerts - 1;
    //         let e = numVertsHalf + i;
    //         let f = numVertsHalf + ((i + 1) % resolution);

    //         setFace(i * 4, [a, c, b]);
    //         setFace(i * 4 + 1, [b, c, e]);
    //         setFace(i * 4 + 2, [c, f, e]);
    //         setFace(i * 4 + 3, [d, e, f]);
    //     }

    //     return new Mesh(verts, links);
    // }

    // // TODO remove center, just move afterwards
    // static newCone(center: Vector3, radius: number, height: number, resolution: number) {
    //     let numVerts = resolution + 2;
    //     let numFaces = resolution * 2;
    //     let verts = MultiVector3.new(numVerts);
    //     let setVert = function (i: number, vector: Vector3) {
    //         verts.set(i, vector.add(center));
    //     };
    //     let links = new IntMatrix(numFaces, 3);
    //     links.fill(-1);
    //     let setFace = function (i: number, row: number[]) {
    //         links.setRow(i, row);
    //     };

    //     // set verts
    //     setVert(0, new Vector3(0, 0, 0));
    //     for (let i = 0; i < resolution; i++) {
    //         setVert(
    //             i + 1,
    //             new Vector3(
    //                 Math.cos((Math.PI * 2 * i) / resolution),
    //                 Math.sin((Math.PI * 2 * i) / resolution),
    //                 0,
    //             ).scale(radius),
    //         );
    //     }
    //     setVert(numVerts - 1, new Vector3(0, 0, height));

    //     // set faces
    //     for (let i = 0; i < resolution; i++) {
    //         let a = 0;
    //         let b = numVerts - 1;
    //         let c = 1 + i;
    //         let d = 1 + ((i + 1) % resolution);

    //         setFace(i * 2, [a, d, c]);
    //         setFace(i * 2 + 1, [c, d, b]);
    //     }

    //     return new Mesh(verts, links);
    // }

    // static newTorus(r1: number, r2: number, ringCount: number, vertCount: number) {
    //     // verts * normals
    //     let count = ringCount * vertCount;
    //     let verts = MultiVector3.new(count);
    //     let normals = MultiVector3.new(count);

    //     // create `resolution` number of section rings
    //     for (let i = 0; i < ringCount; i++) {
    //         let alpha = (Math.PI * 2 * i) / ringCount;
    //         let ringCenter = Vector3.new(Math.cos(alpha) * r1, Math.sin(alpha) * r1, 0);

    //         // per section, create `sectionResolution` number of
    //         for (let j = 0; j < vertCount; j++) {
    //             let beta = (Math.PI * 2 * j) / vertCount;
    //             let normal = Vector3.new(
    //                 Math.cos(beta) * Math.cos(alpha),
    //                 Math.cos(beta) * Math.sin(alpha),
    //                 Math.sin(beta),
    //             ).normalize();

    //             normals.set(i * vertCount + j, normal);
    //             verts.set(i * vertCount + j, normal.scale(r2).add(ringCenter));
    //         }
    //     }

    //     // links & uvs
    //     let links = IntMatrix.new(count * 2, 3);
    //     let uvs = undefined;

    //     let getIndex = (i: number, j: number) => {
    //         return (i % ringCount) * vertCount + (j % vertCount);
    //     };

    //     for (let i = 0; i < ringCount; i++) {
    //         for (let j = 0; j < vertCount; j++) {
    //             let a = getIndex(i, j);
    //             let b = getIndex(i, j + 1);
    //             let c = getIndex(i + 1, j);
    //             let d = getIndex(i + 1, j + 1);

    //             let iRow = a * 2;

    //             links.setRow(iRow, [a, c, b]);
    //             links.setRow(iRow + 1, [b, c, d]);
    //         }
    //     }

    //     let mesh = Mesh.new(verts, links, uvs, normals);
    //     return mesh;
    // }
}

impl Mesh {
    pub fn from_polygon_naive(polygon: &Polygon) -> Mesh {
        let mut mesh = Mesh::default();

        let count = polygon.verts.len(); // the center will end up at this vert id
        for (a, b) in util::iter_pair_ids(count) {
            mesh.verts.push(polygon.verts[a]);
            mesh.tri.append(&mut vec![a, b, count]);
        }
        let center = Vectors::average(&polygon.verts);
        mesh.verts.push(center);

        mesh
    }

    pub fn from_extrude_polygon(polygon: Polygon, extrusion: Vec3) -> Mesh {
        let count = polygon.verts.len();

        let mut mesh = Mesh::from_join(
            [
                Mesh::from_polygon_naive(&polygon).flip(),
                Mesh::from_polygon_naive(&polygon).mv(extrusion),
            ]
            .into(),
        );

        // NOTE: a naive polygon triangulation puts an additional point in the middle
        let offset = count + 1;

        // fill the two rings of vertices with triangles
        for (i, j) in iter_pair_ids(count) {
            mesh.tri.append(&mut vec![i, j, j + offset]);
            mesh.tri.append(&mut vec![j + offset, i + offset, i]);
        }

        mesh
    }
}

// more convoluted and specific constructors
impl Mesh {
    // Get a grid mesh from weaving vertices of a grid.
    // The grid can deal with holes, use None vertices to indicate holes
    // We do this by finding 'valid cells' of 4 vertices, and lacing them like this:
    // Corners of 3 vertices are also created.
    // ```
    //  (d)----(c) .. ( )
    //   |  \   |      .
    //   |   \  |      .
    //  (b)----(a) .. ( )
    //   .      .      .
    //   .      .      .
    //  ( ) .. ( ) .. ( )
    // ```
    pub fn new_holed_weave(verts: Grid2<Option<(Vec3, Vec2)>>) -> Self {
        let mut mesh = Self::default();

        // add all verts, and dummy zero vectors at zero spots
        for (i, res) in verts.items.iter().enumerate() {
            let (x, y) = verts.to_xy(i);
            match res {
                Some((pos, uv)) => {
                    mesh.verts.push(*pos);
                    mesh.uvs.push(*uv);
                }
                None => {
                    mesh.verts.push(vec3(0., 0., 0.));
                    mesh.uvs.push(vec2(0., 0.));
                }
            }
        }

        // add triangles by lacing patches
        for (i, res) in verts.items.iter().enumerate() {
            let (x, y) = verts.to_xy(i);

            if x == 0 || y == 0 {
                continue;
            }

            let ia = i;
            let ib = verts.to_index(x - 1, y).unwrap();
            let ic = verts.to_index(x, y - 1).unwrap();
            let id = verts.to_index(x - 1, y - 1).unwrap();

            let a = verts.get_unsafe(ia);
            let b = verts.get_unsafe(ib);
            let c = verts.get_unsafe(ic);
            let d = verts.get_unsafe(id);

            // do some bitmask checking to get corner triangles
            // there is no elegant way to do this,
            // since we have to get the triangle counter-clockwise direction right
            if a.is_some() && b.is_some() && c.is_some() && d.is_some() {
                mesh.tri.append(&mut vec![ia, id, ib]);
                mesh.tri.append(&mut vec![ia, ic, id]);
            } else if a.is_some() && b.is_some() && c.is_some() {
                mesh.tri.append(&mut vec![ia, ic, ib]);
            } else if a.is_some() && b.is_some() && d.is_some() {
                mesh.tri.append(&mut vec![ia, id, ib]);
            } else if a.is_some() && c.is_some() && d.is_some() {
                mesh.tri.append(&mut vec![ia, ic, id]);
            } else if b.is_some() && c.is_some() && d.is_some() {
                mesh.tri.append(&mut vec![ib, ic, id]);
            }
        }

        // TODO clean away all unused vertices
        // return mesh.to_clean();
        mesh
    }

    /// a very simple shape for if you want to visualize some point with a mesh
    pub fn new_diamond(center: Vec3, size: fxx) -> Self {
        let mut mesh = Mesh::default();

        mesh.verts
            .push(Vec3::new(center.x + size, center.y, center.z));
        mesh.verts
            .push(Vec3::new(center.x - size, center.y, center.z));
        mesh.verts
            .push(Vec3::new(center.x, center.y + size, center.z));
        mesh.verts
            .push(Vec3::new(center.x, center.y - size, center.z));
        mesh.verts
            .push(Vec3::new(center.x, center.y, center.z + size));
        mesh.verts
            .push(Vec3::new(center.x, center.y, center.z - size));

        mesh.tri.append(&mut vec![
            4, 0, 2, 4, 2, 1, 4, 1, 3, 4, 3, 0, 5, 2, 0, 5, 1, 2, 5, 3, 1, 5, 0, 3,
        ]);

        mesh
    }

    // simple join, not taking common verts into account
    pub fn from_join(meshes: Vec<Mesh>) -> Mesh {
        let mut mesh = Mesh::default();

        // this is dumb
        match meshes.get(0) {
            Some(m) => match m.normals {
                Normals::Face(_) => {
                    mesh.normals = Normals::Face(vec![]);
                }
                Normals::Vertex(_) => {
                    mesh.normals = Normals::Vertex(vec![]);
                }
                _ => (),
            },
            None => (),
        }

        let mut vertcount = 0;
        for mut other in meshes {
            let length = other.verts.len();
            mesh.verts.append(&mut other.verts);
            mesh.uvs.append(&mut other.uvs);
            mesh.append_normals(&mut other.normals);
            mesh.tri
                .append(&mut other.tri.iter().map(|t| t + vertcount).collect());
            vertcount += length;
        }

        mesh
    }

    // create a mesh as a hexagonal
    pub fn new_hexagrid(radius: fxx, divisions: usize) -> Self {
        let mut mesh = Self::default();

        // get some ranges / iterations right
        let min_count = 2 + divisions;
        let max_count = min_count + 2 + divisions;

        let upper: Range<usize> = min_count..max_count;
        let lower: Rev<Range<usize>> = (min_count..max_count - 1).rev();
        let range: Vec<usize> = upper.clone().chain(lower).collect();

        // get some counters right for spawning the right grid of points
        let y_count = 3 + 2 * divisions;
        let y_offset = 1 + divisions;

        let dx = radius / (divisions as fxx + 1.0) * 2.0;
        let dy = dx * kernel::SQRT_OF_3;

        // verts
        for (i, steps) in range.iter().enumerate() {
            let x_start = (steps - 1) as fxx * dx;
            let y = (i as fxx - y_offset as fxx) * dy;
            for j in 0..*steps {
                let x = -x_start + (j as fxx * dx * 2.0);
                mesh.verts.push(Vec3::new(x, y, 0.0));
                // println!("({x}, {y})")
            }
        }

        // triangles
        let last = mesh.verts.len() - 1;
        let mut i = 0;
        let upper_steps: Vec<_> = upper.collect();
        for steps in min_count..max_count - 1 {
            for step in 0..steps {
                let a = i;
                let b = i + 1;
                let c = i + steps;
                let d = i + steps + 1;

                // radial mirror on the opposite side
                let aa = last - a;
                let bb = last - b;
                let cc = last - c;
                let dd = last - d;

                // only one triangle at te last segment of the strip
                if step > (steps - 2) {
                    mesh.tri.append(&mut vec![a, d, c]);
                    mesh.tri.append(&mut vec![aa, dd, cc]);
                } else {
                    mesh.tri.append(&mut vec![a, b, d]);
                    mesh.tri.append(&mut vec![aa, bb, dd]);
                    mesh.tri.append(&mut vec![a, d, c]);
                    mesh.tri.append(&mut vec![aa, dd, cc]);
                }

                i += 1;
            }
        }

        mesh
    }
}

impl Mesh {
    pub fn to_lines(&self) -> LineList {
        let mut lines = Vec::new();

        for edge in self.iter_edges() {
            let (a, b) = edge;
            lines.push(self.verts[a]);
            lines.push(self.verts[b]);
        }

        LineList::new(lines)
    }

    pub fn to_clean(&self) -> Mesh {
        // TODO: identify all vertices which are not references by any triangle,
        // exclude them
        // then update the triangle vertex pointers.
        todo!();
    }

    pub fn write_obj(&self, path: &str) -> Result<(), std::io::Error> {
        let obj = self.gen_obj_buffer("obj generated by Hedron", None, None)?;
        let mut obj_file = std::fs::File::create(path)?;
        obj_file.write_all(&obj)?;
        Ok(())
    }

    pub fn write_obj_mtl(
        &self,
        path: &str,
        name_obj: &str,
        name_mtl: &str,
        name_texture: &str,
    ) -> Result<(), std::io::Error> {
        let mat_name = "Material";

        // both the texture and mtl should be in the same folder
        let mtl = Mesh::gen_mtl_buffer("mtl generated by Hedron", mat_name, Some(name_texture))?;
        let obj = self.gen_obj_buffer("obj generated by Hedron", Some(mat_name), Some(name_mtl))?;

        let texture_path = path.to_owned() + name_texture;
        let mtl_path = path.to_owned() + name_mtl;
        let obj_path = path.to_owned() + name_obj;

        let mut obj_file = std::fs::File::create(obj_path)?;
        obj_file.write_all(&obj)?;
        let mut mtl_path = std::fs::File::create(mtl_path)?;
        mtl_path.write_all(&mtl)?;

        Ok(())
    }

    pub fn gen_mtl_buffer(
        header: &str,
        mat_name: &str,
        texture_path: Option<&str>,
    ) -> Result<Vec<u8>, std::io::Error> {
        let mut mtl = Vec::new();
        writeln!(&mut mtl, "# {}", header)?;
        writeln!(&mut mtl, "newmtl {}", mat_name)?;
        writeln!(&mut mtl, "Ns 250.000000")?;
        writeln!(&mut mtl, "Ka 1.000000 1.000000 1.000000")?;
        writeln!(&mut mtl, "Kd 0.000000 0.000000 0.000000")?;
        writeln!(&mut mtl, "Ks 0.000000 0.000000 0.000000")?;
        writeln!(&mut mtl, "Ke 0.000000 0.000000 0.000000")?;
        writeln!(&mut mtl, "Ni 1.450000")?;
        writeln!(&mut mtl, "d 1.000000")?;
        writeln!(&mut mtl, "illum 2")?;
        if let Some(path) = texture_path {
            writeln!(&mut mtl, "map_Ka {}", texture_path.unwrap())?;
            writeln!(&mut mtl, "map_Kd {}", texture_path.unwrap())?;
            writeln!(&mut mtl, "map_Ks {}", texture_path.unwrap())?;
        }
        Ok(mtl)
    }

    pub fn gen_obj_buffer(
        &self,
        header: &str,
        mat_name: Option<&str>,
        mtl_path: Option<&str>,
    ) -> Result<Vec<u8>, std::io::Error> {
        let mut obj = Vec::new();
        let o = &mut obj;

        writeln!(o, "# {}", header)?;

        if mtl_path.is_some() && mat_name.is_some() {
            writeln!(o, "mtllib {}", mtl_path.unwrap())?;
            writeln!(o, "usemtl {}", mat_name.unwrap())?;
        }
        for vert in self.verts.iter() {
            writeln!(o, "v {} {} {}", vert.x, vert.y, vert.z)?;
        }
        for uv in self.uvs.iter() {
            writeln!(o, "vt {} {}", uv.x, uv.y)?;
        }

        if self.uvs.len() == self.verts.len() {
            for (a, b, c) in self.iter_triangles() {
                let (a, b, c) = (a + 1, b + 1, c + 1);
                writeln!(o, "f {a}/{a} {b}/{b} {c}/{c}")?;
            }
        } else {
            for (a, b, c) in self.iter_triangles() {
                let (a, b, c) = (a + 1, b + 1, c + 1);
                writeln!(o, "f {a} {b} {c}")?;
            }
        }
        Ok(obj)
    }

    /// rename to transform_to_oct
    /// Transform a mesh in R(0..1) space towards
    pub fn transform_within_oct(mut self, oct: &Octoid) -> Self {
        for vert in &mut self.verts {
            *vert = oct.tri_lerp(*vert)
        }

        if let Some(normals) = self.get_normals_mut() {
            for n in normals {
                *n = oct.tri_lerp_normal(*n);
            }
        }

        self
    }
}

/// The real modelling tools
impl Mesh {
    /// flip the full mesh by swapping triangle orders
    pub fn flip(mut self) -> Self {
        for i in (0..self.tri.len()).step_by(3) {
            self.tri.swap(i, i + 1)
        }
        self
    }

    pub fn extrude(base: &Mesh, extrusion: Vec3) -> Mesh {
        let offset = base.verts.len();
        let mut mesh = Mesh::from_join([base.clone().flip(), base.clone().mv(extrusion)].into());

        // NOTE: a naive polygon triangulation puts an additional point in the middle

        // fill the two rings of vertices with triangles
        for (i, j) in base.iter_naked_edges() {
            mesh.tri.append(&mut vec![i, j, j + offset]);
            mesh.tri.append(&mut vec![j + offset, i + offset, i]);
        }

        mesh
    }

    /// assumes all curves are of the same length!!!
    #[rustfmt::skip]
    pub fn loft(mut curves: Vec<Vec<Vec3>>) -> Mesh {
        let mut mesh = Mesh::default();
        let curve_count = curves.len();
        let count = curves[0].len();
        for mut curve in &mut curves {
            mesh.verts.append(&mut curve);
        }

        // fill the two rings of vertices with triangles

        for ii in 0..curve_count-1 {

            let base = ii * count;

            for (i, j) in iter_pair_ids(count) {
                mesh.tri
                    .append(&mut vec![
                        base + i,
                        base + j,
                        base + j + count]);
                mesh.tri
                    .append(&mut vec![
                        base + j + count,
                        base + i + count,
                        base + i]);
            }
        }


        mesh
    }

    /// return two linear meshes (we can't maintain the triangle index pointers during splitting.
    /// or we can, but it would still require re-formatting the meshes after the procedure.
    /// This way, we do the reverse: After the operation, the meshes can be de-linearized if desired.
    #[rustfmt::skip]
    pub fn split(&self, cutting_plane: impl Into<Plane>) -> (Mesh, Mesh) {

        let tolerance = 0.001;

        let plane: Plane = cutting_plane.into();
        let normal = plane.normal();
        let plane_ref = &plane;

        // choice of naming is arbitrary
        let mut left = Mesh::default();
        let mut right = Mesh::default();

        // To make the cutting plane actually cut individual triangles, 
        // We must do special shit, depending on if any one vertex is on one or the other side of the cutting plane. 
        for (a,b,c) in self.iter_triangle_verts() {

            let tabc = &[a,b,c].iter().map(|p| plane.half_plane_test_tol(*p, 0.001)).map(|ord| match ord {
                Ordering::Less => Side::Left,
                Ordering::Greater => Side::Right,
                Ordering::Equal => Side::OnTop, 
            }).collect::<Vec<_>>()[..];

            let [ta, tb, tc] = tabc else {
                continue;
            };

            // take special care to keep the ordering cyclicly alphabetical, if you get what I mean
            // otherwise, newly added triangles will become flipped 
            match (ta, tb, tc) {
                (Side::Left | Side::OnTop, Side::Left  | Side::OnTop, Side::Left  | Side::OnTop) => {
                    left.verts.append(&mut vec![a,b,c]);
                },
                (Side::Right  | Side::OnTop, Side::Right  | Side::OnTop, Side::Right | Side::OnTop) => {
                    right.verts.append(&mut vec![a,b,c]);
                },
                (Side::Right, Side::Left, Side::Left)  => asym_split(&plane, &mut left, &mut right, b, c, a), // TODO something like asym_split(swap, ia, ib, ic) -> (list_for_right, list_for_left)
                (Side::Left, Side::Right, Side::Right) => asym_split(&plane, &mut right, &mut left, b, c, a),
                (Side::Left, Side::Right, Side::Left)  => asym_split(&plane, &mut left, &mut right, c, a, b),
                (Side::Right, Side::Left, Side::Right) => asym_split(&plane, &mut right, &mut left, c, a, b),
                (Side::Left, Side::Left, Side::Right)  => asym_split(&plane, &mut left, &mut right, a, b, c),
                (Side::Right, Side::Right, Side::Left) => asym_split(&plane, &mut right, &mut left, a, b, c),

                (Side::OnTop, Side::Left, Side::Right) => perfect_split(&plane, &mut left, &mut right, b, c, a),
                (Side::OnTop, Side::Right, Side::Left) => perfect_split(&plane, &mut right, &mut left, b, c, a),
                (Side::Right, Side::OnTop, Side::Left) => perfect_split(&plane, &mut left, &mut right, c, a, b),
                (Side::Left, Side::OnTop, Side::Right) => perfect_split(&plane, &mut right, &mut left, c, a, b),
                (Side::Left, Side::Right, Side::OnTop) => perfect_split(&plane, &mut left, &mut right, a, b, c),
                (Side::Right, Side::Left, Side::OnTop) => perfect_split(&plane, &mut right, &mut left, a, b, c),
            }

            // in case of an asymetrical split, split like this
            fn asym_split(plane: &Plane, maj_side: &mut Mesh, min_side: &mut Mesh, maj1: Vec3, maj2: Vec3, min: Vec3) {
                let x1 = plane.x_line(maj1, min).expect("asym hit. according to the match dispatch, this should hit");
                let x2 = plane.x_line(maj2, min).expect("asym hit. according to the match dispatch, this should hit");

                // we assume the shortest brace is the most 'delaunay'
                if x1.distance(maj2) < x2.distance(maj1) {
                    maj_side.verts.append(&mut vec![x1, maj1, maj2, maj2, x2, x1]);
                } else {
                    maj_side.verts.append(&mut vec![x2, x1, maj1, maj1, maj2, x2]);
                }
                min_side.verts.append(&mut vec![min, x1, x2]);
            }

            // in case of a perfect split, split like this
            fn perfect_split(plane: &Plane, mesh_top: &mut Mesh, mesh_bot: &mut Mesh, top: Vec3, bot: Vec3, halfway: Vec3) {
                println!("Perfection!");
                let x = plane.x_line(top, bot).expect("perfect hit. according to the match dispatch, this should hit");
                mesh_top.verts.append(&mut vec![top, x, halfway]);
                mesh_bot.verts.append(&mut vec![x, bot, halfway]);
            }
        }

        // cleanup
        left.tri = (0..left.verts.len()).collect::<Vec<usize>>();
        
        let mut left = left.to_uniform();
        left.clean_triangles();
        left.cap_holes_with_normal(normal * -1.0);
        
        right.tri = (0..right.verts.len()).collect::<Vec<usize>>();
        let mut right = right.to_uniform();
        right.clean_triangles();
        right.cap_holes_with_normal(normal);

        (left, right)
    }

    /// Intersect, do not add vertices. Just return the intersection points as polylines
    pub fn intersect(&self, plane: impl Into<Plane>) -> Vec<Polyline> {
        Vec::new()
    }

    /// intersect & add vertices
    /// return aggregated loops of inlayed vertices
    pub fn plane_loop_cut(self, plane: impl Into<Plane>) -> Self {
        self
    }

    /// returns data about where the intersection "WOULD" take place, but don't intersect it yet.
    /// This ensures certain checks are only made once
    /// Returns:
    /// - the side of every vertex (to the of the plane, to the right of the plane, or on the plane)
    /// - hashmap[(usizelowest_vertex, usizehighest_vertex)] -> (Vec3, usize_triangle_left, usize_triangle_right)
    fn pre_intersect_plane(
        &self,
        plane: Plane,
    ) -> Option<(
        Vec<Side>,
        HashMap<(usize, usize), (Vec3, usize, Option<usize>)>,
    )> {
        let vertex_sides = self
            .verts
            .iter()
            .map(|v| plane.half_plane_test(*v))
            .map(|ord| match ord {
                Ordering::Less => Side::Left,
                Ordering::Greater => Side::Right,
                Ordering::Equal => Side::OnTop,
            })
            .collect::<Vec<_>>();

        let mut intersections = HashMap::<(usize, usize), (Vec3, usize, Option<usize>)>::new();
        for (triangle_index, (va, vb, vc)) in self.iter_triangles().enumerate() {
            for (a, b) in [(va, vb), (vb, vc), (vc, va)] {
                // make sure a is always the smallest
                // prevents us from checking the same edge twice
                let (a, b) = if a < b { (a, b) } else { (b, a) };

                // if the same edge is found twice, we know the second triangle
                if let Some(entry) = intersections.get_mut(&(a, b)) {
                    entry.2 = Some(triangle_index);
                    continue;
                }

                let (side_a, side_b) = (&vertex_sides[a], &vertex_sides[b]);
                match (side_a, side_b) {
                    (Side::Left, Side::Right) | (Side::Right, Side::Left) => {
                        let (vert_a, vert_b) = (self.verts[a], self.verts[b]);
                        let x = plane
                            .x_line(vert_a, vert_b)
                            .expect("according to the match dispatch, this should hit");
                        intersections.insert((a, b), (x, triangle_index, None));
                    }
                    _ => (),
                }
            }
        }

        Some((vertex_sides, intersections))
    }

    pub fn get_neighborized_edges(&self) -> HashMap<(usize, usize), (usize, Option<usize>)> {
        let mut neighborized = HashMap::<(usize, usize), (usize, Option<usize>)>::new();
        for (triangle_index, (va, vb, vc)) in self.iter_triangles().enumerate() {
            for (a, b) in [(va, vb), (vb, vc), (vc, va)] {
                let (a, b) = if a < b { (a, b) } else { (b, a) };

                // if the same edge is found twice, we know the second triangle
                if let Some(entry) = neighborized.get_mut(&(a, b)) {
                    entry.1 = Some(triangle_index);
                    continue;
                }
                neighborized.insert((a, b), (triangle_index, None));
            }
        }
        neighborized
    }

    pub fn iter_naked_edges(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.get_neighborized_edges()
            .into_iter()
            .filter_map(|((a, b), (t1, t2))| if t2.is_none() { Some((a, b)) } else { None })
    }

    /// join consequtive edges.
    /// This algorithm essentially plays domino's
    /// Its extremely simple and slow in its current form
    /// not sure what the algorithm does to duplicates
    pub fn aggregate_edges(edges: impl Iterator<Item = (usize, usize)>) -> Vec<Vec<usize>> {
        let mut edges = edges.into_iter().collect::<Vec<_>>();

        let mut loops = Vec::new();
        while let Some(first_edge) = edges.pop() {
            let mut this_loop = vec![first_edge.0];
            let mut cursor = first_edge.1;
            loop {
                this_loop.push(cursor);
                let Some(index) = edges.iter().position(|e| e.0 == cursor || e.1 == cursor) else {
                    break;
                };

                let edge = edges.remove(index);
                if cursor == edge.0 {
                    cursor = edge.1;
                } else {
                    cursor = edge.0;
                }
                if cursor == first_edge.0 {
                    // found a circular loop!
                    // indicate this by adding the first index again
                    this_loop.push(first_edge.0);
                    break;
                }
            }

            loops.push(this_loop);
        }
        loops
    }

    pub fn clean_triangles(&mut self) {
        let mut cleaned_tri = Vec::new();
        let mut existing_tri = HashSet::new();

        // i'm doing this the dumb way, no one can stop me!
        fn sort(a: usize, b: usize, c: usize) -> (usize, usize, usize) {
            if a < b && a < c {
                if b < c {
                    (a, b, c)
                } else {
                    (a, c, b)
                }
            } else if b < a && b < c {
                if a < c {
                    (b, a, c)
                } else {
                    (b, c, a)
                }
            } else {
                if a < b {
                    (c, a, b)
                } else {
                    (c, b, a)
                }
            }
        }

        for (a, b, c) in self.iter_triangles() {

            // check duplicate vertices 
            if a == b || b == c || c == a {
                // println!("kill degenerate!");
                continue;
            }
            
            let hash = sort(a, b, c);
            if existing_tri.contains(&hash) {
                // println!("overlapping triangle!");
                continue;
            }
            existing_tri.insert(hash);
            cleaned_tri.append(&mut vec![a,b,c]);
        }
        self.tri = cleaned_tri;
    }

    /// try to patch any closed loops of naked edges
    pub fn cap_holes_with_normal(&mut self, normal: Vec3) {
        for edge_loop in Self::aggregate_edges(self.iter_naked_edges()) {

            // dbg!(&edge_loop);

            if edge_loop.first() != edge_loop.last() {
                println!("this naked edge is not a loop!");
                continue;
            }
            if edge_loop.len() == 2 {
                println!("this is a naked vertex (this triangle should not exist...)");
                continue;
            }
            if edge_loop.len() == 3 {
                println!("this is a naked sliver edge (this triangle should not exist...)");
                continue;
            }

            let verts = edge_loop
                .iter()
                .skip(1)
                .map(|e| self.verts[*e])
                .collect::<Vec<_>>();

            let plane = Plane::from_pos_normal(Vec3::ZERO, normal);
            // let Some(plane) = Plane::from_points(&verts, 0.001) else {
            //     println!("couldnt find a valid cap plane!");
            //     continue;
            // };
            
            let Some(ids) = earcut_3d(&verts, &vec![], &plane) else {
                println!("something went wrong during earcutting!");
                continue;
            };

            let mut real_ids = ids.iter().map(|id| edge_loop[*id]).collect::<Vec<_>>();
            self.tri.append(&mut real_ids);
        }
    }

    /// cap holes between given edges
    pub fn cap_edges(self, edges: &[(usize, usize)]) -> Self {
        self
    }
}

impl PointBased for Mesh {
    // TODO how to IntoIterator, so we don't have to iter / collect
    fn mutate_points(&mut self) -> Vec<&mut Vec3> {
        self.verts.iter_mut().collect() // its a bit sad we have to do this.
    }
}

impl From<Triangle> for Mesh {
    fn from(t: Triangle) -> Self {
        Self::new(
            vec![t.a, t.b, t.c],
            vec![0, 1, 2],
            vec![],
            Normals::Face(vec![t.normal()]),
        )
    }
}

#[cfg(test)]
mod test {
    use super::Mesh;
    use crate::core::Geometry;
    use crate::kernel::vec3;

    // #[test]
    // fn write_some_obj() {
    //     let mesh = Mesh::new_diamond(vec3(0.5, 0.5, 0.5), 1.333);

    //     mesh.write_obj_mtl("../data-results/", "some.obj", "some.mtl", "some.png")
    //         .expect("something went wrong!");
    // }

    #[test]
    fn test_hexagrid() {
        let mesh = Mesh::new_hexagrid(2.0, 1);
    }

    // #[test]
    // fn write_file() {
    //     let mut buffer = Vec::new();
    //     writeln!(&mut buffer, "test").unwrap();
    //     writeln!(&mut buffer, "formatted {}", "arguments").unwrap();

    //     let mut file = std::fs::File::create("data.txt").expect("create failed");
    //     file.write_all(&buffer).expect("write failed");
    // }

    #[test]
    fn transform_mesh() {
        let mut mesh = Mesh::new_diamond(vec3(0.5, 0.5, 0.5), 0.5);
        mesh = mesh.mv(-vec3(0.5, 0.5, 0.5));
        mesh = mesh.scale_u(2.0);
        assert_eq!(
            mesh.verts,
            vec![
                vec3(1.0, 0.0, 0.0),
                vec3(-1.0, 0.0, 0.0),
                vec3(0.0, 1.0, 0.0),
                vec3(0.0, -1.0, 0.0),
                vec3(0.0, 0.0, 1.0),
                vec3(0.0, 0.0, -1.0)
            ]
        );
    }

    #[test]
    fn test_aggregate() {
        // TODO: convey the difference between an open and a closed loop of edges
        let edges = vec![(1, 4), (2, 4), (3, 2), (5, 6), (3, 1)];
        assert_eq!(
            Mesh::aggregate_edges(edges.into_iter()),
            vec![vec![3, 1, 4, 2, 3], vec![5, 6]]
        );
    }
}
