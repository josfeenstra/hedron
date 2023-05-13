#![allow(dead_code)]
use crate::{
    kernel::*,
    prelude::{Triangle, Vectors},
    util::{roughly_equals, OneOrMany},
};

/// a corner of a triangle face
#[derive(Debug, Clone)]
pub struct TriCorner {
    v: usize,  // index of vertex
    uv: usize, // index of uv
    n: usize,  // index of normal
}

impl TriCorner {
    /// uniform
    pub fn new(v: usize, uv: usize, n: usize) -> Self {
        Self { v, uv, n }
    }

    pub fn uniform(i: usize) -> Self {
        Self { v: i, uv: i, n: i }
    }

    // a face normal is just the unique case in which all three corners of a face point to the same normal
    pub fn try_get_face_normal(face: &(TriCorner, TriCorner, TriCorner)) -> Option<usize> {
        let (a, b, c) = face;
        if a.n == b.n && a.n == c.n {
            return Some(a.n);
        }
        return None;
    }
}

/// triangle model used
#[derive(Debug, Clone)]
enum Triangles {
    Linear,
    Mono(Vec<usize>),
    Hetero(Vec<TriCorner>),
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
        neighbors: Option<Vec<usize>>, // Given triangle (a,b,c), nb of corner (a) is adjacent to (b, c)
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

    pub fn new_hetero(verts: Vec<Vec3>, tri: Vec<TriCorner>) -> Self {
        Self {
            verts,
            tri: Triangles::Hetero(tri),
            ..Default::default()
        }
    }

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

    /// a non-uniform triangle model, with individual pointers for vertex, uvs, and normals.
    /// triangle corners cal also house pointers to neighboring triangles.
    pub fn with_tri_hetero(mut self, tri: Vec<TriCorner>) -> Self {
        self.tri = Triangles::Hetero(tri);
        self
    }

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

    /// turns mesh heterogenos
    pub fn with_face_normals(mut self) -> Self {
        self = self.to_hetero();
        self.normals = OneOrMany::Many(self.calc_flat_face_normals());

        let Triangles::Hetero(hetero) = &mut self.tri else {
            unreachable!()
        };

        for (i, corner) in hetero.iter_mut().enumerate() {
            corner.n = i / 3;
        }

        self
    }

    pub fn with_vertex_normals(mut self) -> Self {
        self.normals = OneOrMany::Many(self.calc_vertex_normals());
        self
    }

    ///////////////////////////////////////////////////////////////////////////

    // TODO: construct UV & Normal getters & setters, wrapping the thing

    ///////////////////////////////////////////////////////////////////////////

    pub fn calc_flat_face_normals(&self) -> Vec<Vec3> {
        self.iter_triangle_verts()
            .map(|(a, b, c)| Triangle::new(a, b, c).normal())
            .collect::<Vec<_>>()
    }

    /// this is not how vertex normals are supposed to work.
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

    ///////////////////////////////////////////////////////////////////////////

    pub fn tri_count(&self) -> usize {
        match &self.tri {
            Triangles::Linear => self.verts.len() / 3,
            Triangles::Mono(tri) => tri.len(),
            Triangles::Hetero(tri) => tri.len(),
        }
    }

    /// pretend to iter mono
    pub fn iter_triangles(&self) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
        (0..self.tri_count() * 3)
            .step_by(3)
            .map(|i| match &self.tri {
                Triangles::Linear => (i * 3, (i * 3) + 1, (i * 3) + 2),
                Triangles::Mono(tri) => (tri[i], tri[i + 1], tri[i + 2]),
                Triangles::Hetero(tri) => (tri[i].v, tri[i + 1].v, tri[i + 2].v),
            })
    }

    /// pretend to iter hetero
    pub fn iter_triangles_hetero(
        &self,
    ) -> impl Iterator<Item = (TriCorner, TriCorner, TriCorner)> + '_ {
        (0..self.tri_count() * 3)
            .step_by(3)
            .map(|i| match &self.tri {
                Triangles::Linear => (
                    TriCorner::uniform(i * 3),
                    TriCorner::uniform(i * 3 + 1),
                    TriCorner::uniform(i * 3 + 2),
                ),
                Triangles::Mono(tri) => (
                    TriCorner::uniform(tri[i]),
                    TriCorner::uniform(tri[i + 1]),
                    TriCorner::uniform(tri[i + 2]),
                ),
                Triangles::Hetero(tri) => (tri[i].clone(), tri[i + 1].clone(), tri[i + 2].clone()),
            })
    }

    pub fn iter_triangle_verts(&self) -> impl Iterator<Item = (Vec3, Vec3, Vec3)> + '_ {
        self.iter_triangles()
            .map(|(a, b, c)| (self.verts[a], self.verts[b], self.verts[c]))
    }

    pub fn iter_edges(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.iter_triangles()
            .flat_map(|(a, b, c)| [(a, b), (b, c), (c, a)])
    }

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

    pub fn to_linear(self) -> Self {
        match self.tri {
            Triangles::Linear => self,
            Triangles::Mono(mono) => {
                let verts = mono.iter().map(|i| self.verts[*i]).collect::<Vec<Vec3>>();
                let normals = match self.normals {
                    OneOrMany::One(one) => OneOrMany::One(one),
                    OneOrMany::Many(many) => {
                        let normals = mono.iter().map(|i| many[*i]).collect::<Vec<Vec3>>();
                        OneOrMany::Many(normals)
                    }
                };
                let uvs = match self.uvs {
                    OneOrMany::One(one) => OneOrMany::One(one),
                    OneOrMany::Many(many) => {
                        let uvs = mono.iter().map(|i| many[*i]).collect::<Vec<Vec2>>();
                        OneOrMany::Many(uvs)
                    }
                };
                Self::new(verts, Triangles::Linear, uvs, normals, None)
            }
            Triangles::Hetero(hetero) => {
                let verts = hetero
                    .iter()
                    .map(|h| self.verts[h.v])
                    .collect::<Vec<Vec3>>();

                let normals = match self.normals {
                    OneOrMany::One(one) => OneOrMany::One(one),
                    OneOrMany::Many(many) => {
                        let normals = hetero.iter().map(|h| many[h.n]).collect::<Vec<Vec3>>();
                        OneOrMany::Many(normals)
                    }
                };
                let uvs = match self.uvs {
                    OneOrMany::One(one) => OneOrMany::One(one),
                    OneOrMany::Many(many) => {
                        let uvs = hetero.iter().map(|h| many[h.uv]).collect::<Vec<Vec2>>();
                        OneOrMany::Many(uvs)
                    }
                };
                Self::new(verts, Triangles::Linear, uvs, normals, None)
            }
        }
    }

    pub fn to_mono(self) -> Self {
        match &self.tri {
            Triangles::Mono(_) => self,
            Triangles::Linear => {
                let mono = Self::desoupify(&self.verts);
                let mut uvs = self.uvs.map(|_| Vec::new());
                let mut normals = self.normals.map(|_| Vec::new());
                let mut verts = Vec::new();
                for (i, id) in mono.iter().enumerate() {
                    if i == *id {
                        verts.push(self.verts[i]);
                        uvs.push(self.uvs.get(i).unwrap().clone());
                        normals.push(self.normals.get(i).unwrap().clone());
                    }
                }

                Self::new(verts, Triangles::Mono(mono), uvs, normals, None)
            }
            Triangles::Hetero(hetero) => {
                // linearize uvs and normals
                let mut uvs = self.uvs.map(|_| Vec::new());
                let mut normals = self.normals.map(|_| Vec::new());

                let mono = hetero
                    .into_iter()
                    .map(|h| {
                        uvs.push(self.uvs.get(h.uv).unwrap().clone());
                        normals.push(self.normals.get(h.n).unwrap().clone());
                        h.v
                    })
                    .collect();
                Self::new(
                    self.verts,
                    Triangles::Mono(mono),
                    self.uvs,
                    self.normals,
                    None,
                )
            }
        }
    }

    pub fn to_hetero(self) -> Self {
        match &self.tri {
            Triangles::Hetero(_) => self,
            Triangles::Linear => self.to_mono().to_hetero(),
            Triangles::Mono(mono) => {
                let hetero = mono
                    .iter()
                    .map(|m| TriCorner::uniform(*m))
                    .collect::<Vec<_>>();
                Self::new(
                    self.verts,
                    Triangles::Hetero(hetero),
                    self.uvs,
                    self.normals,
                    None,
                )
            }
        }
    }

    // just make sure we are re-using vertices, don't care about mono / hetero
    pub fn to_at_least_mono(self) -> Self {
        if matches!(self.tri, Triangles::Linear) {
            self.to_mono()
        } else {
            self
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
