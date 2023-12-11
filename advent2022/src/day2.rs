//! Solution to [AoC 2022 Day 2](https://adventofcode.com/2022/day/2)

use anyhow::{bail, Result};
use std::cmp::Ordering;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Throw {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Throw {
    fn score(&self) -> usize {
        *self as usize
    }

    /// Return the throw that beats the input
    fn beats(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    /// Return the throw that loses to the input
    fn loses(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }
}

impl PartialOrd for Throw {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Ordering::*;
        let ord = if self == other {
            Equal
        } else if self.beats() == *other {
            Greater
        } else if self.loses() == *other {
            Less
        } else {
            unreachable!();
        };
        Some(ord)
    }
}

impl TryFrom<char> for Throw {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Throw::*;
        let throw = match value {
            'A' | 'X' => Rock,
            'B' | 'Y' => Paper,
            'C' | 'Z' => Scissors,
            _ => bail!("invalid char {}", value),
        };
        Ok(throw)
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum GameResult {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl GameResult {
    fn new(yours: Throw, theirs: Throw) -> Self {
        use GameResult::*;
        use Ordering::*;
        match PartialOrd::partial_cmp(&yours, &theirs) {
            Some(Less) => Loss,
            Some(Equal) => Draw,
            Some(Greater) => Win,
            None => unreachable!(),
        }
    }

    fn score(&self) -> usize {
        *self as usize
    }
}

impl TryFrom<char> for GameResult {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use GameResult::*;
        let result = match value {
            'X' => Loss,
            'Y' => Draw,
            'Z' => Win,
            _ => bail!("invalid char {}", value),
        };
        Ok(result)
    }
}

fn part1(input: &[(char, char)]) -> usize {
    let input: Vec<(Throw, Throw)> = input
        .iter()
        .map(|&(c1, c2)| (Throw::try_from(c1).unwrap(), Throw::try_from(c2).unwrap()))
        .collect();
    input
        .iter()
        .map(|&(theirs, yours)| GameResult::new(yours, theirs).score() + yours.score())
        .sum()
}

fn part2(input: &[(char, char)]) -> usize {
    fn your_throw(result: GameResult, theirs: Throw) -> Throw {
        match result {
            GameResult::Win => theirs.loses(),
            GameResult::Draw => theirs,
            GameResult::Loss => theirs.beats(),
        }
    }

    let input: Vec<(Throw, GameResult)> = input
        .iter()
        .map(|&(c1, c2)| {
            (
                Throw::try_from(c1).unwrap(),
                GameResult::try_from(c2).unwrap(),
            )
        })
        .collect();

    input
        .iter()
        .map(|&(theirs, result)| your_throw(result, theirs).score() + result.score())
        .sum()
}

fn parse_input(input: &str) -> Vec<(char, char)> {
    // Parse "X Y" into ('X', 'Y') line-by-line
    utils::trim_and_split(input, "\n")
        .map(|line| line.split_once(' ').unwrap())
        .map(|(s1, s2): (&str, &str)| {
            (
                s1.trim().chars().next().unwrap(),
                s2.trim().chars().next().unwrap(),
            )
        })
        .collect()
}

#[cfg(not(test))]
fn main() {
    let input = include_str!("../inputs/day2.txt");
    let input = parse_input(input);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
A Y
B X
C Z
    "#;

    #[test]
    fn given_part1_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part1(&input), 15);
    }

    #[test]
    fn given_part2_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part2(&input), 12);
    }
}
