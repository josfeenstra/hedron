
use glam::Vec3;

// interpolate the verts of a mesh from its bounding box into an oct.
// I define an oct as a mesh which is a cube in topology, but with varying vertex locations.
// mesh must be in the <x,y,z> space of -1.0..1.0. 
// TODO you might be able to mathematically rewrite this for greater efficiently
pub fn octopolate(oct: &[Vec3; 8], verts: &mut Vec<[f32; 3]>) {
   
    for vert in verts {
        let z0 = Vec3::lerp(oct[0], oct[4], vert[2]);
        let z1 = Vec3::lerp(oct[1], oct[5], vert[2]);
        let z2 = Vec3::lerp(oct[2], oct[6], vert[2]);
        let z3 = Vec3::lerp(oct[3], oct[7], vert[2]);

        let y0 = Vec3::lerp(z0, z2, vert[1]);
        let y1 = Vec3::lerp(z1, z3, vert[1]);

        let point = Vec3::lerp(y0, y1, vert[0]);
    };
}