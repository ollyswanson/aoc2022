use nom::branch::alt;
use nom::character::complete::{char, i8, line_ending};
use nom::combinator::{all_consuming, eof, map, opt};
use nom::multi::many0;
use nom::sequence::{separated_pair, terminated};
use nom::IResult;

pub fn run(input: &str) -> anyhow::Result<(usize, usize)> {
    let diffs = parse_pairs(input)?;

    Ok((part_1(&diffs), part_2(&diffs)))
}

fn part_1(pairs: &[Diff]) -> usize {
    pairs
        .iter()
        .filter(|&&diff| diff == Diff::SubOrSuper)
        .count()
}

fn part_2(pairs: &[Diff]) -> usize {
    pairs.iter().filter(|&&diff| diff != Diff::Disjoint).count()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Diff {
    /// One section contains the other
    SubOrSuper,
    /// The assignments overlap without being a sub or superset
    Overlap,
    /// The assignments are disjoint
    Disjoint,
}

fn diff_assignments(a: (i8, i8), b: (i8, i8)) -> Diff {
    use std::cmp::Ordering::*;

    if b.0 > a.1 || a.0 > b.1 {
        return Diff::Disjoint;
    }

    match (a.0.cmp(&b.0), a.1.cmp(&b.1)) {
        (Greater, Greater) => Diff::Overlap,
        (Less, Less) => Diff::Overlap,
        (Greater, Less) => Diff::SubOrSuper,
        (Less, Greater) => Diff::SubOrSuper,
        (Equal, _) => Diff::SubOrSuper,
        (_, Equal) => Diff::SubOrSuper,
    }
}

fn parse_assignment(input: &str) -> IResult<&str, (i8, i8)> {
    separated_pair(i8, char('-'), i8)(input)
}

fn parse_pair(input: &str) -> IResult<&str, Diff> {
    let pair = separated_pair(parse_assignment, char(','), parse_assignment);

    map(pair, |(a, b)| diff_assignments(a, b))(input)
}

fn parse_pairs(input: &str) -> anyhow::Result<Vec<Diff>> {
    let (_, diffs) = all_consuming(many0(terminated(parse_pair, opt(alt((line_ending, eof))))))(
        input,
    )
    .map_err(|e| -> anyhow::Error {
        dbg!(e);
        anyhow::anyhow!("Parse error!")
    })?;

    Ok(diffs)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn parse_assignment_works() {
        let input = "2-4";
        let expected = (2, 4);

        assert_eq!(expected, parse_assignment(input).unwrap().1);
    }

    #[test]
    fn parse_pair_works() {
        let input = "2-4,6-8";

        assert_eq!(Diff::Disjoint, parse_pair(input).unwrap().1);
    }

    #[test]
    fn part_1_works() {
        let diffs = parse_pairs(TEST_INPUT).unwrap();

        assert_eq!(2, part_1(&diffs));
    }

    #[test]
    fn part_2_works() {
        let diffs = parse_pairs(TEST_INPUT).unwrap();

        assert_eq!(4, part_2(&diffs));
    }
}
