use std::collections::BinaryHeap;
use std::collections::VecDeque;

use hashbrown::HashSet;

pub fn run(input: &str) -> anyhow::Result<(i32, i32)> {
    let valley = Valley::build_valley(input, 100, 35);

    let part_1 = traverse_valley(&valley, (0, 0), 0, (valley.width - 1, valley.height)).unwrap();

    let part_2 = traverse_valley(
        &valley,
        (valley.width - 1, valley.height - 1),
        part_1,
        (0, -1),
    )
    .unwrap();

    let part_2 =
        traverse_valley(&valley, (0, 0), part_2, (valley.width - 1, valley.height)).unwrap();

    Ok((part_1, part_2))
}

fn traverse_valley(valley: &Valley, start: (i32, i32), time: i32, end: (i32, i32)) -> Option<i32> {
    // How soon can we enter the valley?
    let mut t = time + 1;
    while valley.blizzards_at(start, t) > 0 {
        t += 1;
    }

    let mut visited: HashSet<((i32, i32), i32)> = HashSet::new();
    let mut queue: VecDeque<((i32, i32), i32)> = VecDeque::new();
    queue.push_back((start, t));

    while let Some((position, time)) = queue.pop_front() {
        if visited.contains(&(position, time)) {
            continue;
        }

        visited.insert((position, time));

        if position == end {
            return Some(time);
        }

        if position.0 < 0
            || position.0 >= valley.width
            || position.1 < 0
            || position.1 >= valley.height
        {
            continue;
        }

        if valley.blizzards_at(position, time) > 0 {
            continue;
        }

        for mov in MOVES {
            let new_position = (position.0 + mov.0, position.1 + mov.1);
            queue.push_back((new_position, time + 1));
        }
    }

    None
}

const MOVES: [(i32, i32); 5] = [(1, 0), (-1, 0), (0, 1), (0, -1), (0, 0)];

struct Valley {
    width: i32,
    height: i32,
    x_lines: Vec<Vec<Blizzard>>,
    y_lines: Vec<Vec<Blizzard>>,
}

impl Valley {
    fn blizzards_at(&self, location: (i32, i32), time: i32) -> usize {
        let count_y = self.x_lines[location.0 as usize]
            .iter()
            .map(|b| (b.initial + time * b.movement).rem_euclid(self.height))
            .filter(|&y| y == location.1)
            .count();

        let count_x = self.y_lines[location.1 as usize]
            .iter()
            .map(|b| (b.initial + time * b.movement).rem_euclid(self.width))
            .filter(|&x| x == location.0)
            .count();

        count_x + count_y
    }

    fn build_valley(input: &str, width: i32, height: i32) -> Self {
        let mut x_lines: Vec<Vec<Blizzard>> = vec![Vec::default(); width as usize];
        let mut y_lines: Vec<Vec<Blizzard>> = vec![Vec::default(); height as usize];

        for (y, line) in input.lines().enumerate() {
            for (x, b) in line.bytes().enumerate() {
                match b {
                    b'>' => {
                        y_lines[y - 1].push(Blizzard {
                            initial: x as i32 - 1,
                            movement: 1,
                        });
                    }
                    b'<' => {
                        y_lines[y - 1].push(Blizzard {
                            initial: x as i32 - 1,
                            movement: -1,
                        });
                    }
                    b'^' => {
                        x_lines[x - 1].push(Blizzard {
                            initial: y as i32 - 1,
                            movement: -1,
                        });
                    }
                    b'v' => {
                        x_lines[x - 1].push(Blizzard {
                            initial: y as i32 - 1,
                            movement: 1,
                        });
                    }
                    _ => {}
                }
            }
        }

        Self {
            width,
            height,
            x_lines,
            y_lines,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Blizzard {
    initial: i32,
    movement: i32,
}
