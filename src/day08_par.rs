pub fn run(input: &str) -> anyhow::Result<(usize, u32)> {
    let forest = Forest::from_input(input, 99, 99);

    let stats = classify_forest(&forest);

    Ok((
        stats.iter().filter(|stat| stat.visible).count(),
        stats.iter().map(|stat| stat.score).max().unwrap(),
    ))
}

struct Forest {
    trees: Vec<u8>,
    rows: usize,
    cols: usize,
}

impl Forest {
    fn from_input(input: &str, rows: usize, cols: usize) -> Self {
        let mut trees = vec![0; rows * cols];

        for (y, row) in input.as_bytes().chunks(cols + 1).enumerate() {
            for (x, height) in row.iter().map(|b| b - b'0').enumerate().take(cols) {
                trees[y * cols + x] = height;
            }
        }

        Self { trees, rows, cols }
    }

    fn row(&self, y: usize) -> impl DoubleEndedIterator<Item = &u8> {
        self.trees.iter().skip(y * self.cols).take(self.cols)
    }

    fn col(&self, x: usize) -> impl DoubleEndedIterator<Item = &u8> {
        self.trees.iter().skip(x).step_by(self.cols).take(self.rows)
    }
}

#[derive(Copy, Clone)]
struct Stat {
    score: u32,
    visible: bool,
}

impl Stat {
    fn combine(self, other: Self) -> Self {
        Self {
            score: self.score * other.score,
            visible: self.visible | other.visible,
        }
    }
}

fn classify_forest(forest: &Forest) -> Vec<Stat> {
    let ((col_down, col_up), (row_left, row_right)) = rayon::join(
        || rayon::join(|| col_down_stats(forest), || col_up_stats(forest)),
        || rayon::join(|| row_left_stats(forest), || row_right_stats(forest)),
    );

    col_down
        .into_iter()
        .zip(col_up.into_iter())
        .zip(row_left.into_iter())
        .zip(row_right.into_iter())
        .map(|(((a, b), c), d)| a.combine(b).combine(c).combine(d))
        .collect()
}

#[inline]
fn stats(rows: usize, cols: usize) -> Vec<Stat> {
    vec![
        Stat {
            score: 0,
            visible: false
        };
        rows * cols
    ]
}

fn row_right_stats(forest: &Forest) -> Vec<Stat> {
    let mut stats = stats(forest.rows, forest.cols);
    let mut tree_line = TreeLine::new();
    for y in 0..forest.rows {
        tree_line.reset();
        for (i, height) in forest.row(y).enumerate() {
            stats[y * forest.cols + i] = classify_tree(i, *height, &mut tree_line);
        }
    }

    stats
}

fn row_left_stats(forest: &Forest) -> Vec<Stat> {
    let mut stats = stats(forest.rows, forest.cols);
    let mut tree_line = TreeLine::new();
    for y in 0..forest.rows {
        tree_line.reset();
        for (i, height) in forest.row(y).rev().enumerate() {
            stats[y * forest.cols + forest.cols - i - 1] =
                classify_tree(i, *height, &mut tree_line);
        }
    }
    stats
}

fn col_down_stats(forest: &Forest) -> Vec<Stat> {
    let mut stats = stats(forest.rows, forest.cols);
    let mut tree_line = TreeLine::new();
    for x in 0..forest.cols {
        tree_line.reset();
        for (j, height) in forest.col(x).enumerate() {
            stats[x + j * forest.cols] = classify_tree(j, *height, &mut tree_line);
        }
    }
    stats
}

fn col_up_stats(forest: &Forest) -> Vec<Stat> {
    let mut stats = stats(forest.rows, forest.cols);
    let mut tree_line = TreeLine::new();
    for x in 0..forest.cols {
        tree_line.reset();
        for (j, height) in forest.col(x).rev().enumerate() {
            stats[x + forest.cols * (forest.rows - j - 1)] =
                classify_tree(j, *height, &mut tree_line);
        }
    }
    stats
}

fn classify_tree(pos: usize, height: u8, tree_line: &mut TreeLine) -> Stat {
    let furthest_visible = tree_line.furthest_visible((height, pos));
    // u8::MAX is the sentinel value for the edges of the forest, if this sentinel value is
    // the furthest visible tree then the current tree must be visible.
    let visible = furthest_visible.0 == u8::MAX;
    // The position of the tree - the position of the furthest visible tree tells us how
    // many trees are visible
    let score = (pos - furthest_visible.1) as u32;
    // Push the height and position (calculated as the distance from the start of the
    // current line) into the tree line
    tree_line.push((height, pos));

    Stat { visible, score }
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
    fn it_works() {
        let forest = Forest::from_input(TEST_INPUT, 5, 5);
        let stats = classify_forest(&forest);
        let visible = stats.iter().filter(|stat| stat.visible).count();
        let max = stats.iter().map(|stat| stat.score).max().unwrap();
        assert_eq!(21, visible);
        assert_eq!(8, max);
    }

    // Alt solution
    // fn combine_stats(
    //     rows: usize,
    //     cols: usize,
    //     row_right: RowRightStats,
    //     row_left: RowLeftStats,
    //     col_down: ColDownStats,
    //     col_up: ColUpStats,
    // ) -> Vec<Stat> {
    //     itertools::iproduct!(0..cols, 0..rows)
    //         .map(|(x, y)| {
    //             row_left.0[cols - x - 1 + y * cols]
    //                 .combine(row_right.0[x + y * cols])
    //                 .combine(col_down.0[x * rows + y])
    //                 .combine(col_up.0[x * rows + rows - y - 1])
    //         })
    //         .collect()
    // }
}
