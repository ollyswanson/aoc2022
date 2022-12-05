#[macro_export]
macro_rules! run {
    ($day:tt) => {{
        use aoc2022::$day::run;
        let input = include_str!(std::concat!("../../inputs/", std::stringify!($day), ".txt"));
        let (part_1, part_2) = run(input)?;

        println!("Part 1: {}", part_1);
        println!("Part 2: {}", part_2);

        Ok(())
    }};
}
