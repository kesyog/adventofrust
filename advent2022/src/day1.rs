//! Solution to [AoC 2022 Day 1](https://adventofcode.com/2022/day/1)

use std::cmp::Reverse;

fn part1(elves: &[Vec<usize>]) -> usize {
    elves.iter().map(|i| i.iter().sum()).max().unwrap()
}

fn part2(elves: &[Vec<usize>]) -> usize {
    let mut sums: Vec<Reverse<usize>> = elves.iter().map(|i| Reverse(i.iter().sum())).collect();
    sums.sort_unstable();
    sums.into_iter().take(3).map(|i| i.0).sum()
}

fn main() {
    let input = include_str!("../inputs/day1.txt");
    let input = parse_input(input);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    utils::trim_and_split(input, "\n\n")
        .map(utils::find_all_integers)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"1000
    2000
    3000

    4000

    5000
    6000

    7000
    8000
    9000

    10000
    "#;

    #[test]
    fn given_part1_input() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 24000);
    }

    #[test]
    fn given_part2_input() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 45000);
    }
}
