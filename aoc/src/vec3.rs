use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::Number;

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vec3<T: Number> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Vec3i = Vec3<isize>;
pub type Vec3u = Vec3<usize>;
pub type Vec3f = Vec3<f64>;

impl Vec3<isize> {
    // TODO: add direction units here
}

impl TryInto<Vec3u> for Vec3i {
    type Error = ();

    fn try_into(self) -> Result<Vec3u, Self::Error> {
        if self.x < 0 || self.y < 0 || self.z < 0 {
            Err(())
        } else {
            Ok(Vec3u::new(
                self.x as usize,
                self.y as usize,
                self.z as usize,
            ))
        }
    }
}

impl TryInto<Vec3i> for Vec3u {
    type Error = ();

    fn try_into(self) -> Result<Vec3i, Self::Error> {
        if self.x > isize::MAX as usize
            || self.y > isize::MAX as usize
            || self.z > isize::MAX as usize
        {
            Err(())
        } else {
            Ok(Vec3i::new(
                self.x as isize,
                self.y as isize,
                self.z as isize,
            ))
        }
    }
}

impl<T: Number> Vec3<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }

    pub const fn splat(v: T) -> Self {
        Vec3 { x: v, y: v, z: v }
    }

    // to get true distance, sqrt it
    pub fn squared_distance(&self, rhs: Self) -> T {
        let side1 = (self.x - rhs.x) * (self.x - rhs.x);
        let side2 = (self.y - rhs.y) * (self.y - rhs.y);
        let side3 = (self.z - rhs.z) * (self.z - rhs.z);
        side1 + side2 + side3
    }
}

impl<T: Number> From<(T, T, T)> for Vec3<T> {
    fn from((x, y, z): (T, T, T)) -> Self {
        Self { x, y, z }
    }
}

impl<T: Number> Display for Vec3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl<T: Number> Add for Vec3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Number> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Number> Mul for Vec3<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T: Number> Div for Vec3<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<T: Number + Neg<Output = T>> Neg for Vec3<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
