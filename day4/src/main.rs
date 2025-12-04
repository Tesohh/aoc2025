use aoc::{Grid, GridExt, Vec2i, Vec2u};

fn get_rolls(grid: &Grid<char>) -> Vec<Vec2u> {
    let mut cells: Vec<Vec2u> = vec![];
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == '@' {
                cells.push(Vec2u::new(x, y));
            }
        }
    }
    cells
}

fn check_adjacent_rolls(grid: &Grid<char>, cell: Vec2u) -> usize {
    let checks = [
        Vec2i::UP + Vec2i::LEFT,
        Vec2i::UP,
        Vec2i::UP + Vec2i::RIGHT,
        Vec2i::RIGHT,
        Vec2i::DOWN + Vec2i::RIGHT,
        Vec2i::DOWN,
        Vec2i::DOWN + Vec2i::LEFT,
        Vec2i::LEFT,
    ];

    let icell: Vec2i = cell.try_into().unwrap();
    let mut roll_count: usize = 0;

    for direction in checks {
        let check = icell + direction;
        match grid.safe_at(check) {
            Some('@') => roll_count += 1,
            _ => continue,
        };
    }

    roll_count
}

const ALLOWED_ROLLS: usize = 4;

fn main() {
    let mut grid = aoc::Input::from_args().char_grid();
    let initial_rolls_len = get_rolls(&grid).len();

    let mut old_rolls: Vec<Vec2u> = vec![];
    loop {
        let rolls = get_rolls(&grid);
        if rolls == old_rolls {
            break;
        }
        old_rolls = rolls.clone();

        let valid_rolls: Vec<Vec2u> = rolls
            .into_iter()
            .filter(|&cell| check_adjacent_rolls(&grid, cell) < ALLOWED_ROLLS)
            .collect();

        for roll in valid_rolls {
            match grid.safe_at_mut(roll) {
                Some(cell) => *cell = 'x',
                None => todo!(),
            };
        }
    }

    let rolls = get_rolls(&grid);
    println!("{}", initial_rolls_len - rolls.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_rolls() {
        let grid = aoc::Input::from_string("@@.\n@..\n@@@").char_grid();
        assert_eq!(
            get_rolls(&grid)
                .iter()
                .map(|v| (v.x, v.y))
                .collect::<Vec<(usize, usize)>>(),
            [(0, 0), (1, 0), (0, 1), (0, 2), (1, 2), (2, 2)]
        );
    }

    #[test]
    fn test_check_valid_roll() {
        let grid = aoc::Input::from_string("@@.\n@..\n@@@").char_grid();
        assert!(check_adjacent_rolls(&grid, Vec2u::new(1, 1)) >= ALLOWED_ROLLS);

        let grid = aoc::Input::from_string("@@.\n@..\n...").char_grid();
        assert!(check_adjacent_rolls(&grid, Vec2u::new(1, 1)) < ALLOWED_ROLLS);

        let grid = aoc::Input::from_string("@@@\n@@@\n@@@").char_grid();
        assert!(check_adjacent_rolls(&grid, Vec2u::new(0, 0)) < ALLOWED_ROLLS);
    }

    #[test]
    fn test_directions() {
        let checks = [
            Vec2i::UP + Vec2i::LEFT,
            Vec2i::UP,
            Vec2i::UP + Vec2i::RIGHT,
            Vec2i::RIGHT,
            Vec2i::DOWN + Vec2i::RIGHT,
            Vec2i::DOWN,
            Vec2i::DOWN + Vec2i::LEFT,
            Vec2i::LEFT,
        ];

        let grid = aoc::Input::from_path(".inputs/smallest".into()).char_grid();

        let cell = Vec2i::new(1, 1);

        let mut checked = vec![];
        for direction in checks {
            let check = cell + direction;
            checked.push(((check.x, check.y), grid.safe_at(check)));
        }

        /*
        ..@.
        @!@.
        @@..
        ..@. */

        assert_eq!(
            checked,
            [
                ((0, 0), Some(&'.')),
                ((1, 0), Some(&'.')),
                ((2, 0), Some(&'@')),
                ((2, 1), Some(&'@')),
                ((2, 2), Some(&'.')),
                ((1, 2), Some(&'@')),
                ((0, 2), Some(&'@')),
                ((0, 1), Some(&'@')),
            ]
        );

        let cell = Vec2i::new(0, 0);

        let mut checked = vec![];
        for direction in checks {
            let check = cell + direction;
            checked.push(((check.x, check.y), grid.safe_at(check)));
        }

        /*
        !.@.
        @.@.
        @@..
        ..@. */

        assert_eq!(
            checked,
            [
                ((-1, -1), None),
                ((0, -1), None),
                ((1, -1), None),
                ((1, 0), Some(&'.')),
                ((1, 1), Some(&'.')),
                ((0, 1), Some(&'@')),
                ((-1, 1), None),
                ((-1, 0), None),
            ]
        )
    }
}
