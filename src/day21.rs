use std::collections::VecDeque;
use std::ops;

use anyhow::{anyhow, Context};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha0, i64, line_ending};
use nom::combinator::{map, value};
use nom::multi::separated_list0;
use nom::sequence::{terminated, tuple};
use nom::IResult;

use crate::utils::ws;

pub fn run(input: &str) -> anyhow::Result<(i64, i64)> {
    let mut tree = MonkeyTree::build_tree(input)?;

    let root_idx = *tree.names.get("root").unwrap();
    let humn_idx = *tree.names.get("humn").unwrap();

    let path = find_path(root_idx, humn_idx, &tree);

    let part_1 = get_value(root_idx, &tree);
    let mut cached_values = vec![0; tree.monkeys.len()];
    evaluate_tree(root_idx, &mut cached_values, &tree);

    let balanced = balance_values(root_idx, humn_idx, &tree, &cached_values);
    dbg!(balanced);

    let mut part_2 = 0;

    if let Yell::Maths(m) = tree.monkeys[root_idx] {
        let (_, b) = (get_value(m.monkeys.0, &tree), get_value(m.monkeys.1, &tree));

        part_2 = search(b, m.monkeys.0, humn_idx, &mut tree);

        tree.monkeys[humn_idx] = Yell::Value(part_2);
        let (a, b) = (get_value(m.monkeys.0, &tree), get_value(m.monkeys.1, &tree));
        dbg!(a, b);
    } else {
        anyhow::bail!("Error");
    };

    Ok((part_1, part_2))
}

fn evaluate_tree(idx: MonkeyIdx, values: &mut [i64], tree: &MonkeyTree) -> i64 {
    let value = match tree.monkeys[idx] {
        Yell::Value(v) => v,
        Yell::Maths(Maths { monkeys, op }) => {
            let l = evaluate_tree(monkeys.0, values, tree);
            let r = evaluate_tree(monkeys.1, values, tree);

            op.evaluate(l, r)
        }
    };

    values[idx] = value;
    value
}

fn find_path(start: MonkeyIdx, end: MonkeyIdx, tree: &MonkeyTree) -> Vec<MonkeyIdx> {
    let mut path: Vec<MonkeyIdx> = Vec::new();
    path.push(start);

    while let Some(top) = path.last().copied() {
        if top == end {
            return path;
        }

        if let Yell::Maths(m) = tree.monkeys[top] {
            path.push(m.monkeys.0);
        } else {
            while let Some(popped) = path.pop() {
                if let Some(&top) = path.last() {
                    if let Yell::Maths(m) = tree.monkeys[top] {
                        if popped == m.monkeys.0 {
                            path.push(m.monkeys.1);
                            break;
                        }
                    }
                }
            }
        }
    }

    path
}

fn balance_values(
    root: MonkeyIdx,
    humn: MonkeyIdx,
    tree: &MonkeyTree,
    cached_value: &[i64],
) -> i64 {
    let Yell::Maths(m) = tree.monkeys[root] else {
        panic!("Invalid root");
    };

    let path = find_path(root, humn, tree);

    let mut target = if m.monkeys.0 == path[1] {
        cached_value[m.monkeys.1]
    } else {
        cached_value[m.monkeys.0]
    };

    for (&parent, &child) in path[1..].iter().tuple_windows() {
        let Yell::Maths(m) = tree.monkeys[parent] else {
            panic!("Invalid path");
        };

        target = if m.monkeys.0 == child {
            m.op.inverse().evaluate(target, cached_value[m.monkeys.1])
        } else {
            match m.op {
                Op::Div => cached_value[m.monkeys.0] / target,
                Op::Sub => cached_value[m.monkeys.0] - target,
                op => op.inverse().evaluate(target, cached_value[m.monkeys.0]),
            }
        };
    }

    target
}

fn get_value(idx: MonkeyIdx, graph: &MonkeyTree) -> i64 {
    match graph.monkeys[idx] {
        Yell::Value(v) => v,
        Yell::Maths(Maths { monkeys, op }) => {
            let a = get_value(monkeys.0, graph);
            let b = get_value(monkeys.1, graph);

            match op {
                Op::Mul => a * b,
                Op::Div => a / b,
                Op::Add => a + b,
                Op::Sub => a - b,
            }
        }
    }
}

fn is_tree(graph: &MonkeyTree) -> bool {
    let root_idx = *graph.names.get("root").unwrap();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(root_idx);

    while let Some(idx) = queue.pop_front() {
        if visited.contains(&idx) {
            return false;
        }

        visited.insert(idx);

        if let Yell::Maths(m) = graph.monkeys[idx] {
            queue.push_back(m.monkeys.0);
            queue.push_back(m.monkeys.1);
        }
    }

    dbg!(visited.len());

    true
}

fn search(target: i64, idx: MonkeyIdx, humn_idx: MonkeyIdx, graph: &mut MonkeyTree) -> i64 {
    let mut value: i64 = 0;
    let mut magnitude: i64 = 2 << 48;

    while magnitude > 0 {
        graph.monkeys[humn_idx] = Yell::Value(value - magnitude);
        let neg_delta = (target - get_value(idx, graph)).abs();

        if neg_delta == 0 {
            return value - magnitude;
        }

        graph.monkeys[humn_idx] = Yell::Value(value + magnitude);
        let pos_delta = (target - get_value(idx, graph)).abs();

        if pos_delta == 0 {
            return value + magnitude;
        }

        if pos_delta < neg_delta {
            value += magnitude;
        } else {
            value -= magnitude;
        }

        magnitude /= 2;
    }

    value
}

type MonkeyIdx = usize;

struct MonkeyTree<'a> {
    names: HashMap<&'a str, MonkeyIdx>,
    monkeys: Vec<Yell>,
}

#[derive(Debug, Clone, Copy)]
enum Yell {
    Value(i64),
    Maths(Maths),
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Mul,
    Div,
    Add,
    Sub,
}

impl Op {
    #[inline]
    fn evaluate<T>(&self, l: T, r: T) -> T
    where
        T: ops::Add<Output = T>
            + ops::Sub<Output = T>
            + ops::Mul<Output = T>
            + ops::Div<Output = T>,
    {
        use Op::*;

        match self {
            Mul => l * r,
            Div => l / r,
            Add => l + r,
            Sub => l - r,
        }
    }

    #[inline]
    fn inverse(&self) -> Self {
        use Op::*;

        match self {
            Mul => Div,
            Div => Mul,
            Add => Sub,
            Sub => Add,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Maths {
    monkeys: (MonkeyIdx, MonkeyIdx),
    op: Op,
}

impl<'a> MonkeyTree<'a> {
    fn build_tree(input: &'a str) -> anyhow::Result<Self> {
        #[derive(Debug, Clone, Copy)]
        enum ParsedYell<'a> {
            Value(i64),
            Maths((&'a str, Op, &'a str)),
        }

        fn parse_line(input: &str) -> IResult<&str, (&str, ParsedYell)> {
            tuple((
                terminated(alpha0, tag(": ")),
                alt((
                    map(i64, ParsedYell::Value),
                    map(
                        tuple((
                            alpha0,
                            alt((
                                value(Op::Add, ws(tag("+"))),
                                value(Op::Sub, ws(tag("-"))),
                                value(Op::Mul, ws(tag("*"))),
                                value(Op::Div, ws(tag("/"))),
                            )),
                            alpha0,
                        )),
                        |(a, op, b)| ParsedYell::Maths((a, op, b)),
                    ),
                )),
            ))(input)
        }

        let (_, lines) = separated_list0(line_ending, parse_line)(input)
            .map_err(|_| anyhow::anyhow!("Parse error!"))?;

        let names: HashMap<&str, MonkeyIdx> = lines
            .iter()
            .enumerate()
            .map(|(i, (name, _))| (*name, i))
            .collect();

        let monkeys: Vec<Yell> = lines
            .into_iter()
            .map(|(_, yell)| match yell {
                ParsedYell::Value(n) => Ok(Yell::Value(n)),
                ParsedYell::Maths((a, op, b)) => {
                    let a_idx = *names.get(a).context("No monkey")?;
                    let b_idx = *names.get(b).context("No monkey")?;

                    Ok(Yell::Maths(Maths {
                        monkeys: (a_idx, b_idx),
                        op,
                    }))
                }
            })
            .collect::<anyhow::Result<_>>()?;

        Ok(Self { names, monkeys })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "\
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn it_works() {
        let monkey_graph = MonkeyTree::build_tree(TEST_INPUT).unwrap();

        let root_idx = *monkey_graph.names.get("root").unwrap();
        let part_1 = get_value(root_idx, &monkey_graph);
        assert_eq!(152, part_1);
    }
}
