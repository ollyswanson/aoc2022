use itertools::Itertools;

pub fn run(input: &str) -> (u32, u32) {
    let rucksacks = process_input(input);

    (part_1(&rucksacks), part_2(&rucksacks))
}

fn process_input(input: &str) -> Vec<Rucksack> {
    input.lines().map(Rucksack::from_line).collect()
}

const fn ascii_letter_to_mask(byte: u8) -> usize {
    if byte.is_ascii_lowercase() {
        1 << (byte - b'a')
    } else if byte.is_ascii_uppercase() {
        1 << (byte - b'A' + 26)
    } else {
        0
    }
}

const fn mask_to_priority(mask: usize) -> u32 {
    64 - usize::leading_zeros(mask)
}

struct Rucksack {
    left: usize,
    right: usize,
}

impl Rucksack {
    fn from_line(line: &str) -> Self {
        let bytes = line.as_bytes();
        let (left, right) = bytes.split_at(bytes.len() / 2);

        let mask_fold = |acc: usize, &byte: &u8| acc | ascii_letter_to_mask(byte);

        let left = left.iter().fold(0, mask_fold);
        let right = right.iter().fold(0, mask_fold);

        Self { left, right }
    }

    fn intersect(&self) -> usize {
        self.left & self.right
    }

    fn union(&self) -> usize {
        self.left | self.right
    }
}

fn part_1(rucksacks: &[Rucksack]) -> u32 {
    rucksacks
        .iter()
        .map(Rucksack::intersect)
        .map(mask_to_priority)
        .sum()
}

fn part_2(rucksacks: &[Rucksack]) -> u32 {
    rucksacks
        .iter()
        .map(Rucksack::union)
        .chunks(3)
        .into_iter()
        .map(|chunk| chunk.fold(usize::MAX, |acc, mask| mask & acc))
        .map(mask_to_priority)
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
    fn ascii_letter_to_mask_works() {
        assert_eq!(1 << 0, ascii_letter_to_mask(b'a'));
        assert_eq!(1 << 25, ascii_letter_to_mask(b'z'));
        assert_eq!(1 << 26, ascii_letter_to_mask(b'A'));
        assert_eq!(1 << 51, ascii_letter_to_mask(b'Z'));
    }

    #[test]
    fn part_1_works() {
        let rucksacks = process_input(TEST_INPUT);

        assert_eq!(157, part_1(&rucksacks));
    }

    #[test]
    fn part_2_works() {
        let rucksacks = process_input(TEST_INPUT);

        assert_eq!(70, part_2(&rucksacks));
    }
}
