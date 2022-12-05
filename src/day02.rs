// Could combine part 1 and 2 into one fold, but would make readability even worse
pub fn run(input: &str) -> anyhow::Result<(i32, i32)> {
    let score_1: i32 = input
        .as_bytes()
        .chunks(4)
        .map(|chunk| {
            let them = (chunk[0] - b'A') as i32;
            let me = (chunk[2] - b'X') as i32;

            let outcome = ((me - them).rem_euclid(3) + 1) % 3;

            outcome * 3 + me + 1
        })
        .sum();

    let score_2: i32 = input
        .as_bytes()
        .chunks(4)
        .map(|chunk| {
            let them = (chunk[0] - b'A') as i32;
            let outcome = (chunk[2] - b'X') as i32;

            let me = (them + outcome + 2) % 3;

            outcome * 3 + me + 1
        })
        .sum();

    Ok((score_1, score_2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "\
A Y
B X
C Z";

        let (part_1, part_2) = run(input).unwrap();

        assert_eq!(15, part_1);
        assert_eq!(12, part_2);
    }
}
