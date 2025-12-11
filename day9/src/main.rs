use aoc::Vec2i;
use itertools::{Itertools, iproduct};

fn main() {
    let points: Vec<Vec2i> = aoc::Input::from_args()
        .lines()
        .iter()
        .filter_map(|line| line.split_once(","))
        .map(|(a, b)| Vec2i::new(a.parse().unwrap(), b.parse().unwrap()))
        .collect();

    let mut product: Vec<(Vec2i, Vec2i, isize)> = vec![];
    for i in 0..points.len() {
        for j in 0..points.len() {
            if j > i {
                let a = points[i];
                let b = points[j];
                let area = ((b.x - a.x).abs() + 1) * ((b.y - a.y).abs() + 1);
                product.push((a, b, area));
                // product.push((a, b, ((b.x + a.x - 1) * (b.y + a.y - 1)).abs()));
            }
        }
    }
    product.sort_by_key(|&(_, _, area)| area);
    dbg!(product.last().unwrap());
}
