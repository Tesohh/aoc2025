fn main() {
    let input = aoc::Input::from_args();
    let lines = input.lines();
    let grid = input.char_grid();
    println!("part1: {}", day6::part1(&lines));
    println!("part2 (transpose): {}", day6::part2_transpose(&lines));
    println!("part2   (columns): {}", day6::part2_by_column(&grid));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_splitter() {
        assert_eq!(
            day6::split_string_on_multiple_pos("gu ten morgen", &[3, 7]),
            ["gu", "ten", "morgen"]
        );
    }

    #[test]
    fn test_ceph_numbers() {
        assert_eq!(
            day6::to_ceph_numbers(&["64 ".into(), "23 ".into(), "314".into()]),
            [623, 431, 4]
        );

        assert_eq!(
            day6::to_ceph_numbers(&[" 51".into(), "387".into(), "215".into()]),
            [32, 581, 175]
        );

        assert_eq!(
            day6::to_ceph_numbers(&["328".into(), "64 ".into(), "98 ".into()]),
            [369, 248, 8]
        );
    }
}
