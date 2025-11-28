use crate::{Vec2, Vec2i, Vec2u};

// assumes all rows are of same length.
pub type Grid<T> = Vec<Vec<T>>;

pub trait GridExt<T> {
    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn is_out(&self, point: impl Into<Vec2u>) -> bool;

    fn safe_at(&self, point: impl Into<Vec2u>) -> Option<&T>;
    fn safe_at_mut(&mut self, point: impl Into<Vec2u>) -> Option<&mut T>;

    fn at(&self, point: impl Into<Vec2u>) -> &T {
        self.safe_at(point).unwrap()
    }

    fn at_mut(&mut self, point: impl Into<Vec2u>) -> &mut T {
        self.safe_at_mut(point).unwrap()
    }
}

impl<T> GridExt<T> for Vec<Vec<T>> {
    fn width(&self) -> usize {
        self.len()
    }

    fn height(&self) -> usize {
        self[0].len()
    }

    fn is_out(&self, point: impl Into<Vec2u>) -> bool {
        let Vec2 { x, y } = point.into();
        let x_out = x > self.height() - 1;
        let y_out = y > self.width() - 1;

        x_out || y_out
    }

    fn safe_at(&self, point: impl Into<Vec2u>) -> Option<&T> {
        let vec: Vec2u = point.into();
        self.get(vec.y)?.get(vec.x)
    }

    fn safe_at_mut(&mut self, point: impl Into<Vec2u>) -> Option<&mut T> {
        let vec: Vec2u = point.into();
        self.get_mut(vec.y)?.get_mut(vec.x)
    }
}

fn iwefj() {
    let g: Grid<char> = vec![];
    g.at((2, 2));
}
