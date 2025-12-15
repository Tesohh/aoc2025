use std::collections::HashMap;

use crate::Server;

pub fn part1(servers: &HashMap<Server, Vec<Server>>) -> usize {
    let mut paths = 0;

    let mut visitors: Vec<&Server> = vec![];
    visitors.extend(servers.get(&Server::new("you")).expect("you not found!"));

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
        dbg!(&visitors.len());
    }

    paths
}
