use aoc2022::day01::run;

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day01.txt");

    let (part_1, part_2) = run(input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);

    Ok(())
}
