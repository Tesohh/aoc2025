use crate::{Vec2, Vec2u};

// assumes all rows are of same length.
pub type Grid<T> = Vec<Vec<T>>;

pub trait GridExt<T: Clone> {
    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn is_out(&self, point: impl TryInto<Vec2u>) -> bool;

    fn safe_at(&self, point: impl TryInto<Vec2u>) -> Option<&T>;
    fn safe_at_mut(&mut self, point: impl TryInto<Vec2u>) -> Option<&mut T>;

    fn at(&self, point: impl TryInto<Vec2u>) -> &T {
        self.safe_at(point).unwrap()
    }

    fn at_mut(&mut self, point: impl TryInto<Vec2u>) -> &mut T {
        self.safe_at_mut(point).unwrap()
    }

    // creates a second grid where every element v_{xy} becomes v_{yx}
    fn transpose(&self) -> Grid<T>;
}

impl<T: Clone> GridExt<T> for Vec<Vec<T>> {
    fn width(&self) -> usize {
        self.len()
    }

    fn height(&self) -> usize {
        self[0].len()
    }

    fn is_out(&self, point: impl TryInto<Vec2u>) -> bool {
        let Ok(Vec2 { x, y }) = point.try_into() else {
            return false;
        };

        // if self.height() == 0 || self.height() == 0 {}

        let x_out = x > self.height() - 1;
        let y_out = y > self.width() - 1;

        x_out || y_out
    }

    fn safe_at(&self, point: impl TryInto<Vec2u>) -> Option<&T> {
        let Ok(vec) = point.try_into() else {
            return None;
        };

        if self.is_out(vec) {
            return None;
        }

        self.get(vec.y)?.get(vec.x)
    }

    fn safe_at_mut(&mut self, point: impl TryInto<Vec2u>) -> Option<&mut T> {
        let Ok(vec) = point.try_into() else {
            return None;
        };

        if self.is_out(vec) {
            return None;
        }

        self.get_mut(vec.y)?.get_mut(vec.x)
    }

    fn transpose(&self) -> Grid<T> {
        let mut transposed: Grid<T> = vec![];

        for _ in 0..self.height() {
            transposed.push(vec![]);
        }

        for row in self.iter() {
            for (x, cell) in row.iter().enumerate() {
                transposed[x].push(cell.clone());
            }
        }

        transposed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expect = vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];
        assert_eq!(grid.transpose(), expect);
    }
}
