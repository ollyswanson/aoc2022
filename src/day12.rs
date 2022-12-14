use std::collections::VecDeque;

pub fn run(input: &str) -> anyhow::Result<(usize, usize)> {
    let elevation_map = ElevationMap::construct(input, 80, 41);
    let shortest_path = ShortestPath::new(&elevation_map);

    let part_1 = shortest_path.shortest_path(
        elevation_map.start,
        |c, n| c + 1 >= n,
        move |pos, _| pos == elevation_map.end,
    );

    let part_2 = shortest_path.shortest_path(
        elevation_map.end,
        // Use of sentinel values has made this unnecessarily complicated 🤷
        |c, n| n != u8::MAX && n >= c - 1,
        |_, height| height == b'a',
    );

    Ok((part_1, part_2))
}

struct ShortestPath<'a> {
    elevation_map: &'a ElevationMap,
}

impl<'a> ShortestPath<'a> {
    fn new(elevation_map: &'a ElevationMap) -> Self {
        Self { elevation_map }
    }

    /// Returns the length of the shortest path.
    fn shortest_path<T, E>(&self, start: usize, can_traverse: T, end: E) -> usize
    where
        T: Fn(u8, u8) -> bool,
        E: Fn(usize, u8) -> bool,
    {
        let mut visited = vec![false; self.elevation_map.elevations.len()];
        let mut path: Vec<usize> = (0..visited.len()).collect();
        let mut final_pos: usize = 0;
        let mut queue: VecDeque<usize> = VecDeque::new();

        queue.push_back(start);

        'outer: while let Some(pos) = queue.pop_front() {
            if visited[pos] {
                continue;
            }
            visited[pos] = true;

            let current_elevation = self.elevation_map.elevations[pos];
            let (adj_positions, elevations) = self.elevation_map.adjacent_squares(pos);

            for i in 0..NUM_ADJACENT {
                let next_elevation = elevations[i];
                let next_pos = adj_positions[i];

                if visited[next_pos] || !can_traverse(current_elevation, next_elevation) {
                    continue;
                }

                if end(next_pos, next_elevation) {
                    path[next_pos] = pos;
                    final_pos = next_pos;
                    break 'outer;
                }

                queue.push_back(next_pos);
                path[next_pos] = pos;
            }
        }

        let mut previous = final_pos;
        let mut steps = 0;

        while path[previous] != previous {
            previous = path[previous];
            steps += 1;
        }

        steps
    }
}

struct ElevationMap {
    width_adj: usize,
    elevations: Vec<u8>,
    start: usize,
    end: usize,
}

const NUM_ADJACENT: usize = 4;

impl ElevationMap {
    fn construct(input: &str, width: usize, height: usize) -> Self {
        // Add 2 to the width and height for sentinel values
        let width_adj = width + 2;
        let height_adj = height + 2;
        // Populate elevations with sentinel values
        let mut elevations = vec![u8::MAX; width_adj * height_adj];
        let mut start = 0;
        let mut end = 0;

        for (y, row) in input.lines().enumerate().map(|(y, row)| (y + 1, row)) {
            for (x, elevation) in row
                .bytes()
                .enumerate()
                .map(|(x, elevation)| (x + 1, elevation))
            {
                let pos = y * width_adj + x;
                let elevation = match elevation {
                    b'S' => {
                        start = pos;
                        b'a'
                    }
                    b'E' => {
                        end = pos;
                        b'z'
                    }
                    b => b,
                };

                elevations[pos] = elevation;
            }
        }

        Self {
            elevations,
            width_adj,
            start,
            end,
        }
    }

    fn adjacent_squares(&self, position: usize) -> ([usize; NUM_ADJACENT], [u8; NUM_ADJACENT]) {
        let positions = [
            position - self.width_adj,
            position - 1,
            position + self.width_adj,
            position + 1,
        ];
        let elevations = [
            self.elevations[positions[0]],
            self.elevations[positions[1]],
            self.elevations[positions[2]],
            self.elevations[positions[3]],
        ];

        (positions, elevations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_MAP: &str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn part_1() {
        let elevation_map = ElevationMap::construct(TEST_MAP, 8, 5);
        let shortest_path = ShortestPath::new(&elevation_map);

        let part_1 = shortest_path.shortest_path(
            elevation_map.start,
            |c, n| c + 1 >= n,
            move |pos, _| pos == elevation_map.end,
        );

        assert_eq!(31, part_1);
    }

    #[test]
    fn part_2() {
        let elevation_map = ElevationMap::construct(TEST_MAP, 8, 5);
        let shortest_path = ShortestPath::new(&elevation_map);

        let part_2 = shortest_path.shortest_path(
            elevation_map.end,
            |c, n| n != u8::MAX && n >= c - 1,
            |_, height| height == b'a',
        );

        assert_eq!(29, part_2);
    }
}
