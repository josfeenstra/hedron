use glam::{UVec2, UVec3};

#[inline]
pub fn iter_xy_u<'a>(count: UVec2) -> impl Iterator<Item = UVec2> + 'a {
    (0..count.y).flat_map(move |y| (0..count.x).map(move |x| UVec2::new(x, y)))
}

#[inline]
pub fn iter_xyz_u<'a>(count: UVec3) -> impl Iterator<Item = UVec3> + 'a {
    (0..count.z).flat_map(move |z| {
        (0..count.y).flat_map(move |y| (0..count.x).map(move |x| UVec3::new(x, y, z)))
    })
}

#[inline]
pub fn iter_xy<'a>(count: (usize, usize)) -> impl Iterator<Item = (usize, usize)> + 'a {
    (0..count.1).flat_map(move |y| (0..count.0).map(move |x| (x, y)))
}

#[inline]
pub fn iter_xyz<'a>(
    count: (usize, usize, usize),
) -> impl Iterator<Item = (usize, usize, usize)> + 'a {
    (0..count.2)
        .flat_map(move |z| (0..count.1)
        .flat_map(move |y| (0..count.0)
        .map(move |x| (x, y, z))
    ))
}

/// NOTE: this wraps around: given 3, this gives: (0, 1), (1, 2), and (2, 0)
pub fn iter_pair_ids<'a>(count: usize) -> impl Iterator<Item = (usize, usize)> + 'a {
    (0..count).map(move |i| (i, (i + 1) % count))
}

pub fn iter_triplet_ids<'a>(count: usize) -> impl Iterator<Item = (usize, usize, usize)> + 'a {
    (0..count).map(move |i| (i, (i + 1) % count, (i + 2) % count))
}

/// NOTE: this wraps around: given 3, this gives: (0, 1), (1, 2), and (2, 0)
pub fn iter_pairs<'a, T>(items: &'a Vec<T>) -> impl Iterator<Item = (&T, &T)> + 'a {
    iter_pair_ids(items.len()).map(|(a, b)| (items.get(a).unwrap(), items.get(b).unwrap()))
}

pub fn iter_triplets<'a, T>(items: &'a Vec<T>) -> impl Iterator<Item = (&T, &T, &T)> + 'a {
    iter_triplet_ids(items.len()).map(|(a, b, c)| {
        (
            items.get(a).unwrap(),
            items.get(b).unwrap(),
            items.get(c).unwrap(),
        )
    })
}
