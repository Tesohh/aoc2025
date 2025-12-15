use std::cmp::min;
use std::{cmp::max, fmt::Debug};

use num_traits::Float;

use crate::{Scalar, Vec2};

#[derive(Clone, Copy, PartialEq, Eq)]
/// Technically a line segment
pub struct Line2D<T: Scalar>(Vec2<T>, Vec2<T>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Polygon<T: Scalar>(Vec<Line2D<T>>);

impl<T: Scalar> Line2D<T> {
    pub fn new(p: Vec2<T>, q: Vec2<T>) -> Self {
        Self(p, q)
    }
    /// returns the length of this line, squared.
    /// if you need the length not squared, either sqrt it yourself
    /// or use the `length` method on `Line2D<T: Number + Float>`
    pub fn squared_length(&self) -> T {
        let side1 = (self.0.x - self.1.x) * (self.0.x - self.1.x);
        let side2 = (self.0.y - self.1.y) * (self.0.y - self.1.y);
        side1 + side2
    }

    pub fn includes_point(&self, p: Vec2<T>) -> bool {
        p.x <= max(self.0.x, self.1.x)
            && p.x >= min(self.0.x, self.1.x)
            && p.y <= max(self.0.y, self.1.y)
            && p.y >= min(self.0.y, self.1.y)
            && orientation(self.0, self.1, p) == Orientation::Collinear
    }

    // https://www.geeksforgeeks.org/dsa/check-if-two-given-line-segments-intersect/
    pub fn intersects(&self, other: &Self) -> bool {
        // in gfg: points[0] = self, points[1] = other
        let orientations = [
            orientation(self.0, self.1, other.0),
            orientation(self.0, self.1, other.1),
            orientation(other.0, other.1, self.0),
            orientation(other.0, other.1, self.1),
        ];

        // general case
        if orientations[0] != orientations[1] && orientations[2] != orientations[3] {
            return true;
        }

        if orientations[0] == Orientation::Collinear && self.includes_point(other.0) {
            return true;
        }
        if orientations[1] == Orientation::Collinear && self.includes_point(other.1) {
            return true;
        }
        if orientations[2] == Orientation::Collinear && other.includes_point(self.0) {
            return true;
        }
        if orientations[3] == Orientation::Collinear && other.includes_point(self.1) {
            return true;
        }

        false
    }

    /// casts a ray from `p` to the right. would the ray intersect the line?
    pub fn intersected_by_ray(&self, p: &Vec2<T>) -> bool {
        if self.0.y == self.1.y {
            // horizontal line. avoid divide by 0
            // return p.y == self.0.y;
            return false;
        }

        //     if (y0 > p.y) == (y1 > p.y) {
        //         return false;
        //     }

        let sects_y = (p.y < self.0.y) != (p.y < self.1.y);
        let sects_x =
            p.x <= (p.y - self.0.y) * (self.1.x - self.0.x) / (self.1.y - self.0.y) + self.0.x;

        sects_y && sects_x
    }
}

#[derive(PartialEq, PartialOrd)]
enum Orientation {
    Clockwise,
    Counterclockwise,
    Collinear,
}

fn orientation<T: Scalar>(p: Vec2<T>, q: Vec2<T>, r: Vec2<T>) -> Orientation {
    let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);

    if val == T::zero() {
        Orientation::Collinear
    } else if val < T::zero() {
        Orientation::Counterclockwise
    } else {
        Orientation::Clockwise
    }
}

impl<T: Scalar + Float> Line2D<T> {
    pub fn length(&self) -> T {
        self.squared_length().sqrt()
    }
}

impl<T: Scalar> Debug for Line2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Line2D({} -> {})", self.0, self.1)
    }
}

/// Turns a list of points into a list of lines, then a polygon
impl<T: Scalar> From<&[Vec2<T>]> for Polygon<T> {
    fn from(points: &[Vec2<T>]) -> Self {
        if points.len() < 3 {
            panic!("cannot create a polygon from less than 3 points")
        }

        let mut lines: Vec<Line2D<T>> = vec![];

        for i in 0..points.len() {
            if i == points.len() - 1 {
                lines.push(Line2D(points[i], points[0]));
            } else {
                lines.push(Line2D(points[i], points[i + 1]));
            }
        }

        Self(lines)
    }
}
impl<T: Scalar, const N: usize> From<[Vec2<T>; N]> for Polygon<T> {
    fn from(points: [Vec2<T>; N]) -> Self {
        Self::from(&points[..])
    }
}
impl<T: Scalar> From<&[Line2D<T>]> for Polygon<T> {
    fn from(value: &[Line2D<T>]) -> Self {
        Self(value.to_vec())
    }
}
impl<T: Scalar> From<Vec<Line2D<T>>> for Polygon<T> {
    fn from(value: Vec<Line2D<T>>) -> Self {
        Self(value)
    }
}
impl<T: Scalar, const N: usize> From<[Line2D<T>; N]> for Polygon<T> {
    fn from(value: [Line2D<T>; N]) -> Self {
        Self(value.to_vec())
    }
}

impl<T: Scalar> Polygon<T> {
    pub fn rect_from_opposite_corners(a: &Vec2<T>, b: &Vec2<T>) -> Self {
        Polygon::from([*a, Vec2::new(b.x, a.y), *b, Vec2::new(a.x, b.y)])
    }

    pub fn points(&self) -> impl Iterator<Item = &'_ Vec2<T>> {
        self.0.iter().map(|line| &line.0)
    }

    pub fn lines(&self) -> impl Iterator<Item = &'_ Line2D<T>> {
        self.0.iter()
    }

    pub fn includes_point(&self, p: &Vec2<T>) -> bool {
        if self.points().any(|my| *p == *my) {
            return true;
        }

        let mut counter = 0;
        for line in &self.0 {
            if line.intersected_by_ray(p) {
                counter += 1
            }
        }

        counter % 2 == 1
    }

    pub fn intersects_line(&self, l: &Line2D<T>) -> bool {
        self.lines().any(|my| my.intersects(l))
    }

    /// checks if all points of `other` are in `self`
    /// and TODO: if NO line (edge) of `other` intersects with lines of `self`
    pub fn full_overlap(&self, other: &Polygon<T>) -> bool {
        let points_ok = other.points().all(|p| self.includes_point(p));
        let lines_ok = !other.lines().any(|l| self.intersects_line(l));
        points_ok && lines_ok
    }
}

#[cfg(test)]
mod tests {
    use crate::Vec2i;

    use super::*;

    // come from AOC2025 Day 9
    const POINTS: [Vec2i; 8] = [
        Vec2i::new(7, 1),
        Vec2i::new(11, 1),
        Vec2i::new(11, 7),
        Vec2i::new(9, 7),
        Vec2i::new(9, 5),
        Vec2i::new(2, 5),
        Vec2i::new(2, 3),
        Vec2i::new(7, 3),
    ];

    #[test]
    fn test_intersection_by_ray() {
        let line = Line2D(Vec2i::new(0, 0), Vec2i::new(5, 5));
        assert!(line.intersected_by_ray(&Vec2i::new(1, 1)));
        assert!(line.intersected_by_ray(&Vec2i::new(2, 2)));
        assert!(!line.intersected_by_ray(&Vec2i::new(4, 3)));
        assert!(line.intersected_by_ray(&Vec2i::new(4, 4)));
        assert!(!line.intersected_by_ray(&Vec2i::new(4, 5)));
        assert!(!line.intersected_by_ray(&Vec2i::new(5, 5))); // NOTE: this might be a problem (5,5) doesnot intersect (0,0)->(5,5)
    }

    #[test]
    #[should_panic]
    fn test_invalid_polygon_construction() {
        let _ = Polygon::from(&POINTS[..2]);
    }

    #[test]
    fn test_polygon_construction() {
        let poly = Polygon::from(&POINTS[..3]);
        assert_eq!(
            poly.0,
            [
                Line2D(POINTS[0], POINTS[1]),
                Line2D(POINTS[1], POINTS[2]),
                Line2D(POINTS[2], POINTS[0])
            ]
        );
    }

    #[test]
    fn test_point_in_polygon() {
        let poly = Polygon::from(&POINTS[..]);
        assert!(poly.includes_point(&Vec2i::new(6, 4)));
        assert!(poly.includes_point(&Vec2i::new(6, 3)));
        assert!(!poly.includes_point(&Vec2i::new(7, 6)));
        assert!(!poly.includes_point(&Vec2i::new(999, 999)));
    }

    #[test]
    fn test_overlap() {
        let poly = Polygon::from(&POINTS[..]);

        let other = Polygon::from([Vec2i::new(8, 4), Vec2i::new(10, 4), Vec2i::new(8, 2)]);
        assert!(poly.full_overlap(&other));

        let other = Polygon::from([Vec2i::new(8, 4), Vec2i::new(10, 4), Vec2i::new(5, 2)]);
        assert!(!poly.full_overlap(&other));
    }
}
