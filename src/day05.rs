use std::cmp;
use std::str::FromStr;

use anyhow::anyhow;
use nom::bytes::complete::tag;
use nom::character::complete::{multispace0, u32};
use nom::combinator::map;
use nom::error::ParseError;
use nom::multi::many0;
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;

pub fn run(input: &str) -> anyhow::Result<(String, String)> {
    let (mut supplies, moves) = parse_input::<9>(input)?;
    let mut supplies_alt = supplies.clone();

    supplies.move_supplies(&moves);
    supplies_alt.move_supplies_queue(&moves);

    Ok((supplies.tops(), supplies_alt.tops()))
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Supplies<const N: usize> {
    stacks: [Vec<u8>; N],
}

impl<const N: usize> Supplies<N> {
    #[inline]
    fn move_as_stack(&mut self, instr: &CraneInstr) {
        // Normalise from and to
        let (from, to) = (instr.from - 1, instr.to - 1);
        for _ in 0..instr.amount {
            if let Some(supply) = self.stacks[from].pop() {
                self.stacks[to].push(supply)
            }
        }
    }

    #[inline]
    fn move_as_queue(&mut self, instr: &CraneInstr) {
        let (from, to) = (instr.from - 1, instr.to - 1);

        // We don't need to do any work if from = to, returning early is required for safety
        if from == to {
            return;
        }

        let len = self.stacks[from].len();

        // Take at most the capacity of the stack
        let amount = cmp::min(instr.amount, len);
        let offset = len - amount;

        let to: *mut _ = &mut self.stacks[to];
        let from = &mut self.stacks[from];

        // Safety: from != to
        unsafe { (*to).extend_from_slice(&from[offset..]) }

        from.truncate(offset);
    }

    fn move_supplies(&mut self, moves: &[CraneInstr]) {
        for instr in moves {
            self.move_as_stack(instr);
        }
    }

    fn move_supplies_queue(&mut self, moves: &[CraneInstr]) {
        for instr in moves {
            self.move_as_queue(instr);
        }
    }

    fn tops(&self) -> String {
        let tops = self
            .stacks
            .iter()
            .filter_map(|stack| stack.last())
            .copied()
            .collect();

        String::from_utf8(tops).unwrap()
    }
}

impl<const N: usize> FromStr for Supplies<N> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stacks = [(); N].map(|_| Vec::new());

        for line in s.as_bytes().chunks(4 * N).rev().skip(1) {
            for (i, chunk) in line.chunks(4).enumerate() {
                let elf_crate = chunk[1];
                if elf_crate.is_ascii_uppercase() {
                    stacks[i].push(elf_crate);
                }
            }
        }

        Ok(Self { stacks })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct CraneInstr {
    amount: usize,
    from: usize,
    to: usize,
}

fn parse_input<const N: usize>(input: &str) -> anyhow::Result<(Supplies<N>, Vec<CraneInstr>)> {
    let (stacks, moves) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("parse error"))?;

    let stacks: Supplies<N> = stacks.parse()?;
    let moves = parse_crane_moves(moves)?;

    Ok((stacks, moves))
}

// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
// trailing whitespace, returning the output of `inner`.
fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

// move 7 from 3 to 9
fn parse_crane_move(input: &str) -> IResult<&str, CraneInstr> {
    map(
        tuple((
            preceded(ws(tag("move")), u32),
            preceded(ws(tag("from")), u32),
            preceded(ws(tag("to")), u32),
        )),
        |(a, b, c)| CraneInstr {
            amount: a as usize,
            from: b as usize,
            to: c as usize,
        },
    )(input)
}

fn parse_crane_moves(input: &str) -> anyhow::Result<Vec<CraneInstr>> {
    let (_, moves) = many0(parse_crane_move)(input).map_err(|_| anyhow::anyhow!("Parse error!"))?;

    Ok(moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn parse_crane_move_works() {
        let input = "move 7 from 3 to 9";
        let expected = CraneInstr {
            amount: 7,
            from: 3,
            to: 9,
        };

        assert_eq!(expected, parse_crane_move(input).unwrap().1);
    }

    #[test]
    fn parse_crane_moves_works() {
        let input = "\
move 7 from 3 to 9
move 8 from 1 to 7";
        let expected = vec![
            CraneInstr {
                amount: 7,
                from: 3,
                to: 9,
            },
            CraneInstr {
                amount: 8,
                from: 1,
                to: 7,
            },
        ];

        assert_eq!(expected, parse_crane_moves(input).unwrap());
    }

    #[test]
    fn part_1_works() {
        let (mut stacks, moves) = parse_input::<3>(&TEST_INPUT[1..]).unwrap();

        stacks.move_supplies(&moves);

        assert_eq!("CMZ", stacks.tops());
    }

    #[test]
    fn part_2_works() {
        let (mut stacks, moves) = parse_input::<3>(&TEST_INPUT[1..]).unwrap();

        stacks.move_supplies_queue(&moves);

        assert_eq!("MCD", stacks.tops());
    }
}
