use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
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

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vec2<T: Number> {
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

impl<T: Number> Vec2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }

    pub const fn splat(v: T) -> Self {
        Vec2 { x: v, y: v }
    }
}

impl<T: Number> From<(T, T)> for Vec2<T> {
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
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

impl<T: Number + Neg<Output = T>> Neg for Vec2<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
        }
    }
}
