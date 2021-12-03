//! Solution to [AoC 2021 Day 3](https://adventofcode.com/2021/day/3)
#![feature(drain_filter)]

use ndarray::Array1;
use std::ops;

fn flip_bit(bit: char) -> char {
    match bit {
        '1' => '0',
        '0' => '1',
        _ => panic!(),
    }
}

fn part1(input: &[&str]) -> u64 {
    let n_bits = input[0].chars().count();
    // Build an n_bits long array containing the number of 1's at each bit position
    let count_ones: Array1<usize> = input
        .iter()
        .map(|line| {
            line.chars()
                .map(|bit| match bit {
                    '0' => 0,
                    '1' => 1,
                    _ => panic!(),
                })
                .collect::<Array1<usize>>()
        })
        .fold(Array1::zeros(n_bits), ops::Add::add);
    // Create a binary string containing the most common bit at each bit position
    let gamma_rate: String = count_ones
        .iter()
        .map(|&count| if 2 * count >= input.len() { '1' } else { '0' })
        .collect();
    // Create a binary string containing the least common bit at each bit position
    let epsilon_rate: String = gamma_rate.chars().map(flip_bit).collect();
    let gamma_rate = u64::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_rate = u64::from_str_radix(&epsilon_rate, 2).unwrap();
    gamma_rate * epsilon_rate
}

/// Find the most common bit at a given bit position. Ties favor '1'
fn most_common_bit(input: &[&str], position: usize) -> char {
    let count_ones: usize = input
        .iter()
        .map(|line| line.chars().nth(position))
        .filter(|c| *c == Some('1'))
        .count();
    if 2 * count_ones >= input.len() {
        '1'
    } else {
        '0'
    }
}

fn oxygen_generator_rating(mut input: Vec<&str>) -> u64 {
    let mut bit_position: usize = 0;
    while input.len() > 1 {
        let bit = most_common_bit(&input, bit_position);
        input.drain_filter(|line| line.chars().nth(bit_position).unwrap() != bit);
        bit_position += 1;
    }
    u64::from_str_radix(input[0], 2).unwrap()
}

fn co2_scrubber_rating(mut input: Vec<&str>) -> u64 {
    let mut bit_position: usize = 0;
    while input.len() > 1 {
        let bit = flip_bit(most_common_bit(&input, bit_position));
        input.drain_filter(|line| line.chars().nth(bit_position).unwrap() != bit);
        bit_position += 1;
    }
    u64::from_str_radix(input[0], 2).unwrap()
}

fn part2(input: &[&str]) -> u64 {
    let o2_rating = oxygen_generator_rating(input.to_owned());
    let co2_rating = co2_scrubber_rating(input.to_owned());
    o2_rating * co2_rating
}

fn parse_input(input: &str) -> Vec<&str> {
    utils::trim_and_split(input, "\n").collect()
}

fn main() {
    let input = include_str!("../inputs/day3.txt");
    let input = parse_input(input);

    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
    "#;

    #[test]
    fn given_part1_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part1(&input), 198);
    }

    #[test]
    fn given_part2_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part2(&input), 230);
    }
}
