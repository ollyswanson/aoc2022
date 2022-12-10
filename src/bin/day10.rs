use aoc2022::day10::run_with;

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day10.txt");

    let mut out = std::io::stdout().lock();
    run_with(input, &mut out)
}
