use aoc::{Grid, GridExt};
use itertools::Itertools;

#[derive(Debug)]
enum Op {
    None,
    Add,
    Mult,
}

#[derive(Debug)]
struct Calculation {
    pub operands: Vec<usize>,
    pub operation: Op,
}

impl Calculation {
    pub fn total(&self) -> usize {
        match self.operation {
            Op::None => panic!("invalid None operation"),
            Op::Add => self.operands.iter().sum::<usize>(),
            Op::Mult => self.operands.iter().product::<usize>(),
        }
    }
}

// Today it's a parsing problem

fn part1(lines: &[String]) -> usize {
    let mut calcs: Vec<Calculation> = vec![];

    for _ in 0..lines
        .first()
        .unwrap()
        .split(" ")
        .filter(|s| !s.is_empty())
        .count()
    {
        calcs.push(Calculation {
            operands: vec![],
            operation: Op::None,
        });
    }

    for line in lines.iter().take(lines.len() - 1) {
        let numbers = line.split(" ").filter(|s| !s.is_empty()).collect_vec();

        for (j, str) in numbers.iter().enumerate() {
            let number = str.parse().unwrap();
            calcs.get_mut(j).unwrap().operands.push(number);
        }
    }

    for (j, str) in lines
        .last()
        .unwrap()
        .split(" ")
        .filter(|s| !s.is_empty())
        .enumerate()
    {
        let op = match str {
            "+" => Op::Add,
            "*" => Op::Mult,
            _ => panic!("invalid operation"),
        };
        calcs.get_mut(j).unwrap().operation = op;
    }

    calcs.iter().map(|c| c.total()).sum::<usize>()
}

fn split_string_on_multiple_pos(s: &str, positions: &[usize]) -> Vec<String> {
    let mut last = 0;
    let mut splits = vec![];

    for pos in positions {
        splits.push(s[last..*pos - 1].to_string());
        last = *pos;
    }
    splits.push(s[last..].to_string());
    splits
}

fn part2(lines: &[String]) -> usize {
    // find all positions where we have a * or +. that is where the columns start.
    let op_positions = lines
        .last()
        .unwrap()
        .chars()
        .enumerate()
        .filter(|&(_, c)| c == '+' || c == '*')
        .map(|(i, _)| i)
        .collect_vec();

    // split all lines on the op positions and build a Grid<&str>
    // where each line represents an operation
    let mut grid: Grid<String> = vec![];
    for line in lines.iter().take(lines.len() - 1) {
        grid.push(split_string_on_multiple_pos(line, &op_positions[1..]));
    }

    let grid = grid.transpose();

    // build operations.

    0
}

fn main() {
    let lines = aoc::Input::from_args().lines();
    println!("part1: {}", part1(&lines));
    println!("part2: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_splitter() {
        assert_eq!(
            split_string_on_multiple_pos("gu ten morgen", &[3, 7]),
            ["gu", "ten", "morgen"]
        );
    }
}
