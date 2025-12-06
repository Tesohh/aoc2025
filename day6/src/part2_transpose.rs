use crate::Calculation;
use crate::Op;
use aoc::{Grid, GridExt};
use itertools::Itertools;

pub fn split_string_on_multiple_pos(s: &str, positions: &[usize]) -> Vec<String> {
    let mut last = 0;
    let mut splits = vec![];

    for pos in positions {
        splits.push(s[last..*pos - 1].to_string());
        last = *pos;
    }
    splits.push(s[last..].to_string());
    splits
}

pub fn to_ceph_numbers(raw_numbers: &[String]) -> Vec<usize> {
    let mini_grid = raw_numbers
        .iter()
        .map(|s| s.chars().collect_vec())
        .collect_vec();

    let mini_grid = mini_grid.transpose();

    mini_grid
        .iter()
        .map(|chars| chars.iter().collect::<String>().trim().to_string())
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec()
}

pub fn part2_transpose(lines: &[String]) -> usize {
    // find all positions where we have a * or +. that is where the columns start.
    let op_positions = lines
        .last()
        .unwrap()
        .chars()
        .enumerate()
        .filter(|&(_, c)| c == '+' || c == '*')
        .map(|(i, _)| i)
        .collect_vec();

    let operations = lines
        .last()
        .unwrap()
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| match s {
            "+" => Op::Add,
            "*" => Op::Mult,
            _ => panic!("invalid operation"),
        })
        .collect_vec();

    // split all lines on the op positions and build a Grid<&str>
    // where each line represents an operation
    let mut grid: Grid<String> = vec![];
    for line in lines.iter().take(lines.len() - 1) {
        grid.push(split_string_on_multiple_pos(line, &op_positions[1..]));
    }

    // now we have a grid where every row contains all number strings in a calculation
    // note: the numbers here are in "human" form, not cephalopod form
    let grid = grid.transpose();

    // build cephalopod numbers
    let mut grand_total = 0;

    for (y, row) in grid.iter().enumerate() {
        let ceph_numbers = to_ceph_numbers(row);
        let calc = Calculation {
            operands: ceph_numbers,
            operation: operations[y].clone(),
        };

        grand_total += calc.total();
    }

    grand_total
}
