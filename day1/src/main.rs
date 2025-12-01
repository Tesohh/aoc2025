fn main() {
    let lines = aoc::Input::from_args().lines();
    let instructions: Vec<(&str, isize)> = lines
        .iter()
        .map(|s| s.split_at(1))
        .map(|(dir, value)| (dir, value.parse().expect("got unparsable integer")))
        .collect::<_>();

    let mut dial_part1 = 50isize;
    let mut dial_part2 = 50isize;
    let mut password_part1 = 0;
    let mut password_part2 = 0;
    for (direction, value) in instructions {
        match direction {
            "L" => dial_part1 -= value,
            "R" => dial_part1 += value,
            _ => panic!("invalid direction"),
        };
        dial_part1 = dial_part1.rem_euclid(100);
        if dial_part1 == 0 {
            password_part1 += 1;
        }

        // Ugly Ugly Ugly
        for _ in 0..value {
            match direction {
                "L" => dial_part2 -= 1,
                "R" => dial_part2 += 1,
                _ => panic!("invalid direction"),
            };
            dial_part2 = dial_part2.rem_euclid(100);
            if dial_part2 == 0 {
                password_part2 += 1;
            }
        }
    }

    dbg!(password_part1);
    dbg!(password_part2);
}
