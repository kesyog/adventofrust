//! Solution to [AoC 2022 Day 23](https://adventofcode.com/2022/day/23)

use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use num_complex::Complex;
use rayon::prelude::*;

type Coordinate = Complex<isize>;
type Elves = HashSet<Coordinate>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Proposal {
    North,
    South,
    West,
    East,
}

impl Proposal {
    fn delta(self) -> Coordinate {
        match self {
            Proposal::North => Complex::i(),
            Proposal::South => -Complex::i(),
            Proposal::West => Complex::from(-1),
            Proposal::East => Complex::from(1),
        }
    }

    fn to_check(self) -> [Coordinate; 3] {
        let base = self.delta();
        match self {
            Proposal::North | Proposal::South => [base, base + 1, base - 1],
            Proposal::West | Proposal::East => [base, base + Complex::i(), base - Complex::i()],
        }
    }
}

fn empty_ground_tiles(elves: &Elves) -> usize {
    let (mut min_x, mut max_x, mut min_y, mut max_y) =
        (isize::MAX, isize::MIN, isize::MAX, isize::MIN);
    for elf in elves {
        min_x = min_x.min(elf.re);
        max_x = max_x.max(elf.re);
        min_y = min_y.min(elf.im);
        max_y = max_y.max(elf.im);
    }
    (max_x - min_x + 1) as usize * (max_y - min_y + 1) as usize - elves.len()
}

fn both_parts(mut elves: Elves) -> (usize, usize) {
    let mut part1 = None;
    let mut checks = VecDeque::from([
        Proposal::North,
        Proposal::South,
        Proposal::West,
        Proposal::East,
    ]);
    let mut targets = HashMap::with_capacity(elves.len());
    let mut conflicts: HashSet<Coordinate> = HashSet::with_capacity(elves.len());
    let neighbors: [Coordinate; 8] = TryInto::<[Coordinate; 8]>::try_into(
        [-1, 0, 1]
            .into_iter()
            .cartesian_product([-1, 0, 1])
            .filter(|&(r, i)| r != 0 || i != 0)
            .map(|(r, i)| Complex::new(r, i))
            .collect::<Vec<Coordinate>>(),
    )
    .unwrap();

    for round in 1.. {
        targets.clear();
        conflicts.clear();
        // First step: voting
        let votes: Vec<(Coordinate, Coordinate)> = elves
            .par_iter()
            .filter_map(|&elf| {
                if !neighbors.iter().any(|delta| elves.contains(&(elf + delta))) {
                    return None;
                }
                for &proposal in &checks {
                    if proposal
                        .to_check()
                        .into_iter()
                        .all(|delta| !elves.contains(&(elf + delta)))
                    {
                        let new_position = elf + proposal.delta();
                        return Some((new_position, elf));
                    }
                }
                None
            })
            .collect();
        for (new_pos, old_pos) in votes {
            if targets.insert(new_pos, old_pos).is_some() {
                conflicts.insert(new_pos);
            }
        }
        // Second step: Check votes and move uniquely-voting elves
        targets.retain(|new_pos, _| !conflicts.contains(new_pos));
        if targets.is_empty() {
            return (part1.expect("an answer to part1"), round);
        }
        for (&new_pos, old_pos) in &targets {
            elves.remove(old_pos);
            elves.insert(new_pos);
        }
        if round == 10 {
            part1 = Some(empty_ground_tiles(&elves));
        }

        checks.rotate_left(1);
    }
    unreachable!();
}

fn parse_input(input: &str) -> Elves {
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().filter_map(move |(col, c)| {
                // Negate row so that the +j axis is up
                (c == '#').then_some(Complex::new(
                    isize::try_from(col).unwrap(),
                    -(isize::try_from(row).unwrap()),
                ))
            })
        })
        .collect()
}

#[cfg(not(test))]
fn main() {
    let input = include_str!("../inputs/day23.txt");
    let input = parse_input(input);

    let (p1, p2) = both_parts(input);
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
    "#;

    #[test]
    fn given_inputs() {
        let input = parse_input(TEST_INPUT);
        let (p1, p2) = both_parts(input);
        assert_eq!(p1, 110);
        assert_eq!(p2, 20);
    }

    #[test]
    fn test_empty_ground_tiles() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(empty_ground_tiles(&input), 27);
    }
}
