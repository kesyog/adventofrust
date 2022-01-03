//! Solution to [AoC 2021 Day 25](https://adventofcode.com/2021/day/25)

use ndarray::{Array2, Dimension, IntoDimension};

type Grid = Array2<Option<Heading>>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Heading {
    East,
    South,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StepResult {
    Changed(Grid),
    Unchanged,
}

/// Display a grid for debugging purposes
fn _display(grid: &Grid) -> String {
    let mut output = String::with_capacity(grid.dim().into_dimension().size() + grid.nrows());
    for row in grid.rows() {
        output.extend(row.map(|&i| match i {
            Some(Heading::East) => '>',
            Some(Heading::South) => 'v',
            None => '.',
        }));
        output.push('\n');
    }
    output
}

fn tick_east(input: &Grid) -> Grid {
    let mut new = input.clone();
    for ((row, col), _) in input
        .indexed_iter()
        .filter(|&(_, &i)| i == Some(Heading::East))
    {
        let new_col = if col == input.ncols() - 1 { 0 } else { col + 1 };
        if input[(row, new_col)].is_none() {
            new.swap((row, new_col), (row, col));
        }
    }
    new
}

fn tick_south(input: &Grid) -> Grid {
    let mut new = input.clone();
    for ((row, col), _) in input
        .indexed_iter()
        .filter(|&(_, &i)| i == Some(Heading::South))
    {
        let new_row = if row == input.nrows() - 1 { 0 } else { row + 1 };
        if input[(new_row, col)].is_none() {
            new.swap((new_row, col), (row, col));
        }
    }
    new
}

fn tick(input: &Grid) -> StepResult {
    let intermediate = tick_east(input);
    let new = tick_south(&intermediate);
    if new == input {
        StepResult::Unchanged
    } else {
        StepResult::Changed(new)
    }
}

fn part1(mut grid: Grid) -> usize {
    for steps in 1.. {
        match tick(&grid) {
            StepResult::Changed(new_grid) => grid = new_grid,
            StepResult::Unchanged => return steps,
        }
    }
    unreachable!();
}

fn _part2() {
    // Part 2 is solving the rest of the puzzles
}

fn parse_input(input: &str) -> Grid {
    assert!(input.is_ascii());
    let input = input.trim();
    let cols = input.find('\n').unwrap();
    // Each line has a newline except the last one
    let rows = (input.len() + 1) / (cols + 1);
    // Assume square grid
    let mut grid = Array2::default((rows, cols));
    for (line, row) in input.lines().zip(grid.rows_mut()) {
        for (c_in, c_out) in line.chars().zip(row) {
            *c_out = match c_in {
                '>' => Some(Heading::East),
                'v' => Some(Heading::South),
                '.' => None,
                other => panic!("Unexpected character: {}", other),
            };
        }
    }
    grid
}

fn main() {
    let input = include_str!("../inputs/day25.txt");
    let input = parse_input(input);

    println!("Part 1: {}", part1(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>
    "#;

    #[test]
    fn given_part1_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part1(input), 58);
    }
}
