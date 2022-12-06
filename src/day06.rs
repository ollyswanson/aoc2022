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

#[inline]
const fn normalize_letter(ascii_code: u8) -> usize {
    (ascii_code - b'a') as usize
}

struct UniqueLetters {
    counts: [u8; 26],
    uniq: usize,
}

impl UniqueLetters {
    fn new() -> Self {
        Self {
            counts: [0; 26],
            uniq: 0,
        }
    }

    /// Returns uniq after inserting a letter
    #[inline]
    fn insert(&mut self, ascii_code: u8) -> usize {
        let entry = &mut self.counts[normalize_letter(ascii_code)];
        *entry += 1;
        if *entry == 1 {
            self.uniq += 1;
        }

        self.uniq
    }

    /// Returns uniq after removing a letter
    #[inline]
    fn remove(&mut self, ascii_code: u8) -> usize {
        let entry = &mut self.counts[normalize_letter(ascii_code)];
        *entry -= 1;
        if *entry == 0 {
            self.uniq -= 1;
        }

        self.uniq
    }
}

/// K is the length of the unique run
fn unique_run_big_o_n<const K: usize>(input: &str) -> Option<usize> {
    let bytes = input.as_bytes();

    let mut uniq = UniqueLetters::new();

    for &b in bytes.iter().take(K) {
        uniq.insert(b);
    }

    if uniq.uniq == K {
        return Some(K);
    }

    for i in K..bytes.len() {
        let to_remove = bytes[i - K];
        let to_insert = bytes[i];

        uniq.remove(to_remove);

        if uniq.insert(to_insert) == K {
            return Some(i + 1);
        }
    }

    None
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
