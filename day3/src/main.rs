fn highest(bank: &[u64], digits: usize) -> u64 {
    let mut start = 0usize;
    let mut number = 0;

    for step in 1..=digits {
        let end = bank.len() - digits + step;

        let (max_index, &max) = bank[start..end]
            .iter()
            .enumerate()
            .max_by_key(|(_, joltage)| **joltage)
            .unwrap();

        start += max_index + 1;
        number += max * 10_u64.pow((digits - step) as u32);
    }

    number
}

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

    let mut sum1 = 0;
    let mut sum2 = 0;
    let mut sum3 = 0;

    for bank in banks {
        sum1 += highest(&bank, 2);
        sum2 += highest(&bank, 12);
        sum3 += highest(&bank, 18);
    }

    dbg!(sum1);
    dbg!(sum2);
    dbg!(sum3);
}
