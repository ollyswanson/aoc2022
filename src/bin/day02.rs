fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day02.txt");

    let score: i32 = input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let them = bytes[0] - b'A';
            let me = bytes[2] - b'X';

            let score = if them == me {
                3
            } else if (them + 1) % 3 == me {
                6
            } else {
                0
            };
            score + me as i32 + 1
        })
        .sum();

    println!("Part 1: {}", score);

    let score: i32 = input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let them = bytes[0] - b'A';
            let outcome = bytes[2] - b'X';

            let me = match outcome % 3 {
                0 => (them + 2) % 3,
                1 => them,
                2 => (them + 1) % 3,
                _ => unreachable!(),
            };

            outcome as i32 * 3 + me as i32 + 1
        })
        .sum();

    println!("Part 2: {}", score);

    Ok(())
}
