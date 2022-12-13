// /**
// A unsigned integer coordinate, useful for griddy games
//  */

// use std::{ops::{Add, Sub}, fmt::{Display, Formatter, self}, convert::TryInto};

// use glam::{Component, UVec2, IVec2};

// use crate::math::D8;

// #[derive(Debug, Default, Copy, Clone, Ord, PartialEq, PartialOrd, Eq, Hash)]
// #[derive(Component)]
// #[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
// pub struct GridPos {
//     pub x: u16,
//     pub y: u16,
// }

// impl GridPos {

//     pub fn new(x: u16, y: u16) -> Self {
//         Self { x, y }
//     }

//     pub fn from_ivec(ivec: IVec2) -> Self {
//         Self { 
//             x: ivec.x.try_into().unwrap(),
//             y: ivec.y.try_into().unwrap()
//         }
//     }

//     pub fn dis_squared(&self, rhs: Self) -> u16 {
//         (self.x + rhs.x).pow(2) + (self.y + rhs.y).pow(2)
//     }

//     pub fn for_each_surrounding(coord: GridPos) -> impl Iterator<Item = GridPos> {
//         let something = D8::ALL
//             .iter()
//             .copied()
//             .map(move |dir| coord + dir.xy());
//         something
//     }  

//     pub fn to_ivec(&self) -> IVec2 {
//         IVec2::new(self.x as i32, self.y as i32)
//     }
// }

// impl Add for GridPos {

//     type Output = Self;

//     fn add(self, rhs: Self) -> Self::Output {
//         Self {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//         }
//     }
// }

// impl Sub for GridPos {

//     type Output = Self;
    
//     fn sub(self, rhs: Self) -> Self::Output {
//         Self {
//             x: self.x - rhs.x,
//             y: self.y - rhs.y,
//         }
//     }
// }

// impl Add<(i8, i8)> for GridPos {
//     type Output = Self;

//     fn add(self, (x, y): (i8, i8)) -> Self::Output {
//         let x = ((self.x as i16) + x as i16) as u16;
//         let y = ((self.y as i16) + y as i16) as u16;
//         Self { x, y }
//     }
// }

// impl Display for GridPos {

//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         write!(f, "({}, {})", self.x, self.y)
//     }
// }