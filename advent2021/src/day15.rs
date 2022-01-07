//! Solution to [AoC 2021 Day 15](https://adventofcode.com/2021/day/15)

use std::collections::{HashMap, VecDeque};

type Grid = Vec<Vec<Option<u32>>>;
type Point = (usize, usize);
const ADJACENT: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

const fn offset(base: usize, offset: isize) -> usize {
    (base as isize + offset) as usize
}

/// Return all points adjacent to a given point
fn neighbors(base: Point) -> impl Iterator<Item = Point> {
    ADJACENT
        .iter()
        .map(move |(dx, dy)| (offset(base.0, *dx), offset(base.1, *dy)))
}

fn get_next(
    grid: &Grid,
    base: Point,
    initial_risk: u32,
) -> impl Iterator<Item = (Point, u32)> + '_ {
    neighbors(base).filter_map(move |(x, y)| grid[y][x].map(|value| ((x, y), initial_risk + value)))
}

fn solve(grid: &Grid) -> u32 {
    let mut cache: HashMap<Point, u32> = HashMap::new();
    let start: Point = (1, 1);
    // BFS with memoization
    let mut to_check: VecDeque<(Point, u32)> = get_next(grid, start, 0).collect();
    while let Some((next_point, risk)) = to_check.pop_front() {
        // Point is in cache
        if let Some(min_risk) = cache.get_mut(&next_point) {
            // Found a more efficient way to get to this point. Update cache.
            if *min_risk > risk {
                *min_risk = risk;
                to_check.extend(get_next(grid, next_point, risk));
            }
        } else {
            // Point not in cache
            cache.insert(next_point, risk);
            to_check.extend(get_next(grid, next_point, risk));
        }
    }

    cache[&(grid.len() - 2, grid[0].len() - 2)]
}

fn part1(input: &str) -> u32 {
    // Parse input into a NxN array including a border of `None` to make boundary checks easier
    let len = input.trim().find('\n').unwrap() + 2;
    let mut grid = vec![vec![None; len]];
    for line in utils::trim_and_split(input, "\n") {
        let mut row = vec![None];
        row.extend(line.chars().map(|i| Some(i.to_digit(10).unwrap())));
        row.push(None);
        grid.push(row);
    }
    grid.push(vec![None; len]);
    solve(&grid)
}

fn wrapping_add(base: u32, addend: u32) -> u32 {
    let sum = base + addend;
    if sum > 9 {
        sum - 9
    } else {
        sum
    }
}

fn part2(input: &str) -> u32 {
    // Parse input into an NxN array of u32 for ease of later calculatios
    let dim = input.trim().find('\n').unwrap();
    let mut grid: Vec<Vec<u32>> = Vec::with_capacity(dim * dim);
    for line in utils::trim_and_split(input, "\n") {
        let row: Vec<u32> = line.chars().map(|i| i.to_digit(10).unwrap()).collect();
        grid.push(row);
    }
    let grid = grid;

    // Create expanded grid with a border of `None` to make boundary checks easier
    let final_dim = 5 * dim + 2;
    let mut expanded_grid = vec![vec![None; final_dim]];
    for row in grid {
        let mut expanded_row = vec![None];
        for increase in 0..5 {
            expanded_row.extend(row.iter().map(|i| Some(wrapping_add(*i, increase))));
        }
        expanded_row.push(None);
        expanded_grid.push(expanded_row);
    }
    for increase in 1..=4 {
        for row_idx in 1..=dim {
            let new_row: Vec<Option<u32>> = expanded_grid[row_idx]
                .iter()
                .map(|i| i.map(|i| wrapping_add(i, increase)))
                .collect();
            expanded_grid.push(new_row);
        }
    }
    expanded_grid.push(vec![None; final_dim]);
    solve(&expanded_grid)
}

fn main() {
    let input = include_str!("../inputs/day15.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
    "#;

    #[test]
    fn given_part1_input() {
        assert_eq!(40, part1(TEST_INPUT));
    }

    #[test]
    fn given_part2_input() {
        assert_eq!(315, part2(TEST_INPUT));
    }
}
