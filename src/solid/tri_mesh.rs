#![allow(dead_code)]
use crate::{
    kernel::*,
    util::{roughly_equals, OneOrMany},
};

// /// a corner of a triangle face
// #[derive(Debug, Clone)]
// pub struct TriCorner {
//     v: usize,          // index of vertex
//     uv: usize,         // index of uv
//     n: usize,          // index of normal
//     nb: Option<usize>, // index of nb triangle. Given triangle (a,b,c), nb of corner (a) is adjacent to (b, c)
// }

// impl TriCorner {
//     /// uniform
//     pub fn new(v: usize, uv: usize, n: usize, nb: Option<usize>) -> Self {
//         Self { v, uv, n, nb }
//     }

//     pub fn new_uni(i: usize) -> Self {
//         Self {
//             v: i,
//             uv: i,
//             n: i,
//             nb: None,
//         }
//     }

//     pub fn with_nb(mut self, nb: usize) -> Self {
//         self.nb = Some(nb);
//         self
//     }

//     // a face normal is just the unique case in which all three corners of a face point to the same normal
//     pub fn try_get_face_normal(face: &(TriCorner, TriCorner, TriCorner)) -> Option<usize> {
//         let (a, b, c) = face;
//         if a.n == b.n && a.n == c.n {
//             return Some(a.n);
//         }
//         return None;
//     }
// }

/// triangle model used
#[derive(Debug, Clone)]
enum Triangles {
    Linear,
    Mono(Vec<usize>),
    // Hetero(Vec<(TriCorner, TriCorner, TriCorner)>),
}

/// A triangular mesh.
///
/// Represented in various ways, See `Triangles`.
struct TriMesh {
    pub verts: Vec<Vec3>,
    pub tri: Triangles,
    pub uvs: OneOrMany<Vec2>,
    pub normals: OneOrMany<Vec3>,
    pub neighbors: Option<Vec<usize>>,
}

impl Default for TriMesh {
    fn default() -> Self {
        Self {
            verts: Default::default(),
            tri: Triangles::Linear,
            uvs: OneOrMany::One(Vec2::ZERO),
            normals: OneOrMany::One(Vec3::Z),
            neighbors: None,
        }
    }
}

impl TriMesh {
    pub fn new(
        verts: Vec<Vec3>,
        tri: Triangles,
        uvs: OneOrMany<Vec2>,
        normals: OneOrMany<Vec3>,
        neighbors: Option<Vec<usize>>,
    ) -> Self {
        Self {
            verts,
            tri,
            uvs,
            normals,
            neighbors,
        }
    }

    pub fn new_linear(verts: Vec<Vec3>) -> Self {
        Self {
            verts,
            tri: Triangles::Linear,
            ..Default::default()
        }
    }

    pub fn new_mono(verts: Vec<Vec3>, tri: Vec<usize>) -> Self {
        Self {
            verts,
            tri: Triangles::Mono(tri),
            ..Default::default()
        }
    }

    // pub fn new_hetero(verts: Vec<Vec3>, tri: Vec<(TriCorner, TriCorner, TriCorner)>) -> Self {
    //     Self {
    //         verts,
    //         tri: Triangles::Hetero(tri),
    //         ..Default::default()
    //     }
    // }

    pub fn with_verts(mut self, verts: Vec<Vec3>) -> Self {
        self.verts = verts;
        self
    }

    /// leads to a linear mesh model. Verts are a simple vertex lists, and we are not reusing any vertex, normal or uv.
    pub fn with_tri_linear(mut self) -> Self {
        self.tri = Triangles::Linear;
        self
    }

    /// uniform triangle model, where a corner of the triangle points to a vertex, uv and normal.
    pub fn with_tri_mono(mut self, tri: Vec<usize>) -> Self {
        self.tri = Triangles::Mono(tri);
        self
    }

    // // a non-uniform triangle model, with individual pointers for vertex, uvs, and normals.
    // // triangle corners cal also house pointers to neighboring triangles.
    // pub fn with_tri_hetero(mut self, tri: Vec<(TriCorner, TriCorner, TriCorner)>) -> Self {
    //     self.tri = Triangles::Hetero(tri);
    //     self
    // }

    /// set multiple uvs
    pub fn with_uvs(mut self, uvs: impl Into<Vec<Vec2>>) -> Self {
        self.uvs = OneOrMany::Many(uvs.into());
        self
    }

    /// set a singular, uniform uv
    pub fn with_uv(mut self, uv: impl Into<Vec2>) -> Self {
        self.uvs = OneOrMany::One(uv.into());
        self
    }

    /// set multiple normals
    pub fn with_normals(mut self, normals: impl Into<Vec<Vec3>>) -> Self {
        self.normals = OneOrMany::Many(normals.into());
        self
    }

    /// set a singular, uniform normal
    pub fn with_normal(mut self, normal: impl Into<Vec3>) -> Self {
        self.normals = OneOrMany::One(normal.into());
        self
    }

    // // /// turns mesh heterogenos
    // pub fn with_face_normals(mut self) -> Self {
    //     self = self.to_hetero();
    //     self.normals = OneOrMany::Many(self.calc_flat_face_normals());

    //     self
    // }

    // pub fn with_vertex_normals(mut self) -> Self {
    //     self.normals = OneOrMany::Many(self.calc_vertex_normals());
    //     self
    // }

    // pub fn with_neighbors(mut self) -> Self {
    //     todo!();
    // }

    ///////////////////////////////////////////////////////////////////////////

    // TODO: construct UV & Normal getters & setters, wrapping the thing

    ///////////////////////////////////////////////////////////////////////////

    // pub fn calc_flat_face_normals(&self) -> Vec<Vec3> {
    //     self.iter_triangle_verts()
    //         .map(|(a, b, c)| Triangle::new(a, b, c).normal())
    //         .collect::<Vec<_>>()
    // }

    // /// this is not how vertex normals are supposed to work.
    // pub fn calc_vertex_normals(&self) -> Vec<Vec3> {
    //     let flat_face_normals = self.calc_flat_face_normals();
    //     let mut buckets: Vec<Vec<Vec3>> = (0..self.verts.len()).map(|_| Vec::new()).collect();

    //     for (face_id, (ia, ib, ic)) in self.iter_triangles().enumerate() {
    //         for index in [ia, ib, ic] {
    //             buckets[index].push(flat_face_normals[face_id]);
    //         }
    //     }

    //     buckets
    //         .into_iter()
    //         .map(|bucket| Vectors::average(&bucket).normalize())
    //         .collect()
    // }

    ///////////////////////////////////////////////////////////////////////////

    pub fn tri_count(&self) -> usize {
        match &self.tri {
            Triangles::Linear => self.verts.len() / 3,
            Triangles::Mono(tri) => tri.len(),
        }
    }

    // /// This thing is key!
    // pub fn iter_triangles(&self) -> impl Iterator<Item = TriCorner> + '_ {
    //     match self.tri {
    //         Triangles::Linear => {
    //             let count = self.verts.len();
    //             (0..count).step_by(3).map(|i| {
    //                 (
    //                     TriCorner::new_uni(i),
    //                     TriCorner::new_uni(i + 1),
    //                     TriCorner::new_uni(i + 2),
    //                 )
    //             })
    //         }
    //         Triangles::Mono(tri) => tri.iter().map(|(a, b, c)| {
    //             (
    //                 TriCorner::new_uni(*a),
    //                 TriCorner::new_uni(*b),
    //                 TriCorner::new_uni(*c),
    //             )
    //         }),
    //         Triangles::Hetero(tri) => tri.iter(),
    //         Triangles::MonoNb(tri) => tri.iter().map(|(a, b, c, nba, nbb, nbc)| {}),
    //     }
    // }

    // get data of a vertex in a triangle
    fn get_vertex_data(
        &self,
        ivert: usize,
        iuv: usize,
        inorm: usize,
    ) -> Option<(&Vec3, &Vec2, &Vec3)> {
        Some((
            self.verts.get(ivert)?,
            self.uvs.get(iuv)?,
            self.normals.get(inorm)?,
        ))
    }

    // get data of a vertex in a triangle
    fn get_mut_vertex_data(
        &mut self,
        ivert: usize,
        iuv: usize,
        inorm: usize,
    ) -> Option<(&mut Vec3, &mut Vec2, &mut Vec3)> {
        Some((
            self.verts.get_mut(ivert)?,
            self.uvs.get_mut(iuv)?,
            self.normals.get_mut(inorm)?,
        ))
    }

    // fn get_triangle(&self, i: usize) -> Option<(TriCorner, TriCorner, TriCorner)> {

    //     match &self.tri {
    //         Triangles::Linear => Some(
    //             (i * 3,
    //             (i * 3) + 1,
    //             (i * 3) + 2)
    //         ),
    //         Triangles::Mono(mono) => mono.get(i).map(|tr| ),
    //         Triangles::MonoNb(mono_nb) => mono_nb.get(i).map(|tr| (tr.0, tr.1, tr.2)),
    //         Triangles::Hetero(hetero) => hetero.get(i).cloned(),
    //     }
    // }

    /// Whatever triangle model we are using, pretend the triangle is mono, and iterate over it
    // pub fn iter_triangles_mono(&self) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
    //     (0..self.tri_count()).map(|i| self.get_triangle_mono(i).unwrap())
    // }

    // pub fn iter_triangle_verts_ids(&self) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
    //     (0..self.tri.len())
    //         .step_by(3)
    //         .map(|i| (self.tri[i], self.tri[i + 1], self.tri[i + 2]))
    // }

    // pub fn iter_triangle_verts(&self) -> impl Iterator<Item = (Vec3, Vec3, Vec3)> + '_ {
    //     self.iter_triangles()
    //         .map(|(a, b, c)| (self.verts[a], self.verts[b], self.verts[c]))
    // }

    // pub fn iter_edges(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
    //     self.iter_triangles_hetero()
    //         .flat_map(|(a, b, c)| [(a, b), (b, c), (c, a)])
    // }

    // pub fn iter_unique_edges(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
    //     self.iter_triangles_hetero()
    //         .flat_map(|(a, b, c)| [(a, b), (b, c), (c, a)])
    // }

    // pub fn iter_naked_edges(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
    //     self.iter_edges().filter(|edge| {
    //         let occurence_count = self
    //             .iter_edges()
    //             .filter(|other| edge == other || *edge == (other.1, other.0))
    //             .count();
    //         occurence_count < 3
    //     })
    // }

    pub fn to_linear(self) -> Self {
        match self.tri {
            Triangles::Linear => self,
            Triangles::Mono(mono) => {
                let verts = mono
                    .iter()
                    .map(|i| self.verts[*i])
                    .collect::<Vec<Vec3>>();
                let normals = match self.normals {
                    OneOrMany::One(one) => OneOrMany::One(one),
                    OneOrMany::Many(many) => {
                        let normals = mono
                            .iter()
                            .map(|i| many[*i])
                            .collect::<Vec<Vec3>>();
                        OneOrMany::Many(normals)
                    }
                };
                let uvs = match self.uvs {
                    OneOrMany::One(one) => OneOrMany::One(one),
                    OneOrMany::Many(many) => {
                        let uvs = mono
                            .iter()
                            .map(|i| many[*i])
                            .collect::<Vec<Vec2>>();
                        OneOrMany::Many(uvs)
                    }
                };
                Self::new(verts, Triangles::Linear, uvs, normals, None)
            }
            // Triangles::MonoNb(mono_nb) => {} 
            // Triangles::Hetero(hetero) => {},
        }
    }

    // just make sure we are re-using vertices
    pub fn to_at_least_mono(self) -> Self {
        if matches!(self.tri, Triangles::Linear) {
            self.to_mono()
        } else {
            self
        }
    }

    pub fn to_mono(self) -> Self {
        match &self.tri {
            Triangles::Mono(_) => self,
            Triangles::Linear => {
                let ids = Self::desoupify(&self.verts);
                let mut uvs = self.uvs.map(|_| Vec::new());
                let mut normals = self.normals.map(|_| Vec::new());
                let mut verts = Vec::new();
                for (i, id) in ids.iter().enumerate() {
                    if i == *id {
                        verts.push(self.verts[i]);
                        uvs.push(self.uvs.get(i).unwrap().clone());
                        normals.push(self.normals.get(i).unwrap().clone());
                    }
                }

                Self::new(verts, Triangles::Mono(ids), uvs, normals, None)
            }
        }
    }
}

/// associated, but separate functions
impl TriMesh {
    /// Check all vertices before you.
    /// If one looks similar, stop.
    /// Produces a mapping per vertex. Each vertex pointer points to itself of one earlier, similar looking vertex
    pub fn desoupify(verts: &Vec<Vec3>) -> Vec<usize> {
        let mut sim = Vec::new();
        for i in 0..verts.len() {
            let i_similar_vertex = (0..i)
                .find(|j| roughly_equals(verts[i], verts[*j]))
                .unwrap_or(i);
            sim.push(i_similar_vertex)
        }
        sim
    }

    /// join consequtive parts
    pub fn aggregate_edges(&self, _edges: impl Iterator<Item = (usize, usize)>) -> Vec<Vec<usize>> {
        // let mut linked_list = HashSet::new();
        // edges.it

        // linelist to Vec<polyline> operation

        todo!();

        // edges.count()
    }
}
