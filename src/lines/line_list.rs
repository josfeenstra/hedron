use glam::Vec3;

/// A sequence line segments, each with a explicit start and end position
#[derive(Debug, Clone)]
pub struct LineList {
    pub verts: Vec<Vec3>,
}

impl LineList {
    pub fn new(verts: Vec<Vec3>) -> Self {
        LineList { verts }
    }

    pub fn new_empty() -> Self {
        LineList { verts: Vec::new() }
    }

    pub fn new_join(mut line_lists: Vec<LineList>) -> Self {
        if line_lists.len() == 0 {
            return Self::new_empty();
        }

        // this process makes more conceptual sense in reverse
        // this way, we add all lines to the first linelist in the list
        line_lists.reverse();
        let mut joined = line_lists.pop().unwrap();

        while let Some(lines) = line_lists.pop() {
            joined.append(lines);
        }

        joined
    }

    pub fn new_cube(center: Vec3, size: Vec3) -> Self {
        const CORNERS_CUBE: usize = 8;
        const EDGES_CUBE: usize = 12;
        const POS: f32 = 1.0;
        const NEG: f32 = -1.0;
        let mut corners = Vec::with_capacity(CORNERS_CUBE);
        for x in vec![NEG, POS] {
            for y in vec![NEG, POS] {
                for z in vec![NEG, POS] {
                    corners.push(center + size * Vec3::new(x, y, z));
                }
            }
        }

        let mut verts: Vec<Vec3> = Vec::with_capacity(EDGES_CUBE * 2);
        for i in 0..CORNERS_CUBE {
            for j in vec![1, 2, 4] {
                if i + j < CORNERS_CUBE {
                    verts.push(corners[i].clone());
                    verts.push(corners[i + j].clone());
                }
            }
        }

        Self::new(verts)
    }

    pub fn new_grid(count: u32, scale: f32) -> Self {
        let half_total_size = ((count as f32 - 1.0) * scale) / 2.0;

        let mut verts: Vec<Vec3> = Vec::with_capacity((count * 4) as usize);

        for i in 0..count {
            let t = -half_total_size + scale * i as f32;
            verts.push(Vec3::new(t, -half_total_size, 0.0));
            verts.push(Vec3::new(t, half_total_size, 0.0));
            verts.push(Vec3::new(-half_total_size, t, 0.0));
            verts.push(Vec3::new(half_total_size, t, 0.0));
        }

        Self::new(verts)
    }

    ///////////////////////////////////////////////

    pub fn append(&mut self, mut rhs: LineList) {
        self.verts.append(&mut rhs.verts);
    }
}
