//! Solution to [AoC 2022 Day 14](https://adventofcode.com/2022/day/14)

use std::collections::HashSet;

type Point = (isize, isize);
type Grid = HashSet<Point>;

const START: Point = (500, 0);

fn add_line(grid: &mut Grid, (x1, y1): Point, (x2, y2): Point) {
    if x1 == x2 {
        for y in y1.min(y2)..=y1.max(y2) {
            grid.insert((x1, y));
        }
    } else if y1 == y2 {
        for x in x1.min(x2)..=x1.max(x2) {
            grid.insert((x, y1));
        }
    }
}

fn advance_sand(grid: &Grid, (x, y): Point, floor: Option<isize>) -> Option<Point> {
    let moves = [(0, 1), (-1, 1), (1, 1)];
    for (dx, dy) in moves {
        let new_loc = (x + dx, y + dy);
        if !grid.contains(&new_loc) && new_loc.1 < floor.unwrap_or(isize::MAX) {
            return Some(new_loc);
        }
    }
    None
}

// This runs kinda slow
// TODO: Use an array to back the grid to skip hashing
// TODO: Try using the path of the last grain of sand to inform the path of the next one
fn part1(mut grid: Grid) -> isize {
    let bottom_row = grid.iter().max_by_key(|(_x, y)| y).unwrap().1;

    for count in 0.. {
        let mut sand = START;
        loop {
            let Some(new_loc) = advance_sand(&grid, sand, None) else {
                    grid.insert(sand);
                    break;
            };
            if new_loc.1 >= bottom_row {
                return count;
            }
            sand = new_loc;
        }
    }
    unreachable!();
}

fn part2(mut grid: Grid) -> isize {
    let floor = grid.iter().max_by_key(|(_x, y)| y).unwrap().1 + 2;

    for count in 0.. {
        let mut sand = START;
        loop {
            let Some(new_loc) = advance_sand(&grid, sand, Some(floor)) else {
                    if sand == START {
                        return count + 1;
                    }
                    grid.insert(sand);
                    break;
            };
            sand = new_loc;
        }
    }
    unreachable!();
}

fn parse_point(s: &str) -> Point {
    let (s1, s2) = s.split_once(',').unwrap();
    (s1.parse().unwrap(), s2.parse().unwrap())
}

fn parse_input(input: &str) -> Grid {
    let mut grid = Grid::new();
    for line in input.trim().lines() {
        let mut iter_points = line.split(" -> ").peekable();
        while let Some(p) = iter_points.next() {
            let Some(next) = iter_points.peek() else {
                break;
            };
            add_line(&mut grid, parse_point(p), parse_point(next));
        }
    }
    grid
}

fn main() {
    let input = include_str!("../inputs/day14.txt");
    let input = parse_input(input);

    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
    "#;

    #[test]
    fn given_part1_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part1(input), 24);
    }

    #[test]
    fn given_part2_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part2(input), 93);
    }
}
