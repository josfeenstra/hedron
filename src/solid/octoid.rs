use crate::{kernel::Vec3, planar::Polygon};

use super::Mesh;

/// The cube model usd) ------ 2 ------ (c)ed:
/// ```markdown
///
///     (g) --------------- (h)
///     /|                  /|
///    / |      z          / |
///   /  |        y       /  |
/// (e) --------------- (f)  |
///  |   |               |   |
///  |-x |               | x |
///  |   |    -y         |   |
///  |  (c) --------------- (d)
///  |  /                |  /
///  | /       -z        | /
///  |/                  |/
/// (a) --------------- (b)
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
            Polygon::new(vec![a,c,d,b]),
            Polygon::new(vec![a,b,f,e]),
            Polygon::new(vec![b,d,h,f]),
            Polygon::new(vec![d,c,g,h]),
            Polygon::new(vec![c,a,e,g]),
            Polygon::new(vec![e,f,h,g])
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

        let ac = Vec3::lerp(za, zc, point.y);
        let bd = Vec3::lerp(zb, zd, point.y);

        Vec3::lerp(ac, bd, point.x)
    }
 
    /// TODO: add callback (a: Vec3, b: Vec3, f: fxx, ia: usize, ib: usize) -> fxx
    pub fn tri_lerp_smooth(&self, point: Vec3) -> Vec3 {
        self.tri_lerp(point)
    }
}
