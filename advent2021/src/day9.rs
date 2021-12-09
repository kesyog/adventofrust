//! Solution to [AoC 2021 Day 9](https://adventofcode.com/2021/day/9)

use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

type Point = (usize, usize);

const ADJACENT: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

const fn offset(base: usize, offset: isize) -> usize {
    (base as isize + offset) as usize
}

/// Return all points adjacent to a given point
fn neighbors(base: &Point) -> impl Iterator<Item = Point> + '_ {
    ADJACENT
        .iter()
        .map(move |(dx, dy)| (offset(base.0, *dx), offset(base.1, *dy)))
}

fn part1(grid: &[Vec<u32>]) -> u32 {
    let n_rows = grid.len();
    let n_cols = grid[0].len();
    let mut low_points: Vec<u32> = Vec::new();
    for row in 1..n_rows - 1 {
        for col in 1..n_cols - 1 {
            let value = grid[row][col];
            if neighbors(&(row, col)).all(|(r, c)| value < grid[r][c]) {
                low_points.push(value);
            }
        }
    }
    let n_points = low_points.len() as u32;
    low_points.into_iter().sum::<u32>() + n_points
}

// Recusively build a basin. It's assumed that the points in the basin do not belong to another
// basin
fn build_basin(grid: &[Vec<u32>], (row, col): Point, cache: &mut HashSet<Point>) {
    if cache.contains(&(row, col)) {
        return;
    }
    cache.insert((row, col));
    for neighbor in neighbors(&(row, col)).filter(|(row, col)| grid[*row][*col] < 9) {
        build_basin(grid, neighbor, cache);
    }
}

fn part2(grid: &[Vec<u32>]) -> usize {
    let n_rows = grid.len();
    let n_cols = grid[0].len();
    // Map each point to a basin id
    let mut basin_map: HashMap<Point, usize> = HashMap::new();
    let mut basin_count = 0;
    for row in 1..n_rows - 1 {
        for col in 1..n_cols - 1 {
            if grid[row][col] == 9 || basin_map.contains_key(&(row, col)) {
                continue;
            }
            let mut basin = HashSet::new();
            build_basin(grid, (row, col), &mut basin);
            for point in basin.drain() {
                assert!(basin_map.insert(point, basin_count).is_none());
            }
            basin_count += 1;
        }
    }
    let mut basins: HashMap<usize, Vec<Point>> = HashMap::new();
    for (point, b) in basin_map.drain() {
        basins.entry(b).or_insert_with(Vec::new).push(point);
    }
    basins
        .values()
        .map(|i| Reverse(i.len()))
        .k_smallest(3)
        .map(|i| i.0)
        .product()
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let width = input.trim().find('\n').unwrap() + 2;
    let mut grid = vec![vec![u32::MAX; width]];
    grid.extend(utils::trim_and_split(input, "\n").map(|row| {
        let mut buffered_row = vec![u32::MAX];
        buffered_row.extend(row.chars().map(|c| c.to_digit(10).unwrap()));
        buffered_row.push(u32::MAX);
        buffered_row
    }));
    grid.push(vec![u32::MAX; width]);
    grid
}

fn main() {
    let input = include_str!("../inputs/day9.txt");
    let grid = parse_input(input);

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
2199943210
3987894921
9856789892
8767896789
9899965678
    "#;

    #[test]
    fn given_part1_input() {
        let grid = parse_input(TEST_INPUT);
        assert_eq!(part1(&grid), 15);
    }

    #[test]
    fn given_part2_input() {
        let grid = parse_input(TEST_INPUT);
        assert_eq!(part2(&grid), 1134);
    }
}
