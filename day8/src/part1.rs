use aoc::Vec3i;
use itertools::Itertools;

use crate::find_all_distances;

pub fn part1(vectors: &[Vec3i], top_size: usize) -> usize {
    let mut product = find_all_distances(vectors);
    assert_eq!(product.len(), (vectors.len().pow(2) - vectors.len()) / 2);

    product.sort_by_key(|&(_, _, dist)| dist);
    let top10 = product
        .iter()
        .take(top_size)
        .map(|&(a, b, _)| (a, b))
        .collect_vec();

    let mut circuits = crate::connect_boxes(top10);

    circuits.sort_by_key(|set| set.len());
    circuits.reverse();

    circuits.iter().take(3).map(|set| set.len()).product()
}
