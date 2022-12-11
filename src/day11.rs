use std::collections::VecDeque;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, u64};
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::{delimited, pair, preceded, terminated};
use nom::IResult;

use crate::utils::ws;

pub fn run(input: &str) -> anyhow::Result<(usize, usize)> {
    let mut monkeys = parse_monkeys(input)?;
    let mut monkeys_alt = monkeys.clone();

    let factor: usize = monkeys.iter().map(|monke| monke.test.div).product();

    Ok((
        play_game(&mut monkeys, 20, |x| x / 3),
        play_game(&mut monkeys_alt, 10000, |x| x % factor),
    ))
}

fn play_game(monkeys: &mut [Monkey], rounds: usize, adj: impl Fn(usize) -> usize) -> usize {
    let mut inspections = vec![0; monkeys.len()];

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].queue.pop_front() {
                inspections[i] += 1;
                let worry = adj(monkeys[i].op.evaluate(item));
                let to = monkeys[i].test.evaluate(worry);
                monkeys[to].queue.push_back(worry);
            }
        }
    }

    inspections.sort();
    inspections.pop().unwrap() * inspections.pop().unwrap()
}

#[derive(Debug, Clone)]
struct Monkey {
    op: Binary,
    test: Test,
    queue: VecDeque<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Binary {
    op: BinOp,
    left: Operand,
    right: Operand,
}

impl Binary {
    fn evaluate(&self, item: usize) -> usize {
        use BinOp::*;
        use Operand::*;

        let (l, r) = match (self.left, self.right) {
            (Const(l), Const(r)) => (l, r),
            (Const(l), Old) => (l, item),
            (Old, Const(r)) => (item, r),
            (Old, Old) => (item, item),
        };

        match self.op {
            Mul => l * r,
            Add => l + r,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BinOp {
    Mul,
    Add,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operand {
    Const(usize),
    Old,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Test {
    div: usize,
    conds: [usize; 2],
}

impl Test {
    /// Where to throw the item
    fn evaluate(&self, item: usize) -> usize {
        if item % self.div == 0 {
            self.conds[0]
        } else {
            self.conds[1]
        }
    }
}

fn parse_monkeys(input: &str) -> anyhow::Result<Vec<Monkey>> {
    let (_, monkeys) = separated_list0(line_ending, parse_monkey)(input)
        .map_err(|_| anyhow::anyhow!("Parse error"))?;

    Ok(monkeys)
}

// Monkey 7:
//   Starting items: 98, 89, 78, 73, 71
//   Operation: new = old + 4
//   Test: divisible by 2
//     If true: throw to monkey 3
//     If false: throw to monkey 2
fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    map(
        preceded(
            delimited(tag("Monkey "), u64, tag(":\n")),
            pair(
                terminated(parse_starting_items, line_ending),
                pair(
                    terminated(parse_operation, line_ending),
                    terminated(parse_test, line_ending),
                ),
            ),
        ),
        |(queue, (op, test))| Monkey { queue, op, test },
    )(input)
}

// Starting items: 98, 89, 78, 73, 71
fn parse_starting_items(input: &str) -> IResult<&str, VecDeque<usize>> {
    preceded(
        ws(tag("Starting items:")),
        map(
            separated_list0(tag(", "), map(u64, |n| n as usize)),
            VecDeque::from,
        ),
    )(input)
}

// Operation: new = old + 4
fn parse_operation(input: &str) -> IResult<&str, Binary> {
    map(
        preceded(
            ws(tag("Operation: new = ")),
            pair(
                pair(
                    parse_operand,
                    alt((
                        map(ws(tag("+")), |_| BinOp::Add),
                        map(ws(tag("*")), |_| BinOp::Mul),
                    )),
                ),
                parse_operand,
            ),
        ),
        |((left, op), right)| Binary { op, left, right },
    )(input)
}

fn parse_operand(input: &str) -> IResult<&str, Operand> {
    alt((
        map(tag("old"), |_| Operand::Old),
        map(u64, |n| Operand::Const(n as usize)),
    ))(input)
}

// Test: divisible by 2
//   If true: throw to monkey 3
//   If false: throw to monkey 2
fn parse_test(input: &str) -> IResult<&str, Test> {
    map(
        pair(
            preceded(ws(tag("Test: divisible by")), map(u64, |n| n as usize)),
            pair(
                preceded(
                    ws(tag("If true: throw to monkey")),
                    map(u64, |n| n as usize),
                ),
                preceded(
                    ws(tag("If false: throw to monkey")),
                    map(u64, |n| n as usize),
                ),
            ),
        ),
        |(div, (yes, no))| Test {
            div,
            conds: [yes, no],
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_MONKEYS: &str = include_str!("../inputs/day11_test.txt");

    #[test]
    fn part_1_works() {
        let mut monkeys = parse_monkeys(TEST_MONKEYS).unwrap();

        assert_eq!(10605, play_game(&mut monkeys, 20, |x| x / 3));
    }

    #[test]
    fn part_2_works() {
        let mut monkeys = parse_monkeys(TEST_MONKEYS).unwrap();
        let factor: usize = monkeys.iter().map(|m| m.test.div).product();

        assert_eq!(2713310158, play_game(&mut monkeys, 10000, |x| x % factor));
    }

    #[test]
    fn parse_starting_items_works() {
        let input = "Starting items: 98, 89, 78";
        let expected = VecDeque::from(vec![98, 89, 78]);

        assert_eq!(expected, parse_starting_items(input).unwrap().1);
    }

    #[test]
    fn parse_operation_works() {
        let input = "Operation: new = old + 4";
        let expected = Binary {
            op: BinOp::Add,
            left: Operand::Old,
            right: Operand::Const(4),
        };

        assert_eq!(expected, parse_operation(input).unwrap().1);
    }

    #[test]
    fn parse_test_works() {
        let input = "\
    Test: divisible by 2
      If true: throw to monkey 3
      If false: throw to monkey 2";

        let expected = Test {
            div: 2,
            conds: [3, 2],
        };

        assert_eq!(expected, parse_test(input).unwrap().1);
    }
}
