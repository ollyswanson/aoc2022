fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day02.txt");

    let score: i32 = input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let them = bytes[0] - b'A';
            let me = bytes[2] - b'X';

            let outcome = ((me as i32 - them as i32).rem_euclid(3) + 1) % 3;

            outcome * 3 + me as i32 + 1
        })
        .sum();

    println!("Part 1: {}", score);

    let score: i32 = input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let them = bytes[0] - b'A';
            let outcome = bytes[2] - b'X';

            let me = (them + outcome + 2) % 3;

            outcome as i32 * 3 + me as i32 + 1
        })
        .sum();

    println!("Part 2: {}", score);

    Ok(())
}
