use std::collections::HashSet;

use aoc::{Grid, GridExt, Vec2i, Vec2u};

pub fn part1(grid: &Grid<char>, start: &Vec2u) -> usize {
    let start: Vec2i = (*start).try_into().unwrap();

    let mut tachyons: HashSet<Vec2i> = HashSet::new();
    tachyons.insert(start + Vec2i::DOWN);

    let mut splits = 0;
    for _ in 0..grid.height() {
        let mut new_tachyons: HashSet<Vec2i> = HashSet::new();

        for tachyon in &tachyons {
            let candidate = *tachyon + Vec2i::DOWN;

            match grid.safe_at(candidate) {
                Some(&'^') => {
                    new_tachyons.insert(candidate + Vec2i::LEFT);
                    new_tachyons.insert(candidate + Vec2i::RIGHT);
                    splits += 1
                }
                Some(&'.') => {
                    new_tachyons.insert(candidate);
                }
                None => break,
                _ => panic!("invalid char found"),
            };
        }

        tachyons = new_tachyons;
    }

    splits
}
