use std::fmt::Display;

use itertools::Itertools;

pub fn run(input: &str) -> anyhow::Result<(i64, i64)> {
    let jets: Vec<Jet> = input
        .trim()
        .bytes()
        .map(|b| b.try_into())
        .collect::<anyhow::Result<_>>()?;

    let mut jets = jets.into_iter().cycle();
    let mut chamber = Chamber::<7>::new();

    for i in 0..2022 {
        chamber.drop_rock(i, &mut jets);
    }

    let part_1 = chamber.height;

    let mut deltas = Vec::with_capacity(10000);
    let mut prev_height = part_1;

    for i in 2022..10000 {
        chamber.drop_rock(i, &mut jets);
        deltas.push(chamber.height - prev_height);
        prev_height = chamber.height;
    }

    let period = cycle_detection(&deltas, 5000).unwrap();

    let n = (1_000_000_000_000 - 2022) / period;
    let rem = (1_000_000_000_000 - 2022) % period;

    let delta: i64 = deltas[..period].iter().sum();
    let delta_rem: i64 = deltas[0..rem].iter().sum();

    let part_2 = part_1 + delta * n as i64 + delta_rem;

    Ok((part_1, part_2))
}

fn cycle_detection(deltas: &[i64], upper_bound: usize) -> Option<usize> {
    (1..=upper_bound).find(|&period| {
        (0..period)
            .map(|offset| deltas.iter().skip(offset).step_by(period).all_equal())
            .all(|b| b)
    })
}

struct Chamber<const WIDTH: usize> {
    tiles: Vec<Tile>,
    height: i64,
}

impl<const WIDTH: usize> Display for Chamber<WIDTH> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.tiles.chunks(WIDTH).take(self.height as usize).rev() {
            for jet in row.iter() {
                write!(f, "{}", jet)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl<const WIDTH: usize> Chamber<WIDTH> {
    fn new() -> Self {
        let tiles = vec![Tile::Air; (1 << 16) * WIDTH];

        Self { tiles, height: 0 }
    }

    fn drop_rock<J>(&mut self, turn: usize, jets: &mut J)
    where
        J: Iterator<Item = Jet>,
    {
        let mut rock = Rock::new((2, self.height + 3), turn);

        loop {
            let jet = jets.next().unwrap();
            let maybe_next = rock.move_jet(jet);

            if !maybe_next.out_of_bounds(WIDTH) && !self.collision(&maybe_next) {
                rock = maybe_next;
            }

            let maybe_next = rock.move_down();

            if maybe_next.out_of_bounds(WIDTH) || self.collision(&maybe_next) {
                self.settle_rock(rock);
                break;
            } else {
                rock = maybe_next;
            }
        }
    }

    fn collision(&self, rock: &Rock) -> bool {
        let pos = rock.pos;
        for tile_pos in rock
            .shape
            .iter()
            .map(|tile| (tile.0 + pos.0, tile.1 + pos.1))
        {
            if self.get_tile(tile_pos) == Tile::Rock {
                return true;
            }
        }

        false
    }

    fn get_tile(&self, (x, y): (i64, i64)) -> Tile {
        self.tiles[y as usize * WIDTH + x as usize]
    }

    fn settle_rock(&mut self, rock: Rock) {
        let pos = rock.pos;
        for tile_pos in rock
            .shape
            .iter()
            .map(|tile| (tile.0 + pos.0, tile.1 + pos.1))
        {
            self.set_tile(tile_pos, Tile::Rock);
        }

        self.height = self.height.max(rock.pos.1 + rock.bb.1);
    }

    fn set_tile(&mut self, (x, y): (i64, i64), tile: Tile) {
        self.tiles[y as usize * WIDTH + x as usize] = tile
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Air,
    Rock,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Air => f.write_str("."),
            Tile::Rock => f.write_str("#"),
        }
    }
}

struct Rock {
    pos: (i64, i64),
    /// Bounding box
    bb: BoundingBox,
    shape: Shape,
}

impl Rock {
    fn new(pos: (i64, i64), turn: usize) -> Self {
        let (shape, bb) = SHAPES[turn % 5];

        Self { pos, bb, shape }
    }

    fn move_jet(&self, jet: Jet) -> Self {
        let pos = match jet {
            Jet::L => (self.pos.0 - 1, self.pos.1),
            Jet::R => (self.pos.0 + 1, self.pos.1),
        };

        Self {
            pos,
            bb: self.bb,
            shape: self.shape,
        }
    }

    fn move_down(&self) -> Self {
        Self {
            pos: (self.pos.0, self.pos.1 - 1),
            bb: self.bb,
            shape: self.shape,
        }
    }

    fn out_of_bounds(&self, width: usize) -> bool {
        if self.pos.0 < 0 || self.pos.0 + self.bb.0 > width as i64 {
            return true;
        }

        if self.pos.1 < 0 {
            return true;
        }

        false
    }
}

type BoundingBox = (i64, i64);
type Shape = &'static [(i64, i64)];

static SHAPES: &[(Shape, BoundingBox)] = &[
    (&[(0, 0), (1, 0), (2, 0), (3, 0)], (4, 1)),
    (&[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)], (3, 3)),
    (&[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], (3, 3)),
    (&[(0, 0), (0, 1), (0, 2), (0, 3)], (1, 4)),
    (&[(0, 0), (1, 0), (0, 1), (1, 1)], (2, 2)),
];

#[derive(Debug, Clone, Copy)]
enum Jet {
    L,
    R,
}

impl TryFrom<u8> for Jet {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'<' => Ok(Jet::L),
            b'>' => Ok(Jet::R),
            _ => anyhow::bail!("invalid pattern"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_JETS: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn it_works() {
        let (part_1, part_2) = run(TEST_JETS).unwrap();

        assert_eq!(3068, part_1);
        assert_eq!(1514285714288, part_2);
    }
}
