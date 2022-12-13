use glam::Vec3;

// TODO create a cool macro wrapping Vectors::new(vec![points]) as points![]

// abstraction around a list of vectors.
// allows us to easily access function operating on lists of points, and to render them
// TODO, maybe add a more performant internal data structure, like a flat buffer? Would that even be more performant?
pub struct Points {
    pub data: Vec<Vec3>,
}

impl Points {
    pub fn new(data: Vec<Vec3>) -> Self {
        Self { data }
    }

    pub fn from_vec_of_arrays(vec: Vec<[f32; 3]>) -> Points {
        Self::new(vec.iter().map(|v| Vec3::from_array(*v)).collect())
    }

    pub fn to_vec_of_arrays(self) -> Vec<[f32; 3]> {
        self.into()
    }
}

impl From<Points> for Vec<Vec3> {
    fn from(points: Points) -> Self {
        points.data
    }
}

impl From<Points> for Vec<[f32; 3]> {
    fn from(points: Points) -> Self {
        points.data.iter().map(|v| v.to_array()).collect()
    }
}
