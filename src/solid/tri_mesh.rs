#![allow(dead_code)]
use crate::{
    kernel::*,
    prelude::{Triangle, Vectors},
};

/// a corner of a triangle face
#[derive(Debug, Clone)]
pub struct TriCorner {
    v: usize,          // index of vertex
    uv: usize,         // index of uv
    n: usize,          // index of normal
    nb: Option<usize>, // index of nb triangle. Given triangle (a,b,c), nb of corner (a) is adjacent to (b, c)
}

impl TriCorner {
    /// uniform
    pub fn new(v: usize, uv: usize, n: usize, nb: Option<usize>) -> Self {
        Self { v, uv, n, nb }
    }

    pub fn new_uni(i: usize) -> Self {
        Self {
            v: i,
            uv: i,
            n: i,
            nb: Some(i),
        }
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

#[derive(Debug, Clone)]
enum Triangles {
    Linear,
    Mono(Vec<(usize, usize, usize)>), // a, b, c
    Hetero(Vec<(TriCorner, TriCorner, TriCorner)>),
}

/// Doing this ensures we can always offer every iterator at least 'a' normal
#[derive(Debug, Clone)]
enum Normals {
    One(Vec3),
    Many(Vec<Vec3>),
}

/// Doing this ensures we can always offer every iterator at least 'an' uv
#[derive(Debug, Clone)]
enum Uvs {
    One(Vec2),
    Many(Vec<Vec2>),
}

/// A triangular mesh.
///
/// Represented in various ways, See `Triangles`.
struct TriMesh {
    pub verts: Vec<Vec3>,
    pub tri: Triangles,
    pub uvs: Uvs,
    pub normals: Normals,
}

impl Default for TriMesh {
    fn default() -> Self {
        Self {
            verts: Default::default(),
            tri: Triangles::Linear,
            uvs: Uvs::One(Vec2::ZERO),
            normals: Normals::One(Vec3::Z),
        }
    }
}

impl TriMesh {
    pub fn new_linear(verts: Vec<Vec3>, uvs: Uvs, normals: Normals) -> Self {
        Self {
            verts,
            tri: Triangles::Linear,
            uvs,
            normals,
        }
    }

    pub fn new_mono(
        verts: Vec<Vec3>,
        tri: Vec<(usize, usize, usize)>,
        uvs: Uvs,
        normals: Normals,
    ) -> Self {
        Self {
            verts,
            tri: Triangles::Mono(tri),
            uvs,
            normals,
        }
    }

    pub fn new_hetero(
        verts: Vec<Vec3>,
        tri: Vec<(TriCorner, TriCorner, TriCorner)>,
        uvs: Uvs,
        normals: Normals,
    ) -> Self {
        Self {
            verts,
            tri: Triangles::Hetero(tri),
            uvs,
            normals,
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
    pub fn with_tri_mono(mut self, tri: Vec<(usize, usize, usize)>) -> Self {
        self.tri = Triangles::Mono(tri);
        self
    }

    // a non-uniform triangle model, with individual pointers for vertex, uvs, and normals.
    // triangle corners cal also house pointers to neighboring triangles.
    pub fn with_tri_hetero(mut self, tri: Vec<(TriCorner, TriCorner, TriCorner)>) -> Self {
        self.tri = Triangles::Hetero(tri);
        self
    }

    /// set multiple uvs
    pub fn with_uvs(mut self, uvs: impl Into<Vec<Vec2>>) -> Self {
        self.uvs = Uvs::Many(uvs.into());
        self
    }

    /// set a singular, uniform uv
    pub fn with_uv(mut self, uv: impl Into<Vec2>) -> Self {
        self.uvs = Uvs::One(uv.into());
        self
    }

    /// set multiple normals
    pub fn with_normals(mut self, normals: impl Into<Vec<Vec3>>) -> Self {
        self.normals = Normals::Many(normals.into());
        self
    }

    /// set a singular, uniform normal
    pub fn with_normal(mut self, normal: impl Into<Vec3>) -> Self {
        self.normals = Normals::One(normal.into());
        self
    }

    /// turns mesh heterogenos
    pub fn with_face_normals(mut self) -> Self {
        self = self.to_hetero();
        self.normals = Normals::Many(self.calc_flat_face_normals());

        self
    }

    pub fn with_vertex_normals(mut self) -> Self {
        self.normals = Normals::Many(self.calc_vertex_normals());
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

        for (face_id, (ia, ib, ic)) in self.iter_triangles_hetero().enumerate() {
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
        match self.tri {
            Triangles::Linear => self.verts.len() / 3,
            Triangles::Mono(tri) => tri.len(),
            Triangles::Hetero(tri) => tri.len(),
        }
    }

    /// This thing is key!
    pub fn iter_triangles_hetero(
        &self,
    ) -> impl Iterator<Item = (TriCorner, TriCorner, TriCorner)> + '_ {
        match self.tri {
            Triangles::Linear => {
                let count = self.verts.len();
                (0..count)
                    .step_by(3)
                    .map(|i| (TriCorner, TriCorner, TriCorner))
            }
            Triangles::Mono(_) => todo!(),
            Triangles::Hetero(tri) => tri.iter(),
        }
    }

    /// Whatever triangle model we are using, pretend the triangle is mono, and iterate over it
    pub fn iter_triangles_mono(&self) {
        //     todo!()
        //     // (0..self.tri.len())
        //     //     .step_by(3)
        //     //     .map(|i| (self.tri[i], self.tri[i + 1], self.tri[i + 2]))
    }

    // pub fn iter_triangle_verts_ids(&self) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
    //     todo!()
    //     // (0..self.tri.len())
    //     //     .step_by(3)
    //     //     .map(|i| (self.tri[i], self.tri[i + 1], self.tri[i + 2]))
    // }

    // pub fn iter_triangle_verts(&self) -> impl Iterator<Item = (Vec3, Vec3, Vec3)> + '_ {
    //     self.iter_triangles()
    //         .map(|(a, b, c)| (self.verts[a], self.verts[b], self.verts[c]))
    // }

    pub fn iter_edges(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.iter_triangles_hetero()
            .flat_map(|(a, b, c)| [(a, b), (b, c), (c, a)])
    }

    pub fn iter_unique_edges(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.iter_triangles_hetero()
            .flat_map(|(a, b, c)| [(a, b), (b, c), (c, a)])
    }

    pub fn iter_naked_edges(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.iter_edges().filter(|edge| {
            let occurence_count = self
                .iter_edges()
                .filter(|other| edge == other || *edge == (other.1, other.0))
                .count();
            occurence_count < 3
        })
    }

    /// join consequtive parts
    pub fn aggregate_edges(&self, edges: impl Iterator<Item = (usize, usize)>) -> Vec<Vec<usize>> {
        // let mut linked_list = HashSet::new();
        // edges.it

        // linelist to Vec<polyline> operation

        todo!();

        // edges.count()
    }

    pub fn to_linear() -> Self {
        todo!();
    }

    pub fn to_mono(mut self) -> Self {
        todo!();
    }

    pub fn to_hetero(mut self) -> Self {
        todo!();
    }
}
