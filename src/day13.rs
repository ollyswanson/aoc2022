use std::cmp::{Ord, Ordering, PartialOrd};
use std::iter::Peekable;
use std::str::Chars;

pub fn run(input: &str) -> anyhow::Result<(usize, usize)> {
    let pairs: Vec<(List, List)> = Parser::new(input).collect();

    let part_1: usize = pairs
        .iter()
        .enumerate()
        .filter_map(|(i, (l, r))| l.partial_cmp(r).map(|o| (i, o)))
        .filter(|(_, o)| o == &Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum();

    let mut packets = Vec::new();
    for pair in pairs {
        packets.push(pair.0);
        packets.push(pair.1);
    }

    let divider_1 = create_divider(2);
    packets.push(divider_1.clone());

    let divider_2 = create_divider(6);
    packets.push(divider_2.clone());

    packets.sort();

    let pos_1 = packets.binary_search(&divider_1).unwrap() + 1;
    let pos_2 = packets.binary_search(&divider_2).unwrap() + 1;

    Ok((part_1, pos_1 * pos_2))
}

fn create_divider(value: u32) -> List {
    List {
        items: vec![Item::List(List {
            items: vec![Item::Int(value)],
        })],
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct List {
    items: Vec<Item>,
}

impl List {
    fn new() -> Self {
        Self { items: Vec::new() }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Item {
    List(List),
    Int(u32),
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Item::Int(a), Item::Int(b)) => Some(a.cmp(b)),
            (l @ Item::Int(_), Item::List(r)) => {
                if r.items.is_empty() {
                    Some(Ordering::Greater)
                } else {
                    match l.cmp(&r.items[0]) {
                        o if o == Ordering::Equal && r.items.len() > 1 => Some(Ordering::Less),
                        o => Some(o),
                    }
                }
            }
            (Item::List(l), r @ Item::Int(_)) => {
                if l.items.is_empty() {
                    Some(Ordering::Less)
                } else {
                    match r.cmp(&l.items[0]) {
                        o if o == Ordering::Equal && l.items.len() > 1 => Some(Ordering::Greater),
                        Ordering::Less => Some(Ordering::Greater),
                        Ordering::Greater => Some(Ordering::Less),
                        Ordering::Equal => Some(Ordering::Equal),
                    }
                }
            }
            (Item::List(l), Item::List(r)) => l.partial_cmp(r),
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

struct Parser<'a> {
    input: Peekable<Chars<'a>>,
    eof: bool,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
            eof: false,
        }
    }

    fn parse_pair(&mut self) -> (List, List) {
        let l = self.parse_list();
        self.expect_next('\n');
        let r = self.parse_list();

        if self.input.peek() == Some(&'\n') {
            self.input.next();
            if self.input.peek() == Some(&'\n') {
                self.input.next();
            } else {
                self.eof = true;
            }
        } else {
            self.eof = true;
        }

        (l, r)
    }

    // [[1],[2,3,4]]
    fn parse_list(&mut self) -> List {
        let mut list = List::new();

        // Consume opening square bracket
        self.expect_next('[');

        if self.input.peek() == Some(&']') {
            self.input.next();
            return list;
        }

        list.items.push(self.next_item());

        while self.input.peek() == Some(&',') {
            self.input.next();
            list.items.push(self.next_item());
        }

        // Consume closing square bracket
        self.expect_next(']');

        list
    }

    fn next_item(&mut self) -> Item {
        match self.input.peek() {
            Some(c) if c.is_ascii_digit() => Item::Int(self.parse_num()),
            Some('[') => Item::List(self.parse_list()),
            other => panic!("Parse error: {:?}", other),
        }
    }

    fn parse_num(&mut self) -> u32 {
        let mut num = String::new();

        while let Some(c) = self.input.next_if(char::is_ascii_digit) {
            num.push(c);
        }

        num.parse().unwrap()
    }

    #[inline]
    fn expect_next(&mut self, expected: char) {
        match self.input.next() {
            Some(c) if c == expected => {}
            _ => panic!(),
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = (List, List);

    fn next(&mut self) -> Option<Self::Item> {
        if self.eof {
            None
        } else {
            Some(self.parse_pair())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_PAIRS: &str = include_str!("../inputs/day13_test.txt");

    #[test]
    fn parses_a_list() {
        let input = "[[1],[2,3,4]]";
        let expected = List {
            items: vec![
                Item::List(List {
                    items: vec![Item::Int(1)],
                }),
                Item::List(List {
                    items: vec![Item::Int(2), Item::Int(3), Item::Int(4)],
                }),
            ],
        };

        let mut parser = Parser::new(input);

        assert_eq!(expected, parser.parse_list());
    }

    #[test]
    fn part_1_works() {
        let pairs: Vec<(List, List)> = Parser::new(TEST_PAIRS).collect();

        let part_1: usize = pairs
            .iter()
            .enumerate()
            .filter_map(|(i, (l, r))| l.partial_cmp(r).map(|o| (i, o)))
            .filter(|(_, o)| o == &Ordering::Less)
            .map(|(i, _)| i + 1)
            .sum();

        assert_eq!(13, part_1);
    }
}
