use std::collections::HashMap;

fn digits(mut n: usize) -> Vec<usize> {
    let mut v: Vec<usize> = vec![];
    while n > 0 {
        v.push(n % 10);
        n /= 10;
    }
    v.reverse();
    v
}

// returns all factors, excluding `n` itself.
fn factors(n: usize) -> Vec<usize> {
    if n == 0 {
        panic!("n of factors(n) cannot be 0")
    }

    (1..n).filter(|q| n.is_multiple_of(*q)).collect()
}

fn part1(ranges: Vec<(usize, usize)>) -> usize {
    let mut sum = 0;

    for (start, end) in ranges {
        for n in start..=end {
            let digits = digits(n);

            if digits.len().is_multiple_of(2) {
                let chunks: Vec<_> = digits.chunks(digits.len() / 2).collect();
                let first = chunks.first().unwrap();
                let invalid = chunks.iter().all(|c| c == first);
                if invalid {
                    sum += n;
                }
            }
        }
    }

    sum
}
fn part2(ranges: Vec<(usize, usize)>) -> usize {
    let mut factors_cache: HashMap<usize, Vec<usize>> = HashMap::new();

    let mut sum = 0;

    for (start, end) in ranges {
        for n in start..=end {
            let digits = digits(n);
            let factors = factors_cache
                .entry(digits.len())
                .or_insert_with(|| factors(digits.len()));

            let invalid = factors.iter().any(|q| {
                let chunks: Vec<_> = digits.chunks(*q).collect();
                let first = chunks.first().unwrap();
                chunks.iter().all(|c| c == first)
            });

            if invalid {
                sum += n;
            }
        }
    }

    /*
     *Since the young Elf was just doing silly patterns, you can find the invalid IDs by looking for any ID which is made only of some sequence of digits repeated twice. So, 55 (5 twice), 6464 (64 twice), and 123123 (123 twice) would all be invalid IDs.
     */
    // NOTE: we only need to check if a repetition happens TWICE. not q times.
    // NOTE: what i did here probably solves part2

    sum
}

fn main() {
    let ranges: Vec<(usize, usize)> = aoc::Input::from_args()
        .splits(",")
        .iter()
        .map(|s| s.trim())
        .filter_map(|s| s.split_once("-"))
        .map(|(start, end)| (start.parse().unwrap(), end.parse().unwrap()))
        .collect();

    dbg!(part1(ranges.clone()));
    dbg!(part2(ranges));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits() {
        assert_eq!(digits(223), vec![2, 2, 3]);
        assert_eq!(digits(1), vec![1]);
        assert_eq!(digits(12020), vec![1, 2, 0, 2, 0]);
        assert_eq!(digits(0), vec![]);
    }

    #[test]
    fn test_factors() {
        assert_eq!(factors(6), vec![1, 2, 3]);
        assert_eq!(factors(9), vec![1, 3]);
        assert_eq!(factors(8), vec![1, 2, 4]);
    }
}
