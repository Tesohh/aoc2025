use std::collections::HashMap;

use day11::{Server, str_to_char3};
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

    let mut paths = 0;

    let mut visitors: Vec<&Server> = vec![];
    visitors.extend(servers.get(&Server::new("you")).unwrap());

    loop {
        let mut new_visitors = vec![];
        for visitor in &visitors {
            if **visitor == Server::new("out") {
                paths += 1;
                continue;
            }

            let outputs = servers.get(visitor).unwrap();
            new_visitors.extend(outputs);
        }

        if new_visitors.is_empty() {
            break;
        }

        visitors = new_visitors;
        dbg!(&visitors);
    }

    dbg!(paths);
}
