use itertools::Itertools;

fn part1(ranges: &[(usize, usize)], foods: &[usize]) -> usize {
    foods
        .iter()
        .filter(|&&id| ranges.iter().any(|&(start, end)| id >= start && id <= end))
        .count()
}

fn range_includes(range: &(usize, usize), value: usize) -> bool {
    range.0 <= value && value <= range.1
}

fn sum_ranges(ranges: &[(usize, usize)]) -> usize {
    ranges.iter().map(|&(start, end)| end - start + 1).sum()
}

fn part2(ranges: &[(usize, usize)]) -> usize {
    let ranges = ranges.iter().sorted_by_key(|&(start, _)| start);
    let mut merged_ranges: Vec<(usize, usize)> = vec![];

    for &my_range in ranges {
        let Some(other_range) = merged_ranges.last_mut() else {
            merged_ranges.push(my_range);
            continue;
        };

        if range_includes(other_range, my_range.0) {
            if my_range.1 > other_range.1 {
                other_range.1 = my_range.1
            }
        } else {
            merged_ranges.push(my_range);
        }
    }

    for &my_range in &merged_ranges {
        println!("{}-{}", my_range.0, my_range.1)
    }

    sum_ranges(&merged_ranges)
}

fn main() {
    let parts = aoc::Input::from_args().parts("\n\n");
    let ranges: Vec<(usize, usize)> = parts[0]
        .lines()
        .iter()
        .filter_map(|s| s.split_once("-"))
        .map(|(raw_start, raw_end)| (raw_start.parse().unwrap(), raw_end.parse().unwrap()))
        .collect();

    let foods: Vec<usize> = parts[1]
        .lines()
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("part1: {}", part1(&ranges, &foods));
    println!("part2: {}", part2(&ranges));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_includes() {
        assert!(range_includes(&(1, 3), 2));
        assert!(range_includes(&(4, 4), 4));
        assert!(range_includes(&(10, 20), 12));
        assert!(!range_includes(&(10, 20), 9));
    }
}
