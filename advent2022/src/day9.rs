//! Solution to [AoC 2022 Day 9](https://adventofcode.com/2022/day/9)

use lending_iterator::LendingIterator;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
struct Location {
    x: isize,
    y: isize,
}

impl Location {
    #[allow(dead_code)]
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn is_adjacent(&self, other: Self) -> bool {
        self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1
    }

    fn step(&self, step: &Move) -> Self {
        let mut new = *self;
        match step {
            Move::Left(_) => new.x -= 1,
            Move::Right(_) => new.x += 1,
            Move::Up(_) => new.y -= 1,
            Move::Down(_) => new.y += 1,
        };
        new
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Move {
    Left(isize),
    Right(isize),
    Up(isize),
    Down(isize),
}

impl Move {
    fn mag(&self) -> isize {
        *match self {
            Self::Left(mag) | Self::Right(mag) | Self::Up(mag) | Self::Down(mag) => mag,
        }
    }
}

fn move_tail(mut tail: Location, head: Location) -> Location {
    if tail.is_adjacent(head) {
        return tail;
    }

    if tail.x == head.x {
        assert_eq!(tail.y.abs_diff(head.y), 2);
        tail.y += isize::signum(head.y - tail.y);
    } else if tail.y == head.y {
        assert_eq!(tail.x.abs_diff(head.x), 2);
        tail.x += isize::signum(head.x - tail.x);
    } else {
        tail.y += isize::signum(head.y - tail.y);
        tail.x += isize::signum(head.x - tail.x);
    }
    assert!(tail.is_adjacent(head));
    tail
}

fn num_visited<const KNOTS: usize>(moves: &[Move]) -> usize {
    let mut knots: [Location; KNOTS] = [Location::default(); KNOTS];
    let mut tail_visited: HashSet<Location> = HashSet::new();
    tail_visited.insert(knots[KNOTS - 1]);

    for step in moves {
        for _ in 0..step.mag() {
            knots[0] = knots[0].step(step);
            // Use lending_iterator crate to avoid iterating by index
            // array::windows_mut in std someday?
            let mut array_windows = lending_iterator::windows_mut::<_, 2>(&mut knots);
            while let Some(&mut [prev, ref mut next]) = array_windows.next() {
                *next = move_tail(*next, prev);
            }
            tail_visited.insert(knots[KNOTS - 1]);
        }
    }
    tail_visited.len()
}

fn part1(input: &[Move]) -> usize {
    num_visited::<2>(input)
}

fn part2(input: &[Move]) -> usize {
    num_visited::<10>(input)
}

fn parse_input(input: &str) -> Vec<Move> {
    let mut out = Vec::new();
    for line in input.trim().lines() {
        let (d, mag) = line.split_once(' ').unwrap();
        let mag = mag.parse::<isize>().unwrap();
        out.push(match d {
            "R" => Move::Right(mag),
            "L" => Move::Left(mag),
            "U" => Move::Up(mag),
            "D" => Move::Down(mag),
            _ => panic!("invalid direction: {d}"),
        });
    }
    out
}

fn main() {
    let input = include_str!("../inputs/day9.txt");
    let input = parse_input(input);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
    "#;

    #[test]
    fn test_move_tail() {
        let tail = Location::new(1, 1);
        // Overlap -> nop
        assert_eq!(Location::new(1, 1), move_tail(tail, Location::new(1, 1)));
        // Adjacent -> nop
        assert_eq!(Location::new(1, 1), move_tail(tail, Location::new(1, 2)));
        assert_eq!(Location::new(1, 1), move_tail(tail, Location::new(1, 0)));
        assert_eq!(Location::new(1, 1), move_tail(tail, Location::new(2, 2)));
        assert_eq!(Location::new(1, 1), move_tail(tail, Location::new(0, 0)));
        // Move down
        assert_eq!(Location::new(1, 2), move_tail(tail, Location::new(1, 3)));
        // Move up
        assert_eq!(Location::new(1, 0), move_tail(tail, Location::new(1, -1)));
        // Move right
        assert_eq!(Location::new(2, 1), move_tail(tail, Location::new(3, 1)));
        // Move left
        assert_eq!(Location::new(0, 1), move_tail(tail, Location::new(-1, 1)));
    }

    #[test]
    fn given_part1_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn given_part2_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part2(&input), 1);
    }

    #[test]
    fn given_part2_input2() {
        let larger_test_input: &str = r#"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"#;
        let input = parse_input(larger_test_input);
        assert_eq!(part2(&input), 36);
    }
}
