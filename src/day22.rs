use anyhow::Context;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, u32};
use nom::combinator::{map, value};
use nom::multi::many_till;
use nom::IResult;

pub fn run(input: &str) -> anyhow::Result<(isize, isize)> {
    let (grid, moves) = input.split_once("\n\n").context("Invalid input")?;

    let grid = Grid::build_grid(grid, 150)?;
    let moves = parse_moves(moves)?;

    let part_1 = play(&grid, &moves);

    Ok((part_1, play_with_warp(&grid, &moves)))
}

fn play(grid: &Grid, instructions: &[Move]) -> isize {
    let mut player = Player {
        position: Vector::new(0, 0),
        orientation: Vector::new(1, 0),
    };

    for instr in instructions {
        match instr {
            Move::Turn(turn) => match turn {
                Turn::Right => player.orientation = player.orientation.rotate_right(),
                Turn::Left => player.orientation = player.orientation.rotate_left(),
            },
            Move::Ahead(amt) => {
                for _ in 0..*amt {
                    let (next_pos, tile) = grid.next_inbounds_tile(&player);
                    if tile == Tile::Open {
                        player.position = next_pos;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    player.score()
}

fn play_with_warp(grid: &Grid, instructions: &[Move]) -> isize {
    let mut player = Player {
        position: Vector::new(50, 0),
        orientation: Vector::new(1, 0),
    };

    for instr in instructions {
        match instr {
            Move::Turn(turn) => match turn {
                Turn::Right => player.orientation = player.orientation.rotate_right(),
                Turn::Left => player.orientation = player.orientation.rotate_left(),
            },
            Move::Ahead(amt) => {
                for _ in 0..*amt {
                    let (next_pos, orientation, tile) = grid.next_tile_warp(&player);
                    if tile == Tile::Open {
                        player.position = next_pos;
                        player.orientation = orientation;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    player.score()
}

#[derive(Debug, PartialEq, Eq)]
struct Player {
    position: Vector,
    orientation: Vector,
}

impl Player {
    fn score(&self) -> isize {
        (self.position.x + 1) * 4
            + (self.position.y + 1) * 1000
            + match (self.orientation.x, self.orientation.y) {
                (1, 0) => 0,
                (0, 1) => 1,
                (-1, 0) => 2,
                (0, -1) => 3,
                _ => unreachable!(),
            }
    }
}

/// Used to represent the position and orientation of a [`Rover`] in a 2D grid.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Vector {
    x: isize,
    y: isize,
}

impl Vector {
    fn new(x: isize, y: isize) -> Vector {
        Vector { x, y }
    }

    fn rotate_right(&self) -> Vector {
        Vector {
            x: -self.y,
            y: self.x,
        }
    }

    fn rotate_left(&self) -> Vector {
        Vector {
            x: self.y,
            y: -self.x,
        }
    }

    fn wrapping_add(&self, rhs: Self, rows: isize, columns: isize) -> Self {
        Self {
            x: (self.x + rhs.x).rem_euclid(columns),
            y: (self.y + rhs.y).rem_euclid(rows),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Ahead(usize),
    Turn(Turn),
}

#[derive(Debug)]
struct Grid {
    tiles: Vec<Tile>,
    columns: usize,
    rows: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Open,
    Wall,
    /// Out of bounds
    Oob,
}

impl Grid {
    fn next_inbounds_tile(&self, player: &Player) -> (Vector, Tile) {
        let mut next_position = player.position.wrapping_add(
            player.orientation,
            self.rows as isize,
            self.columns as isize,
        );

        while self.get_tile(next_position) == Tile::Oob {
            next_position = next_position.wrapping_add(
                player.orientation,
                self.rows as isize,
                self.columns as isize,
            );
        }

        (next_position, self.get_tile(next_position))
    }

    fn next_tile_warp(&self, player: &Player) -> (Vector, Vector, Tile) {
        let next_position = player.position.wrapping_add(
            player.orientation,
            self.rows as isize,
            self.columns as isize,
        );

        match self.get_tile(next_position) {
            Tile::Oob => {
                let (next_position, orientation) = self.warp(&next_position, &player.orientation);
                let tile = self.get_tile(next_position);

                (next_position, orientation, tile)
            }
            tile => (next_position, player.orientation, tile),
        }
    }

    fn get_tile(&self, p: Vector) -> Tile {
        self.tiles[p.y as usize * self.columns + p.x as usize]
    }

    fn warp(&self, position: &Vector, orientation: &Vector) -> (Vector, Vector) {
        // Warp zones
        match (
            position.x / 50,
            position.y / 50,
            orientation.x,
            orientation.y,
        ) {
            (0, 0, 1, 0) => (Vector::new(99, 150 - position.y - 1), Vector::new(-1, 0)),
            (2, 2, 1, 0) => (
                Vector::new(149, 50 - (position.y % 50) - 1),
                Vector::new(-1, 0),
            ),
            (2, 3, 0, -1) => (Vector::new(position.x - 100, 199), *orientation),
            (0, 0, 0, 1) => (Vector::new(position.x + 100, 0), *orientation),
            (1, 3, 0, -1) => (Vector::new(0, 150 + position.x % 50), Vector::new(1, 0)),
            (2, 3, -1, 0) => (Vector::new(50 + position.y % 50, 0), Vector::new(0, 1)),
            (2, 2, -1, 0) => (Vector::new(50, 50 - position.y % 50 - 1), Vector::new(1, 0)),
            (0, 0, -1, 0) => (Vector::new(0, 150 - position.y % 50 - 1), Vector::new(1, 0)),
            (2, 1, 0, 1) => (Vector::new(99, 50 + position.x % 50), Vector::new(-1, 0)),
            (2, 1, 1, 0) => (Vector::new(100 + position.y % 50, 49), Vector::new(0, -1)),
            (1, 3, 0, 1) => (Vector::new(49, position.x % 50 + 150), Vector::new(-1, 0)),
            (1, 3, 1, 0) => (Vector::new(50 + position.y % 50, 149), Vector::new(0, -1)),
            (0, 1, 0, -1) => (Vector::new(50, position.x % 50 + 50), Vector::new(1, 0)),
            (0, 1, -1, 0) => (Vector::new(position.y % 50, 100), Vector::new(0, 1)),
            _ => panic!(
                "Position was {:?}, orientation was {:?}",
                position, orientation
            ),
        }
    }

    fn build_grid(input: &str, columns: usize) -> anyhow::Result<Self> {
        let mut rows: usize = 0;
        let mut tiles: Vec<Tile> = Vec::new();

        for line in input.lines() {
            rows += 1;
            let bytes = line.as_bytes();
            let len = bytes.len();

            if bytes.len() > columns {
                anyhow::bail!("incorrect column size");
            }

            for b in bytes {
                match b {
                    b'.' => tiles.push(Tile::Open),
                    b'#' => tiles.push(Tile::Wall),
                    b' ' => tiles.push(Tile::Oob),
                    _ => anyhow::bail!("unrecognised tile type"),
                }
            }

            // Pad the end of the row with out of bounds tiles so that all rows are the same length
            for _ in 0..columns - len {
                tiles.push(Tile::Oob)
            }
        }

        Ok(Self {
            tiles,
            columns,
            rows,
        })
    }
}

fn parse_moves(input: &str) -> anyhow::Result<Vec<Move>> {
    fn parse_move(input: &str) -> IResult<&str, Move> {
        alt((
            value(Move::Turn(Turn::Left), tag("L")),
            value(Move::Turn(Turn::Right), tag("R")),
            map(u32, |n| Move::Ahead(n as usize)),
        ))(input)
    }

    let (_, (moves, _)) = many_till(parse_move, line_ending)(input)
        .map_err(|_| anyhow::anyhow!("error parsing moves"))?;

    Ok(moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../inputs/day22_test.txt");

    #[test]
    fn part_1_works() {
        let (grid, moves) = TEST_INPUT.split_once("\n\n").unwrap();

        let grid = Grid::build_grid(grid, 16).unwrap();
        let moves = parse_moves(moves).unwrap();

        let part_1 = play(&grid, &moves);

        assert_eq!(6032, part_1);
    }
}
