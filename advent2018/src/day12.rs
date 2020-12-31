//! Solution to [AoC 2018 Day 12](https://adventofcode.com/2018/day/12)

use anyhow::Error;
use std::convert::TryFrom;
use std::iter::IntoIterator;
use std::str::FromStr;

const PLANT: char = '#';
const NO_PLANT: char = '.';
/// How far out from the currently-considered pot a given rule can look
const RULE_OFFSET: i64 = 2;
/// Number of total positions in a given rule
const RULE_LENGTH: usize = RULE_OFFSET as usize * 2 + 1;

#[derive(Clone, Debug)]
struct Pots {
    pots: Vec<bool>,
    /// Any rule that results in a plant being created/maintained at the current position
    plant_rules: Vec<Vec<bool>>,
    // The true index of the first item in pots
    start_idx: i64,
}

impl Pots {
    const BORDER: [bool; RULE_LENGTH - 1] = [false; RULE_LENGTH - 1];

    fn new<T, U>(pots: T, plant_rules: U) -> Self
    where
        T: IntoIterator<Item = bool>,
        U: IntoIterator<Item = Vec<bool>>,
    {
        Self {
            pots: Self::add_border(pots.into_iter()).collect(),
            plant_rules: plant_rules.into_iter().collect(),
            start_idx: -1 * i64::try_from(Self::BORDER.len()).unwrap(),
        }
    }

    /// Step forward 1 time
    fn step(&mut self) {
        // Assume all pots will contain empty plants as a starting point, such that we only have to
        // check rules that create/maintain plants
        let mut new_pots = vec![false; self.pots.len()];

        // Sweep the rules across our row of pots. This bumps the effective start index by
        // RULE_OFFSET
        'windows: for (i, window) in self.pots.windows(RULE_LENGTH).enumerate() {
            for rule in &self.plant_rules {
                if window.clone().eq(rule) {
                    *new_pots.get_mut(i).unwrap() = true;
                    continue 'windows;
                }
            }
        }

        // Trim the pots vector so that we only store the smallest range of pots that contains all
        // plants, with an additional BORDER of empty pots added at the start and end.
        let first_plant_idx = new_pots.iter().position(|&pot| pot).unwrap();
        let last_plant_idx = new_pots.iter().rposition(|&pot| pot).unwrap();
        self.pots = Self::add_border(
            new_pots
                .get(first_plant_idx..=last_plant_idx)
                .unwrap()
                .iter()
                .copied(),
        )
        .collect();
        self.start_idx = self.start_idx + RULE_OFFSET - i64::try_from(Self::BORDER.len()).unwrap()
            + i64::try_from(first_plant_idx).unwrap();
    }

    /// Step forward n times
    fn step_by(&mut self, n: usize) {
        for _ in 0..n {
            self.step();
        }
    }

    /// Step until the count and relative position of plants remains constant and return the number
    /// of steps it took to confirm that the plants have stabilized.
    ///
    /// Note that since we have to advance to see if the plants have stabilized, the returned value
    /// is 1 higher than the first time the plants have reached their stable relative position.
    fn step_until_stable(&mut self) -> usize {
        for n_steps in 1.. {
            let old_pots = self.pots.clone();
            self.step();
            if old_pots == self.pots {
                return n_steps;
            }
        }
        unreachable!();
    }

    /// Add enough empty pots to the start and end of the given list of pots to provide room for
    /// additional plants to be added at the edges
    fn add_border<T: IntoIterator<Item = bool>>(pots: T) -> impl Iterator<Item = bool> {
        Self::BORDER
            .iter()
            .copied()
            .chain(pots.into_iter())
            .chain(Self::BORDER.iter().copied())
    }

    /// Return the sum of the indices of all pots that contain plants
    fn sum_plant_indices(&self) -> i64 {
        (self.start_idx..)
            .zip(self.pots.iter())
            .filter_map(|(i, &plant)| if plant { Some(i) } else { None })
            .sum()
    }
}

impl FromStr for Pots {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Vec<&str> = utils::trim_and_split(s, "\n").into_iter().collect();
        // Parse initial state
        let pots = input.get(0).unwrap().chars().filter_map(|c| match c {
            PLANT => Some(true),
            NO_PLANT => Some(false),
            _ => None,
        });
        // Parse rules
        let plant_rules = input.iter().skip(2).filter_map(|rule| {
            if rule.chars().last()? == NO_PLANT {
                // Ignore rules that don't create plants
                None
            } else {
                // Keep rules that create plants
                Some(
                    rule.chars()
                        .take(RULE_LENGTH)
                        .map(|c| c == PLANT)
                        .collect::<Vec<bool>>(),
                )
            }
        });
        Ok(Self::new(pots, plant_rules))
    }
}

fn part1(pots: &mut Pots) -> i64 {
    pots.step_by(20);
    pots.sum_plant_indices()
}

fn part2(pots: &mut Pots) -> i64 {
    // At some point, the count and relative positions of plants stabilizes after each turn. The
    // only thing that changes is that their absolute positions shift.
    let n_steps = pots.step_until_stable();
    let sum1 = pots.sum_plant_indices();

    // After stabilization, the sum increases linearly at each step. Calculate the linear
    // relationship (sum = slope * n_turns + offset) and use it to calculate the sum after 50
    // billion terms.
    pots.step();
    let sum2 = pots.sum_plant_indices();

    let slope: i64 = i64::try_from(sum2 - sum1).unwrap();
    let offset = sum1 - slope * i64::try_from(n_steps).unwrap();
    slope * 50_000_000_000 + offset
}

fn main() {
    let input = include_str!("../inputs/day12.txt");
    let mut pots: Pots = input.parse().unwrap();
    println!("Part 1: {:?}", part1(&mut pots));
    let mut pots: Pots = input.parse().unwrap();
    println!("Part 2: {:?}", part2(&mut pots));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #
    "#;

    #[test]
    fn given_part1_input() {
        let mut pots: Pots = TEST_INPUT.parse().unwrap();
        assert_eq!(325, part1(&mut pots));
    }

    #[test]
    fn given_part2_input() {
        // Answer not given for test data 💀
    }
}
