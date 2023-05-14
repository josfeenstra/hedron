use crate::{
    core::Plane,
    kernel::{Vec2, Vec3},
};

extern crate earcutr;

pub fn earcut_2d(verts: &[Vec2], holes: &Vec<usize>) -> Option<Vec<usize>> {
    let verts_flattend = verts.iter().flat_map(|v| [v.x, v.y]).collect::<Vec<_>>();
    earcutr::earcut(&verts_flattend, holes, 2).ok()
}

pub fn earcut_3d(verts: &[Vec3], holes: &Vec<usize>, plane: &Plane) -> Option<Vec<usize>> {
    let verts_flattend = verts
        .iter()
        .map(|v| plane.point_to_plane(*v))
        .flat_map(|v| [v.x, v.y])
        .collect::<Vec<_>>();
    earcutr::earcut(&verts_flattend, holes, 2).ok()
}

#[cfg(test)]
mod test {

    #[test]
    fn cut() {
        let triangles = earcutr::earcut(&vec![10., 0., 0., 50., 60., 60., 70., 10.], &vec![], 2);
        println!("{:?}", triangles); // [1, 0, 3, 3, 2, 1]
    }
}
