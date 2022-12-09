use std::ops;

use hashbrown::HashSet;

pub fn run(input: &str) -> anyhow::Result<(usize, usize)> {
    let moves = parse_moves(input)?;
    let mut rope = Rope::new(2);
    rope.follow_moves(&moves);
    let part_1 = rope.tail_history.len();

    let mut rope = Rope::new(10);
    rope.follow_moves(&moves);
    let part_2 = rope.tail_history.len();

    Ok((part_1, part_2))
}

#[derive(Debug)]
struct Rope {
    knots: Vec<RopeVector>,
    tail_history: HashSet<RopeVector>,
}

impl Rope {
    fn new(knot_count: usize) -> Self {
        let knots = vec![RopeVector { x: 0, y: 0 }; knot_count];
        Self {
            knots,
            tail_history: HashSet::new(),
        }
    }
    fn mov(&mut self, mov: &Move) {
        for _ in 0..mov.distance {
            self.knots[0] += mov.direction;
            for i in 1..self.knots.len() {
                let leader = self.knots[i - 1];
                self.knots[i].follow(leader);
            }
            self.tail_history.insert(*self.knots.last().unwrap());
        }
    }

    fn follow_moves(&mut self, moves: &[Move]) {
        for mov in moves {
            self.mov(mov);
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Move {
    direction: RopeVector,
    distance: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RopeVector {
    x: i32,
    y: i32,
}

impl RopeVector {
    fn follow(&mut self, other: RopeVector) {
        let dx = other.x - self.x;
        let dy = other.y - self.y;

        if dx.abs() > 1 || dy.abs() > 1 {
            self.x += dx.checked_div(dx.abs()).unwrap_or(0);
            self.y += dy.checked_div(dy.abs()).unwrap_or(0);
        }
    }
}

impl ops::Add for RopeVector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        RopeVector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign for RopeVector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

fn parse_moves(input: &str) -> anyhow::Result<Vec<Move>> {
    input
        .lines()
        .map(|line| {
            let mut line = line.split_whitespace();
            let direction = line.next().and_then(|dir| match dir {
                "R" => Some(RopeVector { x: 1, y: 0 }),
                "L" => Some(RopeVector { x: -1, y: 0 }),
                "U" => Some(RopeVector { x: 0, y: 1 }),
                "D" => Some(RopeVector { x: 0, y: -1 }),
                _ => None,
            });
            let distance = line.next().and_then(|dist| dist.parse::<u32>().ok());

            match (direction, distance) {
                (Some(direction), Some(distance)) => Ok(Move {
                    direction,
                    distance,
                }),
                _ => Err(anyhow::anyhow!("Parse error")),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_MOVES: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn part_1_works() {
        let moves = parse_moves(TEST_MOVES).unwrap();
        let mut rope = Rope::new(2);
        rope.follow_moves(&moves);

        assert_eq!(13, rope.tail_history.len());
    }

    #[test]
    fn part_2_works() {
        let moves = parse_moves(TEST_MOVES).unwrap();
        let mut rope = Rope::new(10);
        rope.follow_moves(&moves);

        assert_eq!(1, rope.tail_history.len());
    }
}
