use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::kernel::{fxx, PI};

pub enum D4 {
    Left,
    Down,
    Right,
    Up,
}

// sides of a cube
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum D6 {
    Left,
    Down,
    Right,
    Up,
    Lower,
    Raise,
}

impl D6 {
    // TODO: check left-right correctness!!
    pub fn is_other_the_right_of_me(&self, other: Self) -> bool {
        matches!(
            (self, other),
            (D6::Up, D6::Left) | (D6::Left, D6::Down) | (D6::Down, D6::Right) | (D6::Right, D6::Up)
        )
    }

    pub fn is_other_the_left_of_me(&self, other: Self) -> bool {
        matches!(
            (self, other),
            (D6::Up, D6::Right) | (D6::Right, D6::Down) | (D6::Down, D6::Left) | (D6::Left, D6::Up)
        )
    }
}

/**
 * General purpose direction
 */
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum D8 {
    Left,
    DownLeft,
    Down,
    DownRight,
    Right,
    UpRight,
    Up,
    UpLeft,
}

impl D8 {
    pub const ALL: [D8; 8] = [
        D8::Left,
        D8::DownLeft,
        D8::Down,
        D8::DownRight,
        D8::Right,
        D8::UpRight,
        D8::Up,
        D8::UpLeft,
    ];

    pub fn from_num(i: i32) -> D8 {
        assert!(i > -1 && i < 8);
        match i {
            0 => D8::Left,
            1 => D8::DownLeft,
            2 => D8::Down,
            3 => D8::DownRight,
            4 => D8::Right,
            5 => D8::UpRight,
            6 => D8::Up,
            _ => D8::UpLeft,
        }
    }

    /**
    next direction, clockwise
    */
    pub fn next(&self) -> D8 {
        match self {
            D8::Left => D8::DownLeft,
            D8::DownLeft => D8::Down,
            D8::Down => D8::DownRight,
            D8::DownRight => D8::Right,
            D8::Right => D8::UpRight,
            D8::UpRight => D8::Up,
            D8::Up => D8::UpLeft,
            D8::UpLeft => D8::Left,
        }
    }

    /**
    previous direction, clockwise
    */
    pub fn prev(&self) -> D8 {
        match self {
            D8::Left => D8::UpLeft,
            D8::DownLeft => D8::Left,
            D8::Down => D8::DownLeft,
            D8::DownRight => D8::Down,
            D8::Right => D8::DownRight,
            D8::UpRight => D8::Right,
            D8::Up => D8::UpRight,
            D8::UpLeft => D8::Up,
        }
    }

    pub fn xy(&self) -> (i8, i8) {
        match self {
            D8::Left => (-1, 0),
            D8::Right => (1, 0),
            D8::Up => (0, -1),
            D8::Down => (0, 1),
            D8::DownLeft => (-1, 1),
            D8::DownRight => (1, 1),
            D8::UpRight => (1, -1),
            D8::UpLeft => (-1, -1),
        }
    }

    // pub fn vector(&self) -> Point {
    //     match self {
    //         Dir::Left =>  Point {x: -1 , y: 0},
    //         Dir::Right => Point {x: 1  , y: 0},
    //         Dir::Up =>    Point {x: 0  , y: -1},
    //         Dir::Down =>  Point {x: 0  , y: 1},
    //     }
    // }

    /// the angle in **degrees** in respect to the positive X axis, going counter clockwise (as conventional within certain 3d engines)
    pub fn deg(&self) -> i32 {
        match self {
            D8::Left => 0,
            D8::UpLeft => 45,
            D8::Up => 90,
            D8::UpRight => 135,
            D8::Right => 180,
            D8::DownRight => 225,
            D8::Down => 270,
            D8::DownLeft => 315,
        }
    }

    /// the angle in **radians** in respect to the positive X axis, going counter clockwise (as conventional within certain 3d engines)
    pub fn rad(&self) -> fxx {
        match self {
            D8::Left => 0.0,
            D8::UpLeft => PI * 0.25,
            D8::Up => PI * 0.5,
            D8::UpRight => PI * 0.75,
            D8::Right => PI,
            D8::DownRight => PI * 1.25,
            D8::Down => PI * 1.5,
            D8::DownLeft => PI * 1.75,
        }
    }
}

impl Distribution<D8> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> D8 {
        match rng.gen_range(0..=7) {
            0 => D8::Left,
            1 => D8::UpLeft,
            2 => D8::Up,
            3 => D8::UpRight,
            4 => D8::Right,
            5 => D8::DownRight,
            6 => D8::Down,
            _ => D8::DownLeft,
        }
    }
}

pub enum D4Traverse {
    Forward,
    RotLeft,
    RotRight,
    Reverse,
}
