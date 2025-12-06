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

fn main() {
    let lines = aoc::Input::from_args().lines();
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

    let mut grand_total = 0;
    for calc in calcs {
        grand_total += match calc.operation {
            Op::None => panic!("invalid None operation"),
            Op::Add => calc.operands.iter().sum::<usize>(),
            Op::Mult => calc.operands.iter().product::<usize>(),
        }
    }

    println!("part1: {}", grand_total);
}
