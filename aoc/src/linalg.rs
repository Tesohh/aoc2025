use std::ops::{Add, Div, Mul, Sub};

pub trait Vec {
    type Scalar: Add + Sub + Mul + Div + Clone + Copy;
    const LENGTH: usize;

    fn get(&self, index: usize) -> Self::Scalar;
    fn set(&mut self, index: usize, value: Self::Scalar);

    fn dim(&self) -> usize {
        Self::LENGTH
    }

    // dot
    // magnitude
    // angle
    // add, sub, mul, div
}

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vec2i {
    pub x: i64,
    pub y: i64,
}

impl Vec for Vec2i {
    type Scalar = i64;
    const LENGTH: usize = 2;

    fn get(&self, index: usize) -> Self::Scalar {
        match index {
            0 => self.x,
            1 => self.y,
            _ => panic!("tried to access out of bounds element {index} of Vec2i"),
        }
    }

    fn set(&mut self, index: usize, value: Self::Scalar) {
        match index {
            0 => self.x = value,
            1 => self.y = value,
            _ => panic!("tried to access out of bounds element {index} of Vec2i"),
        }
    }
}
