use aoc::{Grid, GridExt};
use itertools::Itertools;

#[derive(Debug, Clone)]
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

fn to_ceph_numbers(raw_numbers: &[String]) -> Vec<usize> {
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
    dbg!(&grid);

    // build cephalopod numbers
    let mut grand_total = 0;

    for (y, row) in grid.iter().enumerate() {
        let ceph_numbers = to_ceph_numbers(row);
        dbg!(&ceph_numbers);
        let calc = Calculation {
            operands: ceph_numbers,
            operation: operations[y].clone(),
        };

        dbg!(&calc.operation);
        grand_total += dbg!(calc.total());
        println!()
    }

    grand_total
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

    #[test]
    fn test_ceph_numbers() {
        assert_eq!(
            to_ceph_numbers(&["64 ".into(), "23 ".into(), "314".into()]),
            [623, 431, 4]
        );

        assert_eq!(
            to_ceph_numbers(&[" 51".into(), "387".into(), "215".into()]),
            [32, 581, 175]
        );

        assert_eq!(
            to_ceph_numbers(&["328".into(), "64 ".into(), "98 ".into()]),
            [369, 248, 8]
        );
    }
}
