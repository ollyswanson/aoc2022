pub fn run(input: &str) -> anyhow::Result<(usize, usize)> {
    Ok((
        unique_run::<4>(input).unwrap(),
        unique_run::<14>(input).unwrap(),
    ))
}

fn unique_run<const N: usize>(input: &str) -> Option<usize> {
    input
        .as_bytes()
        .windows(N)
        .enumerate()
        .find(|(_, w)| {
            w.iter()
                .fold(0u32, |mask, b| mask | (1 << (b - b'a')))
                .count_ones()
                == N as u32
        })
        .map(|(i, _)| i + N)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let inputs = [
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];

        for (input, expected) in inputs {
            assert_eq!(expected, unique_run::<4>(input).unwrap());
        }
    }

    #[test]
    fn part_2_works() {
        let inputs = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];

        for (input, expected) in inputs {
            assert_eq!(expected, unique_run::<14>(input).unwrap());
        }
    }
}
