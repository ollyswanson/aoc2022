use itertools::{Itertools, MinMaxResult};
use rustc_hash::{FxHashMap, FxHashSet};

pub fn run(input: &str) -> anyhow::Result<(usize, usize)> {
    let mut grid = Elves::build_grid(input);

    let part_1 = grid.simulate(10);
    let now = std::time::Instant::now();
    let part_2 = grid.simulate_until_stopped();
    dbg!(now.elapsed());

    Ok((part_1, part_2))
}

struct Elves {
    grid: FxHashSet<(i32, i32)>,
    round: usize,
}

struct Elf {
    location: (i32, i32),
    desired: (i32, i32),
}

impl Elves {
    fn simulate(&mut self, rounds: usize) -> usize {
        let mut moves_buffer = Vec::with_capacity(self.grid.len());
        let mut location_count = FxHashMap::default();

        for _ in 0..rounds {
            self.move_elves(&mut moves_buffer, &mut location_count);
        }

        self.count_empty()
    }

    fn simulate_until_stopped(&mut self) -> usize {
        let mut moves_buffer = Vec::with_capacity(self.grid.len());
        let mut location_count = FxHashMap::default();

        loop {
            let moved = self.move_elves(&mut moves_buffer, &mut location_count);

            if !moved {
                return self.round;
            }
        }
    }

    fn move_elves(
        &mut self,
        moves_buffer: &mut Vec<Elf>,
        location_count: &mut FxHashMap<(i32, i32), u32>,
    ) -> bool {
        moves_buffer.clear();
        location_count.clear();

        let find_desired = |current: (i32, i32), this: &Self| -> (i32, i32) {
            let mut desired = current;
            let mut no_neighbours = true;

            for i in (this.round..self.round + 4).map(|i| i % 4) {
                let consideration = CONSIDERATIONS[i];

                if consideration.iter().all(|&delta| {
                    !this
                        .grid
                        .contains(&(current.0 + delta.0, current.1 + delta.1))
                }) {
                    if desired == current {
                        desired = (
                            current.0 + consideration[2].0,
                            current.1 + consideration[2].1,
                        );
                    }
                } else {
                    no_neighbours = false;
                }
            }

            if no_neighbours {
                current
            } else {
                desired
            }
        };

        for &location in self.grid.iter() {
            let desired = find_desired(location, self);
            moves_buffer.push(Elf { location, desired });
            let count = location_count.entry(desired).or_default();
            *count += 1;
        }

        self.grid.clear();

        let mut moved = false;

        for elf in moves_buffer {
            if location_count[&elf.desired] == 1 {
                self.grid.insert(elf.desired);

                if elf.location != elf.desired {
                    moved = true;
                }
            } else {
                self.grid.insert(elf.location);
            }
        }

        self.round += 1;

        moved
    }

    fn build_grid(input: &str) -> Self {
        let grid: FxHashSet<(i32, i32)> = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.bytes().enumerate().filter_map(move |(x, b)| match b {
                    b'#' => Some((x as i32, 0 - y as i32)),
                    _ => None,
                })
            })
            .collect();

        Self { grid, round: 0 }
    }

    fn count_empty(&self) -> usize {
        let width = match self.grid.iter().map(|location| location.0).minmax() {
            MinMaxResult::MinMax(min, max) => max - min + 1,
            _ => 1,
        };

        let height = match self.grid.iter().map(|location| location.1).minmax() {
            MinMaxResult::MinMax(min, max) => max - min + 1,
            _ => 1,
        };

        let area = (width * height) as usize;

        area - self.grid.len()
    }
}

// The last check for each direction is where the elf will want to move
const CONSIDERATIONS: [[(i32, i32); 3]; 4] = [
    [(-1, 1), (1, 1), (0, 1)],    // North
    [(-1, -1), (1, -1), (0, -1)], // South
    [(-1, 1), (-1, -1), (-1, 0)], // West
    [(1, 1), (1, -1), (1, 0)],    // East
];

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ELVES: &str = include_str!("../inputs/day23_test.txt");

    #[test]
    fn it_works() {
        let (part_1, part_2) = run(TEST_ELVES).unwrap();

        assert_eq!(110, part_1);
        assert_eq!(20, part_2);
    }
}
