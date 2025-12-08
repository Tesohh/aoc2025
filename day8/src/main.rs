use aoc::Vec3i;
use day8::part1;

fn main() {
    let vectors: Vec<Vec3i> = aoc::Input::from_args()
        .lines()
        .iter()
        .map(|line| {
            let mut splits = line.split(",");
            (
                splits.next().unwrap().parse().unwrap(),
                splits.next().unwrap().parse().unwrap(),
                splits.next().unwrap().parse().unwrap(),
            )
                .into()
        })
        .collect();

    let top_size = match vectors.len() {
        20 => 10,
        1000 => 1000,
        _ => 10,
    };
    println!("part1: {}", part1::part1(&vectors, top_size));
}
