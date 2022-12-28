use crate::kernel::{fxx, PI};
use glam::IVec3;

// just a basic representation of orthogonal rotations
#[derive(Clone, Copy, PartialEq, Default)]
pub enum Rot {
    #[default]
    Rot0,
    Rot90,
    Rot180,
    Rot270,
}

impl Rot {
    pub fn rad(&self) -> fxx {
        match self {
            Rot::Rot0 => 0.0,
            Rot::Rot90 => PI * 0.5,
            Rot::Rot180 => PI,
            Rot::Rot270 => PI * 1.5,
        }
    }

    pub fn next(self) -> Self {
        match self {
            Rot::Rot0 => Rot::Rot90,
            Rot::Rot90 => Rot::Rot180,
            Rot::Rot180 => Rot::Rot270,
            Rot::Rot270 => Rot::Rot0,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Rot::Rot0 => Rot::Rot270,
            Rot::Rot90 => Rot::Rot0,
            Rot::Rot180 => Rot::Rot90,
            Rot::Rot270 => Rot::Rot180,
        }
    }

    pub fn flip(self) -> Self {
        match self {
            Rot::Rot0 => Rot::Rot0,
            Rot::Rot90 => Rot::Rot270,
            Rot::Rot180 => Rot::Rot180,
            Rot::Rot270 => Rot::Rot90,
        }
    }

    pub fn mul(vec: IVec3, rot: Rot) -> IVec3 {
        match rot {
            Rot::Rot0 => vec,
            Rot::Rot90 => IVec3::new(vec.y, -vec.x, vec.z), // ??
            Rot::Rot180 => vec * -1,
            Rot::Rot270 => IVec3::new(-vec.y, vec.x, vec.z), // ??
        }
    }

    pub fn iter() -> [Rot; 4] {
        [Rot::Rot0, Rot::Rot90, Rot::Rot180, Rot::Rot270]
    }
}

// impl Mul<Rot> for IVec3 {
//     type Output = IVec3;

//     fn mul(self, rot: Rot) -> Self::Output {
//         match rot {
//             Rot::Rot0 => self,
//             Rot::Rot90 => Self::new(self.y, -self.x, self.z), // ??
//             Rot::Rot180 => self * -1,
//             Rot::Rot270 => Self::new(-self.y, self.x, self.z), // ??
//         }
//     }
// }
