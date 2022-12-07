use std::path::PathBuf;

use hashbrown::HashMap;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, multispace0};
use nom::combinator::{map, rest};
use nom::error::ParseError;
use nom::sequence::{delimited, preceded, separated_pair};
use nom::IResult;

pub fn run(input: &str) -> anyhow::Result<(u64, u64)> {
    let sizes = DirectorySizes::from_terminal_output(input)?;

    Ok((part_1(&sizes), part_2(&sizes)))
}

struct DirectorySizes(HashMap<PathBuf, u64>);

impl DirectorySizes {
    fn from_terminal_output(input: &str) -> anyhow::Result<Self> {
        use Line::*;

        let mut cwd = PathBuf::new();
        let mut sizes = HashMap::new();

        for line in input.lines() {
            let line = parse_line(line)?;

            match line {
                Cd(d) if d == ".." => {
                    cwd.pop();
                }
                Cd(d) => {
                    cwd.push(d);
                    sizes.insert(cwd.clone(), 0);
                }
                File((file_size, _)) => {
                    if let Some(size) = sizes.get_mut(&cwd) {
                        *size += file_size;
                    }

                    let mut dir = cwd.as_path();

                    while let Some(parent) = dir.parent() {
                        if let Some(size) = sizes.get_mut(parent) {
                            *size += file_size;
                        }
                        dir = parent
                    }
                }
                _ => {}
            }
        }

        Ok(Self(sizes))
    }
}

fn part_1(sizes: &DirectorySizes) -> u64 {
    sizes.0.values().filter(|&&size| size <= 100_000).sum()
}

fn part_2(sizes: &DirectorySizes) -> u64 {
    let disk_space = 70000000;
    let desired = 30000000;

    let used = sizes.0.get(&PathBuf::from("/")).unwrap();
    let unused_space = disk_space - used;
    let to_free = desired - unused_space;

    sizes
        .0
        .values()
        .filter(|&&size| size >= to_free)
        .min()
        .cloned()
        .unwrap()
}

#[derive(Debug, PartialEq, Eq)]
enum Line<'a> {
    Cd(&'a str),
    Ls,
    File((u64, &'a str)),
    Dir(&'a str),
}

fn parse_line(input: &str) -> anyhow::Result<Line<'_>> {
    let (_, line) = alt((parse_file, alt((parse_dir, alt((parse_ls, parse_cd))))))(input)
        .map_err(|e| anyhow::anyhow!(e.to_owned()))?;

    Ok(line)
}

// $ cd /
fn parse_cd(input: &str) -> IResult<&str, Line<'_>> {
    map(
        preceded(ws(tag("$")), preceded(ws(tag("cd")), rest)),
        Line::Cd,
    )(input)
}

// $ ls
fn parse_ls(input: &str) -> IResult<&str, Line<'_>> {
    map(preceded(ws(tag("$")), tag("ls")), |_| Line::Ls)(input)
}

// dir a
fn parse_dir(input: &str) -> IResult<&str, Line<'_>> {
    map(preceded(ws(tag("dir")), alpha1), Line::Dir)(input)
}

// 1485 b.txt
fn parse_file(input: &str) -> IResult<&str, Line<'_>> {
    use nom::character::complete::u64;

    map(separated_pair(u64, tag(" "), rest), Line::File)(input)
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

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn parse_cd_works() {
        let input = "$ cd d";
        let expected = Line::Cd("d");

        assert_eq!(expected, parse_cd(input).unwrap().1);
    }

    #[test]
    fn parse_ls_works() {
        let input = "$ ls";
        let expected = Line::Ls;

        assert_eq!(expected, parse_ls(input).unwrap().1);
    }

    #[test]
    fn parse_dir_works() {
        let input = "dir a";
        let expected = Line::Dir("a");

        assert_eq!(expected, parse_dir(input).unwrap().1);
    }

    #[test]
    fn parse_file_works() {
        let input = "8504156 c.dat";
        let expected = Line::File((8504156, "c.dat"));

        assert_eq!(expected, parse_file(input).unwrap().1);
    }

    #[test]
    fn part_1_works() {
        let sizes = DirectorySizes::from_terminal_output(TEST_INPUT).unwrap();

        assert_eq!(95437, part_1(&sizes));
    }

    #[test]
    fn part_2_works() {
        let sizes = DirectorySizes::from_terminal_output(TEST_INPUT).unwrap();

        assert_eq!(24933642, part_2(&sizes));
    }
}
