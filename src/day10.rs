use std::io::{self, Write};

use anyhow::Context;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{i64, newline};
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::{preceded, terminated};
use nom::IResult;

pub fn run(input: &str) -> anyhow::Result<()> {
    let instrs = parse_instrs(input)?;
    let mut cpu = Cpu::new();
    let register_vals = cpu.execute(&instrs);

    let sum = signal_sum(&register_vals);
    println!("Part 1: {}", sum);

    println!("Part 2:\n");
    draw(&register_vals)?;

    Ok(())
}

fn signal_sum(register_vals: &[i64]) -> i64 {
    (1..=240)
        .zip(register_vals.iter())
        .filter(|(clock, _)| (clock - 20) % 40 == 0)
        .map(|(clock, &r)| clock * r)
        .sum()
}

fn draw(register_vals: &[i64]) -> anyhow::Result<()> {
    let pixels: Vec<u8> = (0..240)
        .zip(register_vals.iter())
        .map(|(clock, &r)| {
            if (r - (clock % 40)).abs() < 2 {
                b'#'
            } else {
                b' '
            }
        })
        .collect();

    let mut out = io::stdout();
    for line in pixels.chunks(40) {
        out.write_all(line)?;
        out.write_all(&[b'\n'])?;
    }
    out.flush().context("Failed to flush")
}

struct Cpu {
    register: i64,
}

impl Cpu {
    fn new() -> Self {
        Self { register: 1 }
    }

    fn execute(&mut self, instrs: &[Instr]) -> Vec<i64> {
        let mut register_vals = vec![1];
        for instr in instrs {
            match instr {
                Instr::Noop => {
                    register_vals.push(self.register);
                }
                Instr::Add(x) => {
                    register_vals.push(self.register);
                    self.register += x;
                    register_vals.push(self.register);
                }
            }
        }
        register_vals
    }
}

enum Instr {
    Noop,
    Add(i64),
}

fn parse_instrs(input: &str) -> anyhow::Result<Vec<Instr>> {
    let (_, instrs) = many0(terminated(parse_instr, newline))(input)
        .map_err(|_| anyhow::anyhow!("Parse error"))?;
    Ok(instrs)
}

fn parse_instr(input: &str) -> IResult<&str, Instr> {
    alt((
        map(tag("noop"), |_| Instr::Noop),
        map(preceded(tag("addx "), i64), Instr::Add),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_INSTRS: &str = include_str!("../inputs/day10_test.txt");

    #[test]
    fn part_1_works() {
        let instrs = parse_instrs(TEST_INSTRS).unwrap();
        let mut cpu = Cpu::new();
        let register_vals = cpu.execute(&instrs);

        let sum = signal_sum(&register_vals);
        assert_eq!(13140, sum);
    }
}
