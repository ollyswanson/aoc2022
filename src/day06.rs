pub fn run(input: &str) -> anyhow::Result<(usize, usize)> {
    Ok((
        unique_run_big_o_n::<4>(input).unwrap(),
        unique_run_big_o_n::<14>(input).unwrap(),
    ))
}

#[allow(dead_code)]
/// K is the length of the unique run
fn unique_run_big_o_nk<const K: usize>(input: &str) -> Option<usize> {
    input
        .as_bytes()
        .windows(K)
        .enumerate()
        .find(|(_, w)| {
            w.iter()
                .fold(0u32, |mask, b| mask | (1 << (b - b'a')))
                .count_ones()
                == K as u32
        })
        .map(|(i, _)| i + K)
}

/// K is the length of the unique run
fn unique_run_big_o_n<const K: usize>(input: &str) -> Option<usize> {
    let bytes = input.as_bytes();

    let mut letter_counts = [0u8; 26];
    let mut unique_letters = 0usize;

    for b in bytes.iter().take(K).copied().map(normalize_letter) {
        letter_counts[b] += 1;

        if letter_counts[b] == 1 {
            unique_letters += 1;
        }
    }

    if unique_letters == K {
        return Some(K);
    }

    for i in K..bytes.len() {
        let removed_letter = normalize_letter(bytes[i - K]);
        let new_letter = normalize_letter(bytes[i]);

        letter_counts[removed_letter] -= 1;
        if letter_counts[removed_letter] == 0 {
            unique_letters -= 1;
        }

        letter_counts[new_letter] += 1;
        if letter_counts[new_letter] == 1 {
            unique_letters += 1;
        }

        if unique_letters == K {
            return Some(i + 1);
        }
    }

    None
}

#[inline]
const fn normalize_letter(b: u8) -> usize {
    (b - b'a') as usize
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
            assert_eq!(expected, unique_run_big_o_n::<4>(input).unwrap());
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
            assert_eq!(expected, unique_run_big_o_n::<14>(input).unwrap());
        }
    }
}
