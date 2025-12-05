use owo_colors::OwoColorize;

fn main() {
    let parts = aoc::Input::from_args().parts("\n\n");
    let ranges: Vec<(usize, usize)> = parts[0]
        .clone()
        .lines()
        .iter()
        .filter_map(|s| s.split_once("-"))
        .map(|(raw_start, raw_end)| (raw_start.parse().unwrap(), raw_end.parse().unwrap()))
        .collect();

    let foods: Vec<usize> = parts[1]
        .clone()
        .lines()
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut counter = 0;
    for id in foods {
        let mut ok = false;
        for &(start, end) in &ranges {
            if id >= start && id <= end {
                ok = true
            }
        }

        if ok {
            counter += 1
        }
    }

    dbg!(counter);
    // ranges.sort_by_key(|(start, _)| *start);

    let mut corrected_ranges: Vec<(usize, usize)> = vec![];
    for &(my_start, my_end) in &ranges {
        let mut corrected = (my_start, my_end);

        for &(other_start, other_end) in &corrected_ranges {
            if my_start >= other_start && my_start <= other_end {
                println!(
                    "correct {} ({}) -> ({})",
                    "start".red(),
                    corrected.0,
                    other_end + 1
                );
                corrected.0 = other_end + 1
            }

            if my_end >= other_start && my_end <= other_end {
                println!(
                    "correct {}   ({}) -> ({})",
                    "end".blue(),
                    corrected.1,
                    other_start - 1
                );

                if other_start == 0 {
                    panic!("NEGATIVE")
                }
                corrected.1 = other_start - 1
            }
        }

        if corrected != (my_start, my_end) {
            println!(
                "corrected old({}, {}) -> new({}, {})",
                my_start, my_end, corrected.0, corrected.1
            );
        }
        if corrected.0 > corrected.1 {
            continue;
        }

        corrected_ranges.push(corrected);
    }

    let mut sum = 0;
    for &(start, end) in &corrected_ranges {
        print!("{start}-{end}");
        if end < start {
            print!(" (Oops)");
            sum -= start - end - 1
        } else {
            sum += end - start + 1
        }
        println!();
    }

    dbg!(sum);
}
