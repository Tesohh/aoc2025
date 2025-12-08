use std::collections::HashSet;

use aoc::Vec3i;
use itertools::Itertools;

#[derive(Debug)]
enum Top10Case {
    Push(usize, Vec3i), // a is in set S. push b into S OR b is in set S. push a into S
    Both,               // a,b are in set S. don't do anything
    New,                // a,b are in no set. create a new set with both
    Merge(usize, usize), // a,b are in different sets. merge the sets
}

pub fn part1(vectors: &[Vec3i]) -> usize {
    // vectors.sort_by(|a, b| a.squared_distance(*b).cmp(&b.squared_distance(*a)));
    // dbg!(vectors);

    let mut product: Vec<(Vec3i, Vec3i, isize)> = vec![];
    for i in 0..vectors.len() {
        for j in 0..vectors.len() {
            if j > i {
                let a = vectors[i];
                let b = vectors[j];
                product.push((a, b, a.squared_distance(b)));
            }
        }
    }

    assert_eq!(product.len(), (vectors.len().pow(2) - vectors.len()) / 2);

    product.sort_by_key(|&(_, _, dist)| dist);
    let top10 = product.iter().take(10).collect_vec();

    let mut circuits: Vec<HashSet<Vec3i>> = vec![];
    for &(a, b, _) in top10 {
        let mut case = Top10Case::New;

        for (i, set) in circuits.iter().enumerate() {
            if set.contains(&a) && set.contains(&b) {
                case = Top10Case::Both;
                break; // should be fine
            } else if let Top10Case::Push(j, _) = case
                && (set.contains(&a) || set.contains(&b))
            {
                case = Top10Case::Merge(i, j);
            } else if set.contains(&a) {
                case = Top10Case::Push(i, b);
            } else if set.contains(&b) {
                case = Top10Case::Push(i, a);
            }
        }

        println!("{:?} ({} {})", &case, &a, &b);

        match case {
            Top10Case::Push(i, vec3) => {
                circuits.get_mut(i).unwrap().insert(vec3);
            }
            Top10Case::Both => {}
            Top10Case::New => {
                let mut set = HashSet::new();
                set.insert(a);
                set.insert(b);
                circuits.push(set);
            }
            Top10Case::Merge(i, j) => {
                let set_b = circuits.swap_remove(j);
                circuits.get_mut(i).unwrap().extend(set_b);
            }
        }
    }

    circuits.sort_by_key(|set| set.len());
    circuits.reverse();
    for set in &circuits {
        for elt in set {
            print!("{} ", elt)
        }
        println!()
    }

    circuits.iter().take(3).map(|set| set.len()).product()
}
