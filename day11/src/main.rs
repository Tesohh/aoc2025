use std::collections::HashMap;

use day11::{Server, part1, part2, str_to_char3};
use itertools::Itertools;

fn main() {
    let mut servers: HashMap<Server, Vec<Server>> = HashMap::new();

    aoc::Input::from_args()
        .lines()
        .iter()
        .filter_map(|line| line.split_once(": "))
        .map(|(key, connections)| {
            (
                Server(str_to_char3(key).unwrap()),
                connections
                    .split(" ")
                    .map(|s| Server(str_to_char3(s).unwrap()))
                    .collect_vec(),
            )
        })
        .for_each(|(key, connections)| {
            servers.insert(key, connections);
        });

    println!("part1: {}", part1::part1(&servers));
    println!("part2: {}", part2::part2(&servers));
}
