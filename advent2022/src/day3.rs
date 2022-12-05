//! Solution to [AoC 2022 Day 3](https://adventofcode.com/2022/day/3)

use std::collections::HashSet;

use itertools::Itertools;

fn part1(input: &[(HashSet<char>, HashSet<char>)]) -> u32 {
    let mut out = Vec::new();
    for (ruck1, ruck2) in input {
        out.extend(ruck1.intersection(ruck2));
    }
    out.into_iter().map(priority).sum()
}

fn part2(input: &[(HashSet<char>, HashSet<char>, HashSet<char>)]) -> u32 {
    let mut out = 0;
    for (ruck1, ruck2, ruck3) in input {
        let badge = &(ruck1 & ruck2) & ruck3;
        assert_eq!(badge.len(), 1);
        out += priority(badge.into_iter().next().unwrap());
    }
    out
}

fn priority(c: char) -> u32 {
    if c.is_ascii_uppercase() {
        u32::from(c) - u32::from('A') + 1 + 26
    } else if c.is_ascii_lowercase() {
        u32::from(c) - u32::from('a') + 1
    } else {
        panic!("Invalid character {}", c);
    }
}

fn parse_input1(input: &str) -> Vec<(HashSet<char>, HashSet<char>)> {
    let mut out = Vec::new();
    for line in input.trim().lines() {
        let (ruck1, ruck2) = line.split_at(line.len() / 2);
        assert_eq!(ruck1.len(), ruck2.len());
        out.push((ruck1.chars().collect(), ruck2.chars().collect()));
    }
    out
}

fn parse_input2(input: &str) -> Vec<(HashSet<char>, HashSet<char>, HashSet<char>)> {
    let mut out = Vec::new();
    for line in &input.trim().lines().chunks(3) {
        let (ruck1, ruck2, ruck3) = line.collect_tuple().unwrap();
        out.push((
            ruck1.chars().collect(),
            ruck2.chars().collect(),
            ruck3.chars().collect(),
        ));
    }
    out
}

fn main() {
    let input = include_str!("../inputs/day3.txt");
    let input1 = parse_input1(input);
    let input2 = parse_input2(input);

    println!("Part 1: {}", part1(&input1));
    println!("Part 2: {}", part2(&input2));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
    "#;

    #[test]
    fn given_part1_input() {
        let input = parse_input1(TEST_INPUT);
        assert_eq!(part1(&input), 157);
    }

    #[test]
    fn given_part2_input() {
        let input = parse_input2(TEST_INPUT);
        assert_eq!(part2(&input), 70);
    }

    #[test]
    fn test_priority() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('A'), 27);
    }
}
