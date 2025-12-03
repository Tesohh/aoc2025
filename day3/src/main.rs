fn highest(bank: Vec<u32>, len: usize) -> usize {
    let mut taken: Vec<(usize, u32)> = vec![];

    for k in 1..=len {
        let mut max = 0;
        let mut index = 0;

        let last_index = match taken.last() {
            Some((i, _)) => *i,
            None => 0,
        };

        for (i, joltage) in bank
            .iter()
            .take(bank.len() - len + k)
            .skip(last_index)
            .enumerate()
        {
            if *joltage > max {
                max = *joltage;
                index = i;
            }
        }

        taken.push((index, max));
        println!("[{}] {}", index, max)
    }

    0
    // taken.iter().map(|(_, x)| **x as usize).sum()
}

// if !taken
//     .iter()
//     .map(|(other_i, _)| *other_i)
//     .collect::<Vec<usize>>()
//     .contains(&i)

const LEN: usize = 2;

fn main() {
    let banks: Vec<Vec<u32>> = aoc::Input::from_args()
        .lines()
        .iter()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let mut sum = 0;

    for bank in banks {
        highest(bank, LEN);
        println!();
    }

    dbg!(sum);
}
