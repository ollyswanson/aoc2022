pub fn run(input: &str) -> (usize, usize) {
    let stats = process_input(input);

    (part_1(&stats), part_2(&stats))
}

fn process_input(input: &str) -> Vec<BagStats> {
    input.lines().map(BagStats::process_line).collect()
}

const fn letter_to_priority(byte: u8) -> u8 {
    if byte.is_ascii_lowercase() {
        byte - b'a' + 1
    } else if byte.is_ascii_uppercase() {
        byte - b'A' + 27
    } else {
        0
    }
}

// Use 64 for the sake of alignment
#[derive(Debug)]
struct BagStats([Stat; 64]);

impl BagStats {
    // Bins fall into the range 1..=52 as specified by `letter_to_priority`
    const PRIORITY_UPPER: usize = 52;

    fn process_line(line: &str) -> Self {
        let mut stats = Self::default();
        let bytes = line.as_bytes();
        let (left, right) = bytes.split_at(bytes.len() / 2);

        for &byte in left {
            let priority = letter_to_priority(byte) as usize;
            let stat = &mut stats.0[priority];
            stat.occurrences += 1;
            stat.in_left = true;
        }

        for &byte in right {
            let priority = letter_to_priority(byte) as usize;
            let stat = &mut stats.0[priority];
            stat.occurrences += 1;
            stat.in_right = true;
        }

        stats
    }
}

impl Default for BagStats {
    fn default() -> Self {
        Self([Stat::default(); 64])
    }
}

#[derive(Default, Clone, Copy, Debug)]
struct Stat {
    occurrences: u16,
    in_left: bool,
    in_right: bool,
}

fn part_1(stats: &[BagStats]) -> usize {
    let priority_in_left_and_right = |stats: &BagStats| {
        stats
            .0
            .iter()
            .take(BagStats::PRIORITY_UPPER)
            .position(|stat| stat.in_left && stat.in_right)
    };

    stats.iter().filter_map(priority_in_left_and_right).sum()
}

fn part_2(stats: &[BagStats]) -> usize {
    assert!(stats.len() % 3 == 0);

    (0..stats.len())
        .step_by(3)
        .map(|i| (&stats[i], &stats[i + 1], &stats[i + 2]))
        .filter_map(|chunk| {
            (0..BagStats::PRIORITY_UPPER + 1).position(|i| {
                chunk.0 .0[i].occurrences > 0
                    && chunk.1 .0[i].occurrences > 0
                    && chunk.2 .0[i].occurrences > 0
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part_1_works() {
        let stats = process_input(TEST_INPUT);

        assert_eq!(157, part_1(&stats));
    }

    #[test]
    fn part_2_works() {
        let stats = process_input(TEST_INPUT);

        assert_eq!(70, part_2(&stats));
    }
}
