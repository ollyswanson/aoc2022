use itertools::Itertools;

pub fn run(input: &str) -> (u32, u32) {
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

    (
        *calories.last().unwrap(),
        calories.iter().rev().take(3).sum::<u32>(),
    )
}
