// we need a different struct, since a bunch of vectors will need to be rendered as a bunch of arrows

use glam::Vec3;

use super::Points;

pub struct Vectors {
    data: Vec<Vec3>,
}

impl Vectors {
    pub fn new(data: Vec<Vec3>) -> Self {
        Self { data }
    }
}

impl From<Points> for Vectors {
    fn from(points: Points) -> Self {
        Self { data: points.data }
    }
}
