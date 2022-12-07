use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, multispace0};
use nom::combinator::{map, rest};
use nom::error::ParseError;
use nom::sequence::{delimited, preceded, separated_pair};
use nom::IResult;

pub fn run(input: &str) -> anyhow::Result<(u64, u64)> {
    let heap = DirectoryHeap::build_heap_from_input(input)?;

    Ok((heap.part_1(), heap.part_2()))
}

type HeapIdx = usize;

struct DirectoryHeap {
    /// A heap of file system entries, we only concern ourselves with tracking directories
    heap: Vec<HeapEntry>,
    /// The index of the most recently inserted `HeapEntry`
    cursor: HeapIdx,
}

struct HeapEntry {
    /// The parent of the directory, i.e. ".."
    parent: HeapIdx,
    /// The size of all of the files in the directory as well as the sizes of the subdirectories in
    /// the directory
    size: u64,
}

impl DirectoryHeap {
    /// A new directory heap is always created with a root directory whose parent is itself
    fn new() -> Self {
        Self {
            heap: vec![HeapEntry { parent: 0, size: 0 }],
            cursor: 0,
        }
    }

    /// Takes the `HeapIdx` of the entry's parent and returns the new entry's `HeapIdx`.
    fn insert_entry(&mut self, parent: HeapIdx) -> HeapIdx {
        self.heap.push(HeapEntry { parent, size: 0 });
        self.cursor += 1;
        self.cursor
    }

    /// Returns the heap index of the parent
    fn get_parent(&self, idx: HeapIdx) -> HeapIdx {
        self.heap[idx].parent
    }

    /// Takes the `HeapIdx` of the directory that we want to update. This function updates the size
    /// of the entry and all of its parents up to root
    fn update_size(&mut self, mut idx: HeapIdx, size: u64) {
        loop {
            let entry = &mut self.heap[idx];
            entry.size += size;
            if idx == entry.parent {
                break;
            } else {
                idx = entry.parent;
            }
        }
    }

    fn build_heap_from_input(input: &str) -> anyhow::Result<Self> {
        use Line::*;

        let mut heap = DirectoryHeap::new();
        let mut cwd = heap.cursor;

        // Skip the first line because we have already initialized the heap with root
        for line in input.lines().skip(1) {
            let line = parse_line(line)?;

            match line {
                Cd(d) if d == ".." => {
                    cwd = heap.get_parent(cwd);
                }
                Cd(_) => {
                    cwd = heap.insert_entry(cwd);
                }
                File((file_size, _)) => {
                    heap.update_size(cwd, file_size);
                }
                _ => {}
            }
        }

        Ok(heap)
    }

    /// Part 1
    fn part_1(&self) -> u64 {
        self.heap
            .iter()
            .map(|entry| entry.size)
            .filter(|&size| size <= 100_000)
            .sum()
    }

    fn part_2(&self) -> u64 {
        let disk_space = 70_000_000;
        let desired_free_space = 30_000_000;
        let used = self.heap[0].size;
        let unused_space = disk_space - used;
        let to_free = desired_free_space - unused_space;

        self.heap
            .iter()
            .map(|entry| entry.size)
            .filter(|&size| size >= to_free)
            .min()
            .unwrap()
    }
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
        let heap = DirectoryHeap::build_heap_from_input(TEST_INPUT).unwrap();

        assert_eq!(95437, heap.part_1());
    }

    #[test]
    fn part_2_works() {
        let heap = DirectoryHeap::build_heap_from_input(TEST_INPUT).unwrap();

        assert_eq!(24933642, heap.part_2());
    }
}
