//! Solution to [AoC 2018 Day 1](https://adventofcode.com/2018/day/1)

use std::collections::HashSet;

const START_FREQ: i32 = 0;

fn part1(changes: &[i32]) -> i32 {
    START_FREQ + changes.iter().sum::<i32>()
}

/// Store already-seen values in a hash set for fast insertion and lookup
fn part2(changes: &[i32]) -> i32 {
    let mut freq = START_FREQ;
    let mut cache = HashSet::new();

    for delta in changes.iter().cycle() {
        cache.insert(freq);
        freq += delta;
        if cache.contains(&freq) {
            return freq;
        }
    }
    unreachable!();
}

#[cfg(not(test))]
fn main() {
    let input = include_str!("../inputs/day1.txt");
    let input: Vec<i32> = utils::find_all_integers(input);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_part1_input() {
        assert_eq!(3, part1(&vec!(1, -2, 3, 1)));
        assert_eq!(3, part1(&vec!(1, 1, 1)));
        assert_eq!(0, part1(&vec!(1, 1, -2)));
        assert_eq!(-6, part1(&vec!(-1, -2, -3)));
    }

    #[test]
    fn given_part2_input() {
        assert_eq!(2, part2(&vec!(1, -2, 3, 1)));
        assert_eq!(0, part2(&vec!(1, -1)));
        assert_eq!(10, part2(&vec!(3, 3, 4, -2, -4)));
        assert_eq!(5, part2(&vec!(-6, 3, 8, 5, -6)));
        assert_eq!(14, part2(&vec!(7, 7, -2, -7, -4)));
    }
}
