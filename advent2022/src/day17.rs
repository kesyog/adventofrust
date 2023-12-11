//! Solution to [AoC 2022 Day 17](https://adventofcode.com/2022/day/17)

use std::collections::HashSet;
use std::fmt::Display;

use anyhow::{bail, Error};
use strum::EnumIter;

type Point = (i64, i64);

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn x_offset(self) -> i64 {
        match self {
            Direction::Left => -1,
            Direction::Right => 1,
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        let out = match c {
            '<' => Self::Left,
            '>' => Self::Right,
            _ => bail!("Invalid direction"),
        };
        Ok(out)
    }
}

#[derive(Clone, Debug, Default)]
struct Chamber {
    /// Walls are at x = -1 and x = Self::WIDTH
    /// Floor is at y = 0
    // Using a hashset to store all points in the chamber
    // Potential optimization:
    // Prune out any points that aren't in the group of points returned by
    // Self::exposed_points_normalized, all the columns, as they cannot affect any falling pieces.
    // But this would make things harder to debug and pretty-print
    settled: HashSet<Point>,
    /// Height of the highest settled rock
    max_height: i64,
}

impl Chamber {
    const WIDTH: i64 = 7;

    fn new() -> Self {
        Self::default()
    }

    /// Starting point of a new rock
    fn new_rock_origin(&self) -> Point {
        (2, self.max_height + 4)
    }

    /// Check if any of the given points overlap with the walls, floor, or settled rocks of the
    /// chamber
    fn conflicts(&self, points: impl IntoIterator<Item = Point>) -> bool {
        points.into_iter().any(|point| {
            point.0 < 0 || point.0 >= Self::WIDTH || point.1 <= 0 || self.settled.contains(&point)
        })
    }

    /// Add the given points to the chamber. Assumes that the points don't conflict.
    fn add(&mut self, points: impl IntoIterator<Item = Point>) {
        for point in points {
            self.max_height = self.max_height.max(point.1);
            self.settled.insert(point);
        }
    }

    /// Return the floor tiles
    fn floor() -> impl Iterator<Item = Point> {
        (0..Self::WIDTH).into_iter().map(|x| (x, 0))
    }

    /// Return the portion of the chamber that may potentially affect rocks
    /// The y values of the returned points are relative to the max height of the chamber
    fn exposed_points_normalized(&self) -> HashSet<Point> {
        let mut tops = Vec::new();
        // Find the top of each column in the chamber
        for column in 0..Self::WIDTH {
            tops.push(
                self.settled
                    .iter()
                    .copied()
                    .filter(|(x, _)| *x == column)
                    .max_by_key(|(_, y)| *y)
                    .unwrap_or((column, 0)),
            );
        }
        // Any settled rocks/floor tiles that are >= the lowest of these tops can affect a falling
        // rock. We can't just use the tops of each column since we need to account for the
        // possibility of jets blowing falling rocks into the side of columns of rock
        let lowest_top = tops.into_iter().min_by_key(|&(_, y)| y).unwrap();
        let mut out = HashSet::new();
        out.extend(
            self.settled
                .iter()
                .copied()
                .chain(Self::floor())
                .filter(|&(_, y)| y >= lowest_top.1)
                .map(|(x, y)| (x, self.max_height - y)),
        );
        out
    }
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut points = vec![vec![" "; Self::WIDTH as usize]; (self.max_height + 1) as usize];
        for &(x, y) in &self.settled {
            points[y as usize][x as usize] = "█";
        }
        for row in points.into_iter().rev() {
            writeln!(f, "{}", row.join(""))?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
enum Rock {
    Minus,
    Plus,
    L,
    I,
    Square,
}

impl Rock {
    fn generator() -> impl Iterator<Item = Self> {
        use Rock::*;
        [Minus, Plus, L, I, Square].into_iter().cycle()
    }

    /// Coordinates of all points relative to bottom left
    fn all_points(self) -> Vec<Point> {
        match self {
            Rock::Minus => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            Rock::Plus => vec![(0, 1), (1, 0), (1, 1), (2, 1), (1, 2)],
            Rock::L => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            Rock::I => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            Rock::Square => vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        }
    }

    /// Lowest points in each column
    fn bottom_points(self) -> Vec<Point> {
        match self {
            Rock::Minus => self.all_points(),
            Rock::Plus => vec![(0, 1), (1, 0), (2, 1)],
            Rock::L => vec![(0, 0), (1, 0), (2, 0)],
            Rock::I => vec![(0, 0)],
            Rock::Square => vec![(0, 0), (1, 0)],
        }
    }

    /// Rightmost points in each row
    fn right_points(self) -> Vec<Point> {
        match self {
            Rock::Minus => vec![(3, 0)],
            Rock::Plus => vec![(1, 0), (2, 1), (1, 2)],
            Rock::L => vec![(2, 0), (2, 1), (2, 2)],
            Rock::I => self.all_points(),
            Rock::Square => vec![(1, 0), (1, 1)],
        }
    }

    /// Leftmost points in each row
    fn left_points(self) -> Vec<Point> {
        match self {
            Rock::Minus => vec![(0, 0)],
            Rock::Plus => vec![(1, 0), (0, 1), (1, 2)],
            Rock::L => vec![(0, 0), (2, 1), (2, 2)],
            Rock::I => self.all_points(),
            Rock::Square => vec![(0, 0), (0, 1)],
        }
    }
}

fn side_boundary(rock: Rock, direction: Direction) -> Vec<Point> {
    match direction {
        Direction::Left => rock.left_points(),
        Direction::Right => rock.right_points(),
    }
}

fn shift(points: impl IntoIterator<Item = Point>, dx: i64, dy: i64) -> impl Iterator<Item = Point> {
    let mut iter = points.into_iter();
    std::iter::from_fn(move || iter.next().map(|(x, y)| (x + dx, y + dy)))
}

/// Simulate the Tetris-like scenario and return the height of the stack
// Nothing fancy. Only small insight is that we only need to use the relevant boundary tiles of the
// falling rock when checking for collisions.
fn simulate(directions: &[Direction], n_rocks: u64) -> i64 {
    let mut chamber = Chamber::new();
    let mut directions = directions.iter().copied().cycle();
    for rock in Rock::generator().take(n_rocks as usize) {
        let mut rock_origin = chamber.new_rock_origin();
        // Loop until rock is settled
        loop {
            let direction = directions.next().unwrap();
            // Jet step: shift left or right if possible
            if !chamber.conflicts(shift(
                side_boundary(rock, direction),
                rock_origin.0 + direction.x_offset(),
                rock_origin.1,
            )) {
                rock_origin.0 += direction.x_offset();
            }

            // Fall by one
            if chamber.conflicts(shift(
                rock.bottom_points(),
                rock_origin.0,
                rock_origin.1 - 1,
            )) {
                // Rock has settled
                break;
            }
            rock_origin.1 -= 1;
        }
        chamber.add(shift(rock.all_points(), rock_origin.0, rock_origin.1));
    }
    chamber.max_height
}

/// On some interval, the game will repeat itself. Find the (# of rocks, increase in height) between
/// cycles.
///
/// Approach:
/// 1. Simulate to n_rocks and save off the position of the exposed settled rocks
/// 2. Continue simulating until the exposed settled rocks matches what was seen at n_rocks
///
/// Assumptions:
/// * n_rocks is larger than the number of rocks between cycles
/// * n_rocks is large enough to cover up all floor tiles
// Mostly a copy of simulate()
fn find_cycle(directions: &[Direction], n_rocks: usize) -> (u64, u64) {
    let mut chamber = Chamber::new();
    let mut directions = directions.iter().copied().cycle();
    let mut saved_exposed_points = None;
    let mut saved_height = None;
    for (i, rock) in Rock::generator().take(2 * n_rocks).enumerate() {
        let mut rock_origin = chamber.new_rock_origin();
        // Loop until rock is settled
        loop {
            let direction = directions.next().unwrap();
            // Jet step
            if !chamber.conflicts(shift(
                side_boundary(rock, direction),
                rock_origin.0 + direction.x_offset(),
                rock_origin.1,
            )) {
                rock_origin.0 += direction.x_offset();
            }

            // Fall step
            if chamber.conflicts(shift(
                rock.bottom_points(),
                rock_origin.0,
                rock_origin.1 - 1,
            )) {
                break;
            }
            rock_origin.1 -= 1;
        }
        chamber.add(shift(rock.all_points(), rock_origin.0, rock_origin.1));
        if i == n_rocks {
            saved_exposed_points = Some(chamber.exposed_points_normalized());
            saved_height = Some(chamber.max_height);
        }
        if i > n_rocks && Some(chamber.exposed_points_normalized()) == saved_exposed_points {
            return (
                (i - n_rocks) as u64,
                (chamber.max_height - saved_height.unwrap()) as u64,
            );
        }
    }
    panic!("Could not find cycle");
}

fn part1(directions: &[Direction]) -> i64 {
    simulate(directions, 2022)
}

fn part2(directions: &[Direction]) -> u64 {
    // Arbitarily using 2022 as a "large enough" solution base. There's room to use something
    // smaller for speed and to combine this with part1 but ¯\_(ツ)_/¯
    let (cycle_len, jump_per_cycle) = find_cycle(directions, 2022);
    let n_rocks: u64 = 1_000_000_000_000;
    // 2022 + cycle_len * n_cycles + b = n_rocks
    // b = (n_rocks - 2022) % cycle_len
    // We need to simulate to (2022 + b) rocks
    let n_rocks_simulation = ((n_rocks - 2022) % cycle_len) + 2022;
    let n_cycles = (n_rocks - n_rocks_simulation) / cycle_len;
    simulate(directions, n_rocks_simulation) as u64 + jump_per_cycle * n_cycles
}

fn parse_input(input: &str) -> Vec<Direction> {
    input.trim().chars().flat_map(Direction::try_from).collect()
}

#[cfg(not(test))]
fn main() {
    let input = include_str!("../inputs/day17.txt");
    let input = parse_input(input);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    const TEST_INPUT: &str = r#"
>>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
    "#;

    #[test]
    fn given_part1_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part1(&input), 3068);
    }

    #[test]
    fn given_part2_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part2(&input), 1514285714288);
    }

    #[test]
    fn rocks() {
        for rock in Rock::iter() {
            println!("Testing {rock:?}");
            let all_points: HashSet<Point> = rock.all_points().into_iter().collect();
            assert_eq!(
                rock.all_points().len(),
                all_points.len(),
                "duplicate points"
            );
            for point in rock.right_points() {
                assert!(all_points.contains(&point), "right_points() not unique");
                assert!(
                    all_points
                        .iter()
                        .filter(|(_, y)| *y == point.1)
                        .all(|(x, _)| *x <= point.0),
                    "right_points() not on right"
                );
            }
            for point in rock.left_points() {
                assert!(all_points.contains(&point), "left_points() not unique");
                assert!(
                    all_points
                        .iter()
                        .filter(|(_, y)| *y == point.1)
                        .all(|(x, _)| *x >= point.0),
                    "left_points() not on left"
                );
            }
            for point in rock.bottom_points() {
                assert!(all_points.contains(&point), "bottom_points() not unique");
                assert!(
                    all_points
                        .iter()
                        .filter(|(x, _)| *x == point.0)
                        .all(|(_, y)| *y >= point.1),
                    "bottom_points not on bottom"
                );
            }
        }
    }
}
