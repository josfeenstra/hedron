use crate::{kernel::{Vec3, fxx}, planar::Polygon, math::{lerp, Range3, Shaper}};

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
#[derive(Debug)]
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
    pub fn tri_lerp(&self, t: Vec3) -> Vec3 {

        // create a z plane from the point z
        let [a, b, c, d, e, f, g, h] = self.verts;
        
        let za = Vec3::lerp(a, e, t.z);
        let zb = Vec3::lerp(b, f, t.z);
        let zc = Vec3::lerp(c, g, t.z);
        let zd = Vec3::lerp(d, h, t.z);

        let ac = Vec3::lerp(za, zc, t.y);
        let bd = Vec3::lerp(zb, zd, t.y);

        Vec3::lerp(ac, bd, t.x)
    }

    /// forcedir: 3D Vector in (-1.0..1.0) space, representing a deformation force
    pub fn tri_lerp_deformed(&self, t: Vec3, force_dir: Vec3, weights: [fxx; 8]) -> Vec3 {
        let master_weight = tri_lerp_weight_box(t, weights);
        let moved_t = Range3::UNIT.lerp_shaped(t, (
            Shaper::BezierMorph(force_dir.x), 
            Shaper::BezierMorph(force_dir.y), 
            Shaper::BezierMorph(force_dir.z), 
        ));

        let final_t = Vec3::lerp(t, moved_t, master_weight);
        self.tri_lerp(final_t)
    }
 
    pub fn tri_lerp_normal(&self, t: Vec3) -> Vec3 {
        // TODO figure this out
        self.tri_lerp(t)
    }

    // /// TODO: add callback (a: Vec3, b: Vec3, f: fxx, ia: usize, ib: usize) -> fxx
    // pub fn tri_lerp_smooth(&self, point: Vec3) -> Vec3 {
    //     // self. (point)
    // }
}

pub fn tri_lerp_weight_box(t: Vec3, weights: [fxx; 8]) -> fxx {
    let [a, b, c, d, e, f, g, h] = weights;
        
    let za = lerp(t.z, a, e);
    let zb = lerp(t.z, b, f);
    let zc = lerp(t.z, c, g);
    let zd = lerp(t.z, d, h);

    let ac = lerp(t.y, za, zc);
    let bd = lerp(t.y, zb, zd);

    lerp(t.x, ac, bd)
}