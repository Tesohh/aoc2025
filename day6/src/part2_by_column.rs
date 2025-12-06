use aoc::{Grid, GridExt};

use crate::{Calculation, Op};

pub fn part2_by_column(grid: &Grid<char>) -> usize {
    let mut grand_total = 0;

    // why in 0..grid.height()??? my own library is confusing me
    let mut calc = Calculation {
        operands: vec![],
        operation: Op::None,
    };

    for x in 0..grid.height() {
        let mut buf = String::new();

        for y in 0..grid.width() {
            let cell = grid.at((x, y));

            if cell.is_ascii_digit() {
                buf.push(*cell);
            } else if *cell == '*' {
                calc.operation = Op::Mult;
            } else if *cell == '+' {
                calc.operation = Op::Add;
            }
        }

        if buf.is_empty() {
            grand_total += calc.total();
            calc.operands.clear();
            calc.operation = Op::None;
        } else {
            calc.operands.push(buf.parse().unwrap());
        }
    }
    grand_total += calc.total(); // flush out the last calculation

    grand_total
}
