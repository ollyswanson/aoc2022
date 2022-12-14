use anyhow::Context;
use hashbrown::HashMap;
use itertools::Itertools;

pub fn run(input: &str) -> anyhow::Result<(usize, usize)> {
    let mut cave = Cave::build(input)?;

    let drops = cave.fill_from_until((500, 0), |cave, settled_at| settled_at.1 > cave.ymax);
    let more_drops = cave.fill_from_until((500, 0), |_, settled_at| settled_at == (500, 0));

    // Take 1 off the drops because we're are dropping sand until it falls into the abyss, but
    // we're counting how many drops there are _before_ it falls into the abyss
    Ok((drops - 1, drops + more_drops))
}

struct Cave {
    tiles: HashMap<(i32, i32), Tile>,
    ymax: i32,
}

impl Cave {
    fn fill_from_until<F>(&mut self, from: (i32, i32), mut pred: F) -> usize
    where
        F: FnMut(&Self, (i32, i32)) -> bool,
    {
        let mut drops = 0;

        loop {
            let settled_at = self.drop_sand(from);
            drops += 1;

            if pred(self, settled_at) {
                return drops;
            }
        }
    }

    #[inline]
    fn drop_sand(&mut self, (mut x, mut y): (i32, i32)) -> (i32, i32) {
        'outer: loop {
            let next_y = y + 1;

            for next_x in [x, x - 1, x + 1] {
                if self.get_tile((next_x, next_y)) == Tile::Air {
                    (x, y) = (next_x, next_y);
                    continue 'outer;
                }
            }

            self.set_tile((x, y), Tile::Sand);
            return (x, y);
        }
    }

    fn get_tile(&self, coord: (i32, i32)) -> Tile {
        if coord.1 >= self.ymax + 2 {
            Tile::Rock
        } else {
            self.tiles.get(&coord).copied().unwrap_or(Tile::Air)
        }
    }

    fn set_tile(&mut self, coord: (i32, i32), tile: Tile) {
        self.tiles.insert(coord, tile);
    }

    fn build(input: &str) -> anyhow::Result<Self> {
        let mut ymax = 0;
        let mut tiles = HashMap::new();

        for line in input.lines() {
            for (a, b) in line.split(" -> ").tuple_windows() {
                let (xa, ya) = a.split_once(',').context("Invalid input")?;
                let (xa, ya): (i32, i32) = (xa.parse()?, ya.parse()?);

                let (xb, yb) = b.split_once(',').context("Invalid input")?;
                let (xb, yb): (i32, i32) = (xb.parse()?, yb.parse()?);

                if ya == yb {
                    let start = xa.min(xb);
                    let end = xa.max(xb);
                    for x in start..=end {
                        tiles.insert((x, ya), Tile::Rock);
                    }
                } else if xa == xb {
                    let start = ya.min(yb);
                    let end = ya.max(yb);
                    for y in start..=end {
                        tiles.insert((xa, y), Tile::Rock);
                    }
                }

                ymax = ya.max(yb).max(ymax);
            }
        }

        Ok(Self { tiles, ymax })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Air,
    Rock,
    Sand,
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn part_1_works() {
        let mut cave = Cave::build(TEST_INPUT).unwrap();

        let drops = cave.fill_from_until((500, 0), |cave, settled_at| settled_at.1 > cave.ymax) - 1;
        assert_eq!(24, drops);
    }

    #[test]
    fn part_2_works() {
        let mut cave = Cave::build(TEST_INPUT).unwrap();

        let drops = cave.fill_from_until((500, 0), |_, settled_at| settled_at == (500, 0));
        assert_eq!(93, drops);
    }
}
