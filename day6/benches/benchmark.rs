fn main() {
    divan::main();
}

#[divan::bench]
fn part1_bench(b: divan::Bencher) {
    let lines = aoc::Input::from_path(".inputs/official".into()).lines();

    b.bench_local(|| {
        divan::black_box(day6::part1(divan::black_box(&lines)));
    });
}

#[divan::bench]
fn part2_transpose_bench(b: divan::Bencher) {
    let lines = aoc::Input::from_path(".inputs/official".into()).lines();

    b.bench_local(|| {
        divan::black_box(day6::part2_transpose(divan::black_box(&lines)));
    });
}

#[divan::bench]
fn part2_by_column_bench(b: divan::Bencher) {
    let lines = aoc::Input::from_path(".inputs/official".into()).char_grid();

    b.bench_local(|| divan::black_box(day6::part2_by_column(divan::black_box(&lines))));
}
