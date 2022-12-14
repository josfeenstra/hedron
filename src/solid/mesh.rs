use glam::{Vec2, Vec3};

#[derive(Default)]
pub struct Mesh {
    pub verts: Vec<Vec3>,
    pub uvs: Option<Vec<Vec2>>,
    pub tri: Vec<usize>,
}

impl Mesh {
    pub fn new(verts: Vec<Vec3>, tri: Vec<usize>) -> Self {
        Self {
            verts,
            tri,
            uvs: None,
        }
    }

    pub fn get_triangles(&self) -> Vec<(usize, usize, usize)> {
        let mut data = Vec::new();
        assert!(self.tri.len() % 3 == 0);
        for i in (0..self.tri.len()).step_by(3) {
            data.push((self.tri[i], self.tri[i + 1], self.tri[i + 2]))
        }
        data
    }

    pub fn get_edges(&self) -> Vec<(usize, usize)> {
        let tri = self.get_triangles();
        let mut edges = Vec::new();
        for (a, b, c) in tri {
            edges.push((a, b));
            edges.push((b, c));
            edges.push((c, a));
        }
        edges
    }
}
