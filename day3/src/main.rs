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
        let highest = bank.iter().enumerate().max_by_key(|x| x.1).unwrap();
        let mut max = 0;

        for (i, joltage) in bank.iter().enumerate() {
            let total_joltage = if i < highest.0 {
                joltage * 10 + highest.1
            } else if i > highest.0 {
                highest.1 * 10 + joltage
            } else {
                *highest.1
            };

            if total_joltage > max {
                max = total_joltage
            }
        }

        sum += max;
    }

    dbg!(sum);
}
