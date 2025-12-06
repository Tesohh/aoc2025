use crate::Calculation;
use crate::Op;
use itertools::Itertools;

pub fn part1(lines: &[String]) -> usize {
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
