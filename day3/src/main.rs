fn highest(bank: Vec<u64>, digits: u64) -> u64 {
    let len = bank.len() as u64;

    let mut max_index: Option<u64> = None;

    let mut number = 0;

    for step in 1..=digits {
        let start = match max_index {
            Some(i) => i + 1, // +1?
            None => 0,
        };

        let end = len - digits + step;

        let mut max = 0;
        for i in start..end {
            if bank[i as usize] > max {
                max = bank[i as usize];
                max_index = Some(i);
            }
        }

        number += max * 10_u64.pow((digits - step) as u32);
    }

    number
}

const DIGITS: u64 = 12;

fn main() {
    let banks: Vec<Vec<u64>> = aoc::Input::from_args()
        .lines()
        .iter()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as u64)
                .collect()
        })
        .collect();

    let mut sum = 0;

    for bank in banks {
        let highest = highest(bank, DIGITS);
        sum += highest;
    }

    dbg!(sum);
}
