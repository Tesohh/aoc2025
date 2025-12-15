pub mod vec;
pub use vec::*;

pub mod vec3;
pub use vec3::*;

pub mod geometry;
pub use geometry::*;

pub mod grid;
pub use grid::*;

pub mod input;
pub use input::*;

use num_traits::Num;

pub trait Scalar: Num + Clone + Copy + std::fmt::Display + PartialOrd + Ord + PartialEq {}

impl<T> Scalar for T where T: Num + Clone + Copy + std::fmt::Display + PartialOrd + Ord + PartialEq {}
