//! Solution to [AoC 2023 Day 1](https://adventofcode.com/2023/day/1)

use radix_trie::{Trie, TrieCommon};

type Parsed = Vec<&'static str>;

const TEXT_DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn part1(input: &Parsed) -> u32 {
    let mut sum = 0;
    for line in input {
        let calibration_value = 10
            * line
                .chars()
                .find(char::is_ascii_digit)
                .unwrap()
                .to_digit(10)
                .unwrap()
            + line
                .chars()
                .rfind(char::is_ascii_digit)
                .unwrap()
                .to_digit(10)
                .unwrap();
        sum += calibration_value;
    }
    sum
}

fn part2(input: &Parsed) -> u32 {
    let forward_search: Trie<String, u32> = {
        let mut trie = Trie::new();
        for i in 0..10 {
            trie.insert(i.to_string(), i);
        }
        for (i, v) in TEXT_DIGITS.into_iter().enumerate() {
            trie.insert(v.to_owned(), i as u32 + 1);
        }
        trie
    };
    let reverse_search: Trie<String, u32> = {
        let mut trie = Trie::new();
        for i in 0..10 {
            trie.insert(i.to_string(), i);
        }
        for (i, v) in TEXT_DIGITS.into_iter().enumerate() {
            trie.insert(v.chars().rev().collect(), i as u32 + 1);
        }
        trie
    };

    let mut sum = 0;
    for line in input {
        let reversed: String = line.chars().rev().collect();
        let value1 = first_match(&forward_search, line).unwrap();
        let value2 = first_match(&reverse_search, &reversed).unwrap();
        let calibration_value = 10 * value1 + value2;
        sum += calibration_value;
    }
    sum
}

/// Returns node value if the given string is an exact match
fn is_exact_match(matcher: &Trie<String, u32>, s: &str) -> Option<u32> {
    let subtrie = matcher.get_ancestor(s)?;
    subtrie.is_leaf().then(|| *subtrie.value().unwrap())
}

/// Return the first complete match in a string
fn first_match(matcher: &Trie<String, u32>, line: &str) -> Option<u32> {
    let mut iter = line.chars();
    if let Some(value) = is_exact_match(matcher, line) {
        return Some(value);
    }
    while iter.next().is_some() {
        if let Some(value) = is_exact_match(matcher, iter.as_str()) {
            return Some(value);
        }
    }
    None
}

fn parse_input(input: &'static str) -> Parsed {
    utils::trim_and_split(input, "\n").collect()
}

fn main() {
    let input = include_str!("../inputs/day1.txt");
    let input = parse_input(input);

    // Rayon overhead slows things down
    // let (p1, p2) = rayon::join(|| part1(&input), || part2(&input));
    let p1 = part1(&input);
    let p2 = part2(&input);
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_part1_input() {
        let test_input = r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
    "#;
        let input = parse_input(test_input);
        assert_eq!(part1(&input), 142);
    }

    #[test]
    fn given_part2_input() {
        let test_input = r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
        "#;
        let input = parse_input(test_input);
        assert_eq!(part2(&input), 281);
    }
}
