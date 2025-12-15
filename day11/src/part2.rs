use std::collections::HashMap;

use crate::Server;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Visitor<'a> {
    server: &'a Server,
    history: Vec<&'a Server>,
}

pub fn part2(servers: &HashMap<Server, Vec<Server>>) -> usize {
    let mut paths = 0;

    let start = Visitor {
        server: &Server::new("svr"),
        history: vec![],
    };
    let mut visitors: HashMap<Visitor, usize> = HashMap::from([(start, 1)]);

    loop {
        let mut new_visitors: HashMap<Visitor, usize> = HashMap::new();
        for (visitor, times) in &visitors {
            if visitor.server == &Server::new("out") {
                if visitor.history.contains(&&Server::new("fft"))
                    && visitor.history.contains(&&Server::new("dac"))
                {
                    paths += times;
                }
                continue;
            }

            let outputs = servers.get(visitor.server).unwrap().iter().map(|server| {
                let mut new_visitor = Visitor {
                    server,
                    history: vec![],
                };
                new_visitor.history.extend(&visitor.history);
                new_visitor.history.push(visitor.server);
                new_visitor
            });

            for output in outputs {
                new_visitors
                    .entry(output)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            }
        }

        if new_visitors.is_empty() {
            break;
        }

        visitors = new_visitors;
        dbg!(&visitors.len());
    }

    paths
}
