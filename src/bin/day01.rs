use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day01.txt");

    let calories: Vec<u32> = input
        .split("\n\n")
        .map(|section| {
            section
                .lines()
                .filter_map(|line| line.parse::<u32>().ok())
                .sum()
        })
        .sorted()
        .collect();

    println!("Part 1: {}", calories.last().unwrap());
    println!("Part 2: {}", calories.iter().rev().take(3).sum::<u32>());

    Ok(())
}
