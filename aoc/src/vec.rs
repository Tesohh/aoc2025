use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::Scalar;

#[derive(Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vec2<T: Scalar> {
    pub x: T,
    pub y: T,
}

pub type Vec2i = Vec2<isize>;
pub type Vec2u = Vec2<usize>;
pub type Vec2f = Vec2<f64>;

impl Vec2<isize> {
    pub const ZERO: Vec2<isize> = Vec2 { x: 0, y: 0 };

    pub const UP: Vec2<isize> = Vec2 { x: 0, y: -1 };
    pub const DOWN: Vec2<isize> = Vec2 { x: 0, y: 1 };
    pub const LEFT: Vec2<isize> = Vec2 { x: -1, y: 0 };
    pub const RIGHT: Vec2<isize> = Vec2 { x: 1, y: 0 };

    pub const DIRECTIONS: [Vec2<isize>; 4] = [Vec2::UP, Vec2::DOWN, Vec2::LEFT, Vec2::RIGHT];
}

impl TryInto<Vec2u> for Vec2i {
    type Error = ();

    fn try_into(self) -> Result<Vec2u, Self::Error> {
        if self.x < 0 || self.y < 0 {
            Err(())
        } else {
            Ok(Vec2u::new(self.x as usize, self.y as usize))
        }
    }
}

impl<T: Scalar> Debug for Vec2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec2{{{}, {}}}", self.x, self.y)
    }
}

impl TryInto<Vec2i> for Vec2u {
    type Error = ();

    fn try_into(self) -> Result<Vec2i, Self::Error> {
        if self.x > isize::MAX as usize || self.y > isize::MAX as usize {
            Err(())
        } else {
            Ok(Vec2i::new(self.x as isize, self.y as isize))
        }
    }
}

impl<T: Scalar> Vec2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }

    pub const fn splat(v: T) -> Self {
        Vec2 { x: v, y: v }
    }

    // to get true distance, sqrt it
    pub fn squared_distance(&self, rhs: Self) -> T {
        let side1 = (self.x - rhs.x) * (self.x - rhs.x);
        let side2 = (self.y - rhs.y) * (self.y - rhs.y);
        side1 + side2
    }
}

impl<T: Scalar> From<(T, T)> for Vec2<T> {
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
}

impl<T: Scalar> Display for Vec2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: Scalar> Add for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Scalar> Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Scalar> Mul for Vec2<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T: Scalar> Div for Vec2<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T: Scalar + Neg<Output = T>> Neg for Vec2<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
        }
    }
}
