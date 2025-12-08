use std::collections::HashSet;

use aoc::Vec3i;
use itertools::Itertools;

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
        // Cases:
        // LHS:    a is in set S. push b into S
        // RHS:    b is in set S. push a into S
        // BOTH:   a,b are in set S. don't do anything
        // NEW:    a,b are in no set. create a new set with both
        // MERGE:  a,b are in different sets. merge the sets

        // let mut index: Option<usize> = None;
        // let mut target: Option<Vec3i> = None;
        // let mut push = true;
        //
        // for (i, set) in circuits.iter().enumerate() {
        //     if set.contains(&a) && set.contains(&b) {
        //         push = false;
        //         break;
        //     } else if set.contains(&a) {
        //         index = Some(i);
        //         target = Some(b);
        //         break;
        //     } else if set.contains(&b) {
        //         index = Some(i);
        //         target = Some(a);
        //         break;
        //     }
        // }

        // if let Some(index) = index {
        //     circuits.get_mut(index).unwrap().insert(target.unwrap());
        // } else if push {
        //     let mut set = HashSet::new();
        //     set.insert(a);
        //     set.insert(b);
        //     circuits.push(set);
        // } else if !push {
        //     println!("Not pushing pair {} {}", a, b)
        // }
    }
    // maybe we need to check the "dotted lines" (when both a and b are already in the circuit)

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
