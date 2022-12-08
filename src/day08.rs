pub fn run(input: &str) -> anyhow::Result<(usize, u32)> {
    let forest = Forest::<99, 99>::from_input(input);

    Ok((forest.visible(), forest.best_score()))
}

struct Forest<const X: usize, const Y: usize> {
    forest: [[Tree; X]; Y],
}

#[derive(Copy, Clone)]
struct Tree {
    height: u8,
    score: u32,
    visible: bool,
}

impl Default for Tree {
    fn default() -> Self {
        Self {
            height: 0,
            score: 1,
            visible: false,
        }
    }
}

impl<const X: usize, const Y: usize> Forest<X, Y> {
    #[allow(clippy::needless_range_loop)]
    fn from_input(input: &str) -> Self {
        let mut forest: [[Tree; X]; Y] = [[Default::default(); X]; Y];

        for (y, row) in input.as_bytes().chunks(X + 1).enumerate() {
            for (x, height) in row.iter().map(|b| b - b'0').enumerate().take(X) {
                forest[y][x].height = height;
            }
        }

        let process_tree = |pos: usize, tree: &mut Tree, tree_line: &mut TreeLine| {
            let furthest_visible = tree_line.furthest_visible((tree.height, pos));
            // u8::MAX is the sentinel value for the edges of the forest, if this sentinel value is
            // the furthest visible tree then the current tree must be visible.
            if furthest_visible.0 == u8::MAX {
                tree.visible = true;
            }
            // The position of the tree - the position of the furthest visible tree tells us how
            // many trees are visible
            tree.score *= (pos - furthest_visible.1) as u32;
            // Push the height and position (calculated as the distance from the start of the
            // current line) into the tree line
            tree_line.push((tree.height, pos));
        };

        // Allocate a single stack and reset it for each line to avoid repeated allocations
        let mut tree_line = TreeLine::new();

        for y in 0..Y {
            tree_line.reset();
            for x in 0..X {
                let tree = &mut forest[y][x];
                process_tree(x, tree, &mut tree_line);
            }

            tree_line.reset();
            for (i, x) in (0..X).rev().enumerate() {
                let tree = &mut forest[y][x];
                process_tree(i, tree, &mut tree_line);
            }
        }

        for x in 0..X {
            tree_line.reset();
            for y in 0..Y {
                let tree = &mut forest[y][x];
                process_tree(y, tree, &mut tree_line);
            }

            tree_line.reset();
            for (i, y) in (0..Y).rev().enumerate() {
                let tree = &mut forest[y][x];
                process_tree(i, tree, &mut tree_line);
            }
        }

        Self { forest }
    }

    fn visible(&self) -> usize {
        self.forest
            .iter()
            .map(|row| row.iter().filter(|tree| tree.visible).count())
            .sum()
    }

    fn best_score(&self) -> u32 {
        self.forest
            .iter()
            .map(|row| row.iter().map(|tree| tree.score).max().unwrap())
            .max()
            .unwrap()
    }
}

/// A stack of trees' heights and their distance from the start of the line of trees we are
/// currently examining
struct TreeLine {
    stack: Vec<(u8, usize)>,
}

impl TreeLine {
    fn new() -> Self {
        let mut stack = Self { stack: Vec::new() };
        stack.reset();
        stack
    }

    fn reset(&mut self) {
        self.stack.clear();
        self.stack.push((u8::MAX, 0))
    }

    /// Returns the height and position of the furthest visible tree from the current tree in the
    /// line. The elves can't see the tops of trees that are taller than the current tree so a tree
    /// that is >= the height of the current tree block the view of an even taller tree behind it).
    fn furthest_visible(&mut self, tree: (u8, usize)) -> (u8, usize) {
        // Safe to unwrap as the sentinel value will always be in the stack
        while self.stack.last().unwrap().0 < tree.0 {
            self.stack.pop();
        }

        *self.stack.last().unwrap()
    }

    fn push(&mut self, tree: (u8, usize)) {
        self.stack.push(tree);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "\
30373
25512
65332
33549
35390";

    #[test]
    fn part_1_works() {
        let forest = Forest::<5, 5>::from_input(TEST_INPUT);
        assert_eq!(21, forest.visible());
    }

    #[test]
    fn part_2_works() {
        let forest = Forest::<5, 5>::from_input(TEST_INPUT);
        assert_eq!(8, forest.best_score());
    }
}
