use itertools::Itertools;

pub fn run(input: &str) -> anyhow::Result<(u32, u32)> {
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

    Ok((
        *calories.last().unwrap(),
        calories.iter().rev().take(3).sum::<u32>(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        let (part_1, part_2) = run(input).unwrap();
        println!("{}", input);

        assert_eq!(24000, part_1);
        assert_eq!(45000, part_2);
    }
}
