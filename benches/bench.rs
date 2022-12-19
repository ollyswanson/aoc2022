use criterion::{black_box, criterion_group, criterion_main, Criterion};

macro_rules! benches {
    ($($day:tt),*) => {
        $(pub fn $day(c: &mut Criterion) {
            use aoc2022::$day::run;

            let input = include_str!(std::concat!("../inputs/", std::stringify!($day), ".txt"));

            c.bench_function(std::stringify!($day), |b| b.iter(|| run(black_box(input))));
        })*

        criterion_group!(benches, $($day),*);
    };
}

benches!(
    day01, day02, day03, day04, day05, day06, day07, day08, day08_par, day09, day10, day11, day12,
    day13, day14, day15, day16, day17, day18, day19
);
criterion_main!(benches);
