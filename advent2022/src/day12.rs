//! Solution to [AoC 2022 Day 12](https://adventofcode.com/2022/day/12)

use std::collections::VecDeque;

const MOVES: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, -1), (0, 1)];

#[derive(Debug)]
struct Grid {
    inner: Vec<char>,
    n_rows: usize,
    n_cols: usize,
    start: usize,
    finish: usize,
}

impl Grid {
    fn coords(&self, index: usize) -> (usize, usize) {
        let row = index / self.n_cols;
        let col = index % self.n_cols;
        (row, col)
    }

    fn index_from_coords(&self, (row, col): (usize, usize)) -> usize {
        row * self.n_cols + col
    }

    fn value(&self, index: usize) -> usize {
        let c = self.inner[index];
        match c {
            'S' => 0,
            'E' => 26,
            'a'..='z' => c as usize - 'a' as usize,
            _ => panic!("invalid char: {c}"),
        }
    }
}

/// Generate a list of cells that we can go to from the current cell
fn visitors(grid: &Grid, current: usize) -> impl Iterator<Item = usize> + '_ {
    let (row, col) = grid.coords(current);

    MOVES.iter().filter_map(move |(drow, dcol)| {
        let new_row = row as isize + drow;
        let new_col = col as isize + dcol;
        if new_row < 0
            || new_row as usize >= grid.n_rows
            || new_col < 0
            || new_col as usize >= grid.n_cols
        {
            // Coordinates are off the grid
            return None;
        }
        let new_index = grid.index_from_coords((new_row as usize, new_col as usize));
        if (grid.value(current) as isize) - (grid.value(new_index) as isize) > 1 {
            // Too big of a step
            return None;
        }
        Some(new_index)
    })
}

/// Helper function to solve part1 or part2
///
/// `is_valid_start` takes a 1-d index into the grid as an argument and returns whether we've
/// reached a valid start
fn helper<F>(grid: &Grid, mut is_valid_start: F) -> usize
where
    F: FnMut(usize) -> bool,
{
    // BFS + dynamic programming solution
    // Work backwards from the end since there is only ever one possible end. In part 2, there are
    // multiple possible starts.
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut best: Vec<Option<usize>> = vec![None; grid.inner.len()];
    best[grid.finish] = Some(0);
    queue.extend(
        visitors(grid, grid.finish)
            .into_iter()
            .zip(std::iter::repeat(0)),
    );

    while let Some((next, prev_len)) = queue.pop_front() {
        if best[next].is_some() {
            // Already found an optimal path to this cell
            continue;
        }
        best[next] = Some(prev_len + 1);
        if is_valid_start(next) {
            return best[next].unwrap();
        }

        queue.extend(
            visitors(grid, next)
                .into_iter()
                .filter(|i| best[*i].is_none())
                .map(|i| (i, prev_len + 1)),
        );
    }

    panic!("Couldn't find a path");
}

fn part1(grid: &Grid) -> usize {
    helper(grid, |idx| idx == grid.start)
}

fn part2(grid: &Grid) -> usize {
    helper(grid, |idx| grid.value(idx) == 0)
}

fn parse_input(input: &str) -> Grid {
    let input = input.trim();
    let n_cols = input.chars().position(|c| c == '\n').unwrap();
    let inner: Vec<char> = input.chars().filter(char::is_ascii_alphanumeric).collect();
    let start = inner.iter().position(|&c| c == 'S').unwrap();
    let finish = inner.iter().position(|&c| c == 'E').unwrap();
    let n_rows = inner.len() / n_cols;

    Grid {
        inner,
        n_rows,
        n_cols,
        start,
        finish,
    }
}

fn main() {
    let input = include_str!("../inputs/day12.txt");
    let input = parse_input(input);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
    "#;

    #[test]
    fn given_part1_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part1(&input), 31);
    }

    #[test]
    fn given_part2_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part2(&input), 29);
    }
}
