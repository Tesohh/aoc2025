use std::{collections::HashSet, hash::Hash};

use aoc::Vec3i;

pub mod part1;
pub mod part2;

pub fn find_all_distances(vectors: &[Vec3i]) -> Vec<(Vec3i, Vec3i, isize)> {
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

    product
}

pub fn merge_sets<T>(all_sets: &mut Vec<HashSet<T>>, mergee_index: usize, target_index: usize)
where
    T: Eq + Hash,
{
    let set_b = all_sets.swap_remove(target_index);
    all_sets.get_mut(mergee_index).unwrap().extend(set_b);
}

pub fn print_product(product: &[(Vec3i, Vec3i, isize)]) {
    for (a, b, dist) in product {
        println!("{} --> {} ({})", a, b, dist)
    }
}

#[derive(Debug)]
enum Top10Case {
    Push(usize, Vec3i), // a is in set S. push b into S OR b is in set S. push a into S
    Both,               // a,b are in set S. don't do anything
    New,                // a,b are in no set. create a new set with both
    Merge(usize, usize), // a,b are in different sets. merge the sets
}

pub fn connect_boxes(links: &Vec<(Vec3i, Vec3i)>) -> (Vec<HashSet<Vec3i>>, Vec<(Vec3i, Vec3i)>) {
    let mut circuits: Vec<HashSet<Vec3i>> = vec![];
    let mut connections: Vec<(Vec3i, Vec3i)> = vec![];

    for &(a, b) in links {
        let mut case = Top10Case::New;

        for (i, set) in circuits.iter().enumerate() {
            if set.contains(&a) && set.contains(&b) {
                case = Top10Case::Both;
                break; // should be fine
            } else if let Top10Case::Push(j, _) = case
                && (set.contains(&a) || set.contains(&b))
            {
                case = Top10Case::Merge(j, i);
            } else if set.contains(&a) {
                case = Top10Case::Push(i, b);
            } else if set.contains(&b) {
                case = Top10Case::Push(i, a);
            }
        }

        match case {
            Top10Case::Push(i, vec3) => {
                circuits.get_mut(i).unwrap().insert(vec3);
                connections.push((a, b));
            }
            Top10Case::Both => {}
            Top10Case::New => {
                let mut set = HashSet::new();
                set.insert(a);
                set.insert(b);
                circuits.push(set);
                connections.push((a, b));
            }
            Top10Case::Merge(i, j) => {
                merge_sets(&mut circuits, i, j);
                connections.push((a, b));
            }
        }
    }

    (circuits, connections)
}
