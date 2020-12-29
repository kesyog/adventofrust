//! Solution to [AoC 2018 Day 11](https://adventofcode.com/2018/day/11)

use itertools::Itertools;
use ndarray::{prelude::*, s};
use std::convert::{From, TryInto};

type GridSerial = u16;
type Grid = Array2<i32>;

fn hundredths_place(n: usize) -> u8 {
    ((n / 100) % 10).try_into().unwrap()
}

fn cell_power(x: usize, y: usize, serial: GridSerial) -> i32 {
    let rack_id = x + 10;
    let power: usize = (rack_id * y + usize::from(serial)) * rack_id;
    let power = i32::from(hundredths_place(power)) - 5;
    power
}

/// Create 300x300 grid of power values
///
/// Note: [Grid] is backed by an [ndarray::ArrayBase], which uses row, column indexing. It should
/// be indexed as y, x given the problem definition, but all the computations on the grid are
/// symmetric so it doesn't matter significantly in practice as long as we are consistent.
// Note: ndarray uses row, column indexing so this is swapped, but since we are only searching
// over square grids, it doesn't matter
fn create_grid(serial: GridSerial) -> Grid {
    let mut grid = Grid::zeros((300, 300));
    for (x, y) in (0..300).into_iter().cartesian_product(0..300) {
        grid[[x, y]] = cell_power(x + 1, y + 1, serial);
    }
    grid
}

/// Brute force search all 3x3 grids
fn part1(grid: &Grid) -> (usize, usize) {
    let mut max = (i32::MIN, (0, 0));
    for (x, y) in (0..297).into_iter().cartesian_product(0..297) {
        let sum = grid.slice(s![x..x + 3, y..y + 3]).sum();
        if sum > max.0 {
            max = (sum, (x + 1, y + 1));
        }
    }
    max.1
}

/// Brute force search all square grids 🐌
fn part2(grid: &Grid) -> (usize, usize, usize) {
    let mut max = (i32::MIN, (0, 0, 0));
    for (x, y) in (0..300).into_iter().cartesian_product(0..300) {
        // Initialize with 1x1 grid
        let mut sum = grid[[x, y]];
        if sum > max.0 {
            max = (sum, (x + 1, y + 1, 1));
        }
        for size in 2..=std::cmp::min(300 - x, 300 - y) {
            // Optimization: rather than recalculating the sum over the whole grid at every size
            // iteration, only calculate the sum of the incremental squares needed to expand the
            // square by 1 unit.
            let delta = grid.slice(s![x + size - 1, y..y + size]).sum()
                + grid.slice(s![x..x + size - 1, y + size - 1]).sum();
            sum += delta;
            if delta > 0 && sum > max.0 {
                max = (sum, (x + 1, y + 1, size));
            }
        }
    }
    max.1
}

fn main() {
    let grid_serial: GridSerial = 1788;
    let grid = create_grid(grid_serial);
    println!("Part 1: {:?}", part1(&grid));
    println!("Part 2: {:?}", part2(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hundredths_place() {
        assert_eq!(3, hundredths_place(12345));
        assert_eq!(0, hundredths_place(99));
        assert_eq!(0, hundredths_place(0));
        assert_eq!(1, hundredths_place(100));
        assert_eq!(1, hundredths_place(199));
        assert_eq!(2, hundredths_place(200));
        assert_eq!(9, hundredths_place(999));
        assert_eq!(0, hundredths_place(1000));
        assert_eq!(0, hundredths_place(1001));
        assert_eq!(1, hundredths_place(1101));
    }

    #[test]
    fn test_power_level() {
        assert_eq!(4, cell_power(3, 5, 8));
        assert_eq!(-5, cell_power(122, 79, 57));
        assert_eq!(0, cell_power(217, 196, 39));
        assert_eq!(4, cell_power(101, 153, 71));
    }

    fn test_part1(serial: GridSerial) -> (usize, usize) {
        part1(&create_grid(serial))
    }

    #[test]
    fn given_part1_input() {
        assert_eq!((33, 45), test_part1(18));
        assert_eq!((21, 61), test_part1(42));
    }

    fn test_part2(serial: GridSerial) -> (usize, usize, usize) {
        part2(&create_grid(serial))
    }

    #[test]
    fn given_part2_input() {
        assert_eq!((90, 269, 16), test_part2(18));
        assert_eq!((232, 251, 12), test_part2(42));
    }
}
