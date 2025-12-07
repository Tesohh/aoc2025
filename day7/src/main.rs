use aoc::Vec2u;
use day7::part1::part1;
use day7::part2::part2;

fn main() {
    let grid = aoc::Input::from_args().char_grid();
    let start = grid[0]
        .iter()
        .enumerate()
        .find(|&(_, &c)| c == 'S')
        .map(|(i, _)| Vec2u::new(i, 0))
        .unwrap();

    println!("part1: {}", part1(&grid, &start));
    println!("part2: {}", part2(&grid, &start));
}
