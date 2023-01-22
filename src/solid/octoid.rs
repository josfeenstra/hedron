use crate::{kernel::Vec3, planar::Polygon};

use super::Mesh;

/// The cube model usd) ------ 2 ------ (c)ed:
/// ```markdown
///
///     (e) --------------- (f)
///     /|                  /|
///    / |      z          / |
///   /  |        y       /  |
/// (h) --------------- (g)  |
///  |   |               |   |
///  |-x |               | x |
///  |   |    -y         |   |
///  |  (a) --------------- (b)
///  |  /                |  /
///  | /       -z        | /
///  |/                  |/
/// (d) --------------- (c)
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

    /// TODO: can be more efficient
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

    /// trilinear interpolation
    pub fn tri_lerp(&self, point: Vec3) -> Vec3 {

        // create a z plane from the point z
        let [a, b, c, d, e, f, g, h] = self.verts;

        let za = Vec3::lerp(a, e, point.z);
        let zb = Vec3::lerp(b, f, point.z);
        let zc = Vec3::lerp(c, g, point.z);
        let zd = Vec3::lerp(d, h, point.z);

        let ya = Vec3::lerp(zd, za, point.y);
        let yb = Vec3::lerp(zc, zb, point.y);

        Vec3::lerp(ya, yb, point.x)
    }


    /// TODO: add callback (a: Vec3, b: Vec3, f: fxx, ia: usize, ib: usize) -> fxx
    pub fn tri_lerp_smooth(&self, point: Vec3) -> Vec3 {
        self.tri_lerp(point)
    }
}
