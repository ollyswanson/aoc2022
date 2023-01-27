use aoc2022::run;

fn main() -> anyhow::Result<()> {
    let alt_input = include_str!("../../inputs/day21_alt.txt");
    use aoc2022::day21;

    let (p1, p2) = day21::run(alt_input)?;

    println!("p1 {}", p1);
    println!("p2 {}", p2);

    Ok(())
}
