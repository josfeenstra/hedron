use glam::{UVec3, UVec2};

#[inline]
pub fn iter_xy_u<'a>(count: UVec2) -> impl Iterator<Item = UVec2> + 'a  {
    (0..count.y)
    .flat_map(move |y| (0..count.x)
    .map(move |x| UVec2::new(x, y)))
}

#[inline]
pub fn iter_xyz_u<'a>(count: UVec3) -> impl Iterator<Item = UVec3> + 'a  {
    (0..count.z)
    .flat_map(move |z| (0..count.y)
    .flat_map(move |y| (0..count.x)
    .map(move |x| UVec3::new(x, y, z))))
}

#[inline]
pub fn iter_xy<'a>(count: (usize, usize)) -> impl Iterator<Item = (usize, usize)> + 'a {
    (0..count.1).flat_map(move |y| (0..count.0).map(move |x| (x, y)))
}

#[inline]
pub fn iter_xyz<'a>(count: (usize, usize, usize)) -> impl Iterator<Item = (usize, usize, usize)> + 'a {
    (0..count.2)
    .flat_map(move |z| (0..count.1)
    .flat_map(move |y| (0..count.0)
    .map(move |x| (x, y, z))))
}
