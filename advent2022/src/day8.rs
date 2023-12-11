//! Solution to [AoC 2022 Day 8](https://adventofcode.com/2022/day/8)

use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Grid {
    inner: Vec<u8>,
    side_length: usize,
}

impl Grid {
    /// Transform 1-D index to 2-D index
    fn index_to_coords(&self, index: usize) -> (usize, usize) {
        (index / self.side_length, index % self.side_length)
    }

    /// Return an iterator over the trees in a given row, returning the 1-D index and height of
    /// each tree.
    fn iter_row(&self, row: usize) -> impl DoubleEndedIterator<Item = (usize, u8)> + '_ {
        self.inner
            .iter()
            .copied()
            .enumerate()
            .skip(row * self.side_length)
            .take(self.side_length)
    }

    /// Return an iterator over the trees in a given column, returning the 1-D index and height of
    /// each tree.
    fn iter_col(&self, col: usize) -> impl DoubleEndedIterator<Item = (usize, u8)> + '_ {
        self.inner
            .iter()
            .copied()
            .enumerate()
            .skip(col)
            .step_by(self.side_length)
            .take(self.side_length)
    }

    /// Find the scenic score for the tree at the given 1-D index
    fn scenic_score(&self, i: usize) -> usize {
        fn count_visible_trees(height: u8, iter: impl Iterator<Item = u8>) -> usize {
            let mut count = 0;
            for tree in iter {
                count += 1;
                if tree >= height {
                    break;
                }
            }
            count
        }

        let (row, col) = self.index_to_coords(i);
        // Trees on the edge will have a 0 score since they can see zero trees in one direction
        if row == 0 || col == 0 || row == self.side_length - 1 || col == self.side_length - 1 {
            return 0;
        }
        let height = self.inner[i];
        let mut score = 1;
        // Right
        score *= count_visible_trees(height, self.iter_row(row).skip(col + 1).map(|(_, i)| i));
        // Left
        score *= count_visible_trees(
            height,
            self.iter_row(row)
                .rev()
                .skip(self.side_length - col)
                .map(|(_, i)| i),
        );
        // Up
        score *= count_visible_trees(
            height,
            self.iter_col(col)
                .rev()
                .skip(self.side_length - row)
                .map(|(_, i)| i),
        );
        // Down
        score *= count_visible_trees(height, self.iter_col(col).skip(row + 1).map(|(_, i)| i));
        score
    }
}

impl FromIterator<u8> for Grid {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let inner: Vec<u8> = iter.into_iter().collect();
        let side_length = (inner.len() as f32).sqrt() as usize;
        assert_eq!(
            side_length * side_length,
            inner.len(),
            "Grid must be a square"
        );

        Self { inner, side_length }
    }
}

fn part1(grid: &Grid) -> usize {
    /// Find trees visible in a given slice, "looking" from the direction of the start of the slice
    fn find_visible<I>(items: I) -> impl Iterator<Item = (usize, u8)>
    where
        I: IntoIterator<Item = (usize, u8)>,
    {
        // Trees are visible if they are higher than any previously-seen tree
        let mut prev_max: Option<u8> = None;
        items
            .into_iter()
            .filter_map(move |(i, height)| match prev_max {
                Some(value) if value >= height => None,
                _ => {
                    prev_max = Some(height);
                    Some((i, height))
                }
            })
    }

    let mut visible: HashSet<(usize, u8)> = HashSet::new();
    for i in 0..grid.side_length {
        // Look right
        visible.extend(find_visible(grid.iter_row(i)));
        // Look left
        visible.extend(find_visible(grid.iter_row(i).rev()));
        // Look down
        visible.extend(find_visible(grid.iter_col(i)));
        // Look up
        visible.extend(find_visible(grid.iter_col(i).rev()));
    }
    visible.len()
}

fn part2(grid: &Grid) -> usize {
    (0..grid.inner.len())
        .map(|i| grid.scenic_score(i))
        .max()
        .unwrap()
}

fn parse_input(input: &str) -> Grid {
    input
        .trim()
        .chars()
        .filter(char::is_ascii_digit)
        .map(|c| c.to_digit(10).expect("a digit").try_into().expect("a u8"))
        .collect()
}

#[cfg(not(test))]
fn main() {
    let input = include_str!("../inputs/day8.txt");
    let input = parse_input(input);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
30373
25512
65332
33549
35390
    "#;

    #[test]
    fn given_part1_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part1(&input), 21);
    }

    #[test]
    fn given_part2_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part2(&input), 8);
    }
}
