use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

pub trait Number:
    Sized
    + Display
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Copy
    + PartialEq
    + PartialOrd
{
}

impl<T> Number for T where
    T: Sized
        + Display
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Copy
        + PartialEq
        + PartialOrd
{
}

#[derive(Debug, Hash, Copy, Clone, PartialEq, PartialOrd)]
pub struct Vec2<T>
where
    T: Number,
{
    pub x: T,
    pub y: T,
}

impl Vec2<isize> {
    pub const ZERO: Vec2<isize> = Vec2 { x: 0, y: 0 };

    pub const UP: Vec2<isize> = Vec2 { x: 0, y: -1 };
    pub const DOWN: Vec2<isize> = Vec2 { x: 0, y: 1 };
    pub const LEFT: Vec2<isize> = Vec2 { x: -1, y: 0 };
    pub const RIGHT: Vec2<isize> = Vec2 { x: 1, y: 0 };

    pub const DIRECTIONS: [Vec2<isize>; 4] = [Vec2::UP, Vec2::DOWN, Vec2::LEFT, Vec2::RIGHT];
}

impl<T: Number> Vec2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }

    pub const fn splat(v: T) -> Self {
        Vec2 { x: v, y: v }
    }
}

impl<T: Number> From<(T, T)> for Vec2<T> {
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl<T: Number> Display for Vec2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: Number> Add for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Number> Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Number> Mul for Vec2<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T: Number> Div for Vec2<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}
