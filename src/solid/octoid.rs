use crate::{kernel::Vec3, planar::Polygon};

use super::Mesh;


// //
// //
// // The cube model used:
// //
// //     (e) ------ 4 ------ (f)
// //     /|                  /|
// //    7 |                 5 |
// //   /  |                /  |
// // (h) ------ 6 ------ (g)  |
// //  |   8               |   9
// //  |   |               |   |
// //  |   |               |   |
// //  |  (a) ------ 0 ------ (b)
// //  11 /               10  /
// //  | 3                 | 1
// //  |/                  |/
// // (d) ------ 2 ------ (c)
/// ```
pub struct Octoid {
    pub verts: [Vec3; 8]
}

impl Octoid {
    pub fn new(verts: [Vec3; 8]) -> Self {
        Self { verts }
    }

    pub fn from_extrude(base_verts: [Vec3; 4], extrusion: Vec3) -> Self {
        let [a, b, c, d] = base_verts;
        Self::new([a, b, c, d, a + extrusion, b + extrusion, c + extrusion, d + extrusion])
    }

    pub fn to_mesh(&self) -> Mesh {
        Mesh::from_join(self.faces_to_polygons().iter().map(|p| p.triangulate_naive()).collect())
    }

    pub fn faces_to_polygons(&self) -> [Polygon; 6] {
        
        let [a, b, c, d, e, f, g, h] = self.verts;

        [
            Polygon::new(vec![a,b,c,d]),
            Polygon::new(vec![d,c,g,h]),
            Polygon::new(vec![c,b,f,g]),
            Polygon::new(vec![b,a,e,f]),
            Polygon::new(vec![a,d,h,e]),
            Polygon::new(vec![e,f,g,h])
        ]
    }
    
}
