//! Solution to [AoC 2021 Day 11](https://adventofcode.com/2021/day/11)

type Point = (usize, usize);

const ADJACENT: [(isize, isize); 8] = [
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
    (-1, 0),
    (0, -1),
    (0, 1),
    (1, 0),
];

const fn offset(base: usize, offset: isize) -> usize {
    (base as isize + offset) as usize
}

/// Return all points adjacent to a given point
fn neighbors(base: &Point) -> impl Iterator<Item = Point> + '_ {
    ADJACENT
        .iter()
        .map(move |(dx, dy)| (offset(base.0, *dx), offset(base.1, *dy)))
}

fn flash(grid: &mut [Vec<Option<u32>>], point: Point) -> usize {
    let mut n_flashes = 0;
    for neighbor in neighbors(&point) {
        if let Some(value) = grid[neighbor.0][neighbor.1].as_mut() {
            *value += 1;
            if *value == 10 {
                n_flashes += 1 + flash(grid, neighbor);
            }
        }
    }
    n_flashes
}

fn tick(grid: &mut [Vec<Option<u32>>]) -> usize {
    let mut n_flashes = 0;
    let width = grid.len();
    for row in 1..(width - 1) {
        for col in 1..(width - 1) {
            if let Some(value) = grid[row][col].as_mut() {
                *value += 1;
                if *value == 10 {
                    n_flashes += 1 + flash(grid, (row, col));
                }
            }
        }
    }

    for row in grid.iter_mut().take(width - 1).skip(1) {
        for cell in row.iter_mut().take(width - 1).skip(1) {
            match cell.as_mut() {
                Some(value) if *value > 9 => *value = 0,
                _ => (),
            };
        }
    }
    n_flashes
}

fn part1(grid: &[Vec<Option<u32>>]) -> usize {
    let mut grid = grid.to_owned();
    let mut n_flashes = 0;
    for _ in 0..100 {
        n_flashes += tick(&mut grid);
    }
    n_flashes
}

fn part2(grid: &[Vec<Option<u32>>]) -> usize {
    let mut grid = grid.to_owned();
    for i in 1.. {
        if tick(&mut grid) == 100 {
            return i;
        }
    }
    unreachable!();
}

/// Parse input into a NxN array with a border of `None` to make boundary checks easier
fn parse_input(input: &str) -> Vec<Vec<Option<u32>>> {
    let len = input.trim().find('\n').unwrap() + 2;
    let mut grid = vec![vec![None; len]];
    for line in utils::trim_and_split(input, "\n") {
        let mut row = vec![None];
        row.extend(line.chars().map(|i| Some(i.to_digit(10).unwrap())));
        row.push(None);
        grid.push(row);
    }
    grid.push(vec![None; len]);
    grid
}

fn main() {
    let input = include_str!("../inputs/day11.txt");
    let input = parse_input(input);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
    "#;

    #[test]
    fn given_part1_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part1(&input), 1656);
    }

    #[test]
    fn given_part2_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part2(&input), 195);
    }
}
