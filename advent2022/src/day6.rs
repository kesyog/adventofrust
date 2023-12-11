//! Solution to [AoC 2022 Day 6](https://adventofcode.com/2022/day/6)

use counter::Counter;
use std::iter;

// Scan through string via overlapping windows, re-using a HashMap-based counter to minimize
// allocations and re-counting of characters
fn find_start_packet(input: &str, window_size: usize) -> Option<usize> {
    let mut counter: Counter<u8> = input
        .as_bytes()
        .iter()
        // Counter is initialized with all but one of the elements of the first window since the
        // loop will add the last element
        .take(window_size - 1)
        .copied()
        .collect();

    for (i, window) in input.as_bytes().windows(window_size).enumerate() {
        counter.update(iter::once(window[window_size - 1]));
        if counter.len() == window_size {
            return Some(i + window_size);
        }
        counter.subtract(iter::once(window[0]));
    }
    None
}

fn part1(input: &str) -> Option<usize> {
    find_start_packet(input, 4)
}

fn part2(input: &str) -> Option<usize> {
    find_start_packet(input, 14)
}

#[cfg(not(test))]
fn main() {
    let input = include_str!("../inputs/day6.txt").trim();

    println!("Part 1: {}", part1(input).expect("a start message"));
    println!("Part 2: {}", part2(input).expect("a start message"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_part1_input() {
        const TEST_INPUTS: [(&str, usize); 6] = [
            ("abcde", 4),
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];

        for (input, expected) in TEST_INPUTS {
            assert_eq!(Some(expected), part1(input));
        }
    }

    #[test]
    fn given_part2_input() {
        const TEST_INPUTS: [(&str, usize); 5] = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];
        for (input, expected) in TEST_INPUTS {
            assert_eq!(Some(expected), part2(input));
        }
    }
}
