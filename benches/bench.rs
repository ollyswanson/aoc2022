use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn day01(c: &mut Criterion) {
    use aoc2022::day01::run;
    let input = include_str!("../inputs/day01.txt");

    c.bench_function("day01", |b| b.iter(|| run(black_box(input))));
}

pub fn day02(c: &mut Criterion) {
    use aoc2022::day02::run;
    let input = include_str!("../inputs/day02.txt");

    c.bench_function("day02", |b| b.iter(|| run(black_box(input))));
}

pub fn day03(c: &mut Criterion) {
    use aoc2022::day03::run;
    let input = include_str!("../inputs/day03.txt");

    c.bench_function("day03", |b| b.iter(|| run(black_box(input))));
}

pub fn day04(c: &mut Criterion) {
    use aoc2022::day04::run;
    let input = include_str!("../inputs/day04.txt");

    c.bench_function("day04", |b| b.iter(|| run(black_box(input))));
}

criterion_group!(benches, day01, day02, day03, day04);
criterion_main!(benches);
