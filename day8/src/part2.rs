use aoc::Vec3i;
use itertools::Itertools as _;

use crate::find_all_distances;

pub fn part2(vectors: &[Vec3i]) -> isize {
    let mut product = find_all_distances(vectors);
    assert_eq!(product.len(), (vectors.len().pow(2) - vectors.len()) / 2);

    product.sort_by_key(|&(_, _, dist)| dist);
    let best_connections = product.iter().map(|&(a, b, _)| (a, b)).collect_vec();

    let (_, connections) = crate::connect_boxes(&best_connections);

    let last = connections.last().unwrap();
    last.0.x * last.1.x
}
