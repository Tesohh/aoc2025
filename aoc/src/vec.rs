use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

#[derive(Debug, Hash, Copy, Clone, PartialEq, PartialOrd)]
pub struct Vec2 {
    pub x: isize,
    pub y: isize,
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: 0, y: 0 };

    pub const UP: Vec2 = Vec2 { x: 0, y: -1 };
    pub const DOWN: Vec2 = Vec2 { x: 0, y: 1 };
    pub const LEFT: Vec2 = Vec2 { x: -1, y: 0 };
    pub const RIGHT: Vec2 = Vec2 { x: 1, y: 0 };

    pub const DIRECTIONS: [Vec2; 4] = [Vec2::UP, Vec2::DOWN, Vec2::LEFT, Vec2::RIGHT];

    pub const fn new(x: isize, y: isize) -> Self {
        Vec2 { x, y }
    }

    pub const fn splat(v: isize) -> Self {
        Vec2 { x: v, y: v }
    }
}

impl From<(isize, isize)> for Vec2 {
    fn from((x, y): (isize, isize)) -> Self {
        Self { x, y }
    }
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Div for Vec2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
        }
    }
}
