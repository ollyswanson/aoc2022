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

benches!(day01, day02, day03, day04, day05);
criterion_main!(benches);
