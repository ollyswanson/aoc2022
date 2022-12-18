use std::ops::Add;

use hashbrown::HashSet;
use nom::bytes::complete::tag;
use nom::character::complete::{i32, line_ending};
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::{preceded, tuple};
use nom::IResult;

pub fn run(input: &str) -> anyhow::Result<(usize, usize)> {
    let cubes = parse_lines(input)?;
    let flooded = flood_fill(&cubes);

    let (part_1, part_2): (usize, usize) = cubes
        .iter()
        .map(|&tile| {
            (
                DIFFS
                    .iter()
                    .filter(|&&diff| {
                        let tile = diff + tile;
                        !cubes.contains(&tile)
                    })
                    .count(),
                DIFFS
                    .iter()
                    .filter(|&&diff| {
                        let tile = diff + tile;
                        !cubes.contains(&tile) && flooded.contains(&tile)
                    })
                    .count(),
            )
        })
        .fold((0, 0), |acc, elem| (acc.0 + elem.0, acc.1 + elem.1));

    Ok((part_1, part_2))
}

const DIFFS: [Point; 6] = [
    Point { x: -1, y: 0, z: 0 },
    Point { x: 1, y: 0, z: 0 },
    Point { x: 0, y: -1, z: 0 },
    Point { x: 0, y: 1, z: 0 },
    Point { x: 0, y: 0, z: -1 },
    Point { x: 0, y: 0, z: 1 },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Point {
    fn min(self, rhs: Self) -> Self {
        Point {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
            z: self.z.min(rhs.z),
        }
    }

    fn max(self, rhs: Self) -> Self {
        Point {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
            z: self.z.max(rhs.z),
        }
    }
}

type Cubes = HashSet<Point>;

fn flood_fill(cubes: &HashSet<Point>) -> HashSet<Point> {
    // Expanded bounding box
    let bounding_box: (Point, Point) = cubes.iter().fold(
        (
            Point {
                x: -2,
                y: -2,
                z: -2,
            },
            Point { x: 1, y: 1, z: 1 },
        ),
        |acc, &tile| {
            (
                acc.0.min(
                    tile + Point {
                        x: -2,
                        y: -2,
                        z: -2,
                    },
                ),
                acc.1.max(tile + Point { x: 2, y: 2, z: 2 }),
            )
        },
    );

    dbg!(&bounding_box);

    let mut visited = HashSet::new();
    let starting_point = Point { x: 0, y: 0, z: 0 };
    let mut stack = Vec::new();
    stack.push(starting_point);

    while let Some(tile) = stack.pop() {
        visited.insert(tile);
        for tile in DIFFS.iter().map(|&diff| tile + diff) {
            {
                if !visited.contains(&tile)
                    && !cubes.contains(&tile)
                    && tile.x >= bounding_box.0.x
                    && tile.x <= bounding_box.1.x
                    && tile.y >= bounding_box.0.y
                    && tile.y <= bounding_box.1.y
                    && tile.z >= bounding_box.0.z
                    && tile.z <= bounding_box.1.z
                {
                    stack.push(tile)
                }
            }
        }
    }

    visited
}

fn parse_lines(input: &str) -> anyhow::Result<Cubes> {
    fn parse_line(input: &str) -> IResult<&str, Point> {
        map(
            tuple((i32, preceded(tag(","), i32), preceded(tag(","), i32))),
            |(x, y, z)| Point { x, y, z },
        )(input)
    }

    let (_, cubes) = separated_list0(line_ending, parse_line)(input)
        .map_err(|_| anyhow::anyhow!("parse error!"))?;

    Ok(cubes.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_CUBES: &str = include_str!("../inputs/day18_test.txt");

    #[test]
    fn it_works() {
        let (part_1, part_2) = run(TEST_CUBES).unwrap();

        assert_eq!(64, part_1);
        assert_eq!(58, part_2);
    }
}
