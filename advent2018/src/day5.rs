//! Solution to [AoC 2018 Day 5](https://adventofcode.com/2018/day/5)

fn swap_ascii_case(chr: &char) -> char {
    assert!(chr.is_ascii_alphabetic());
    if chr.is_ascii_uppercase() {
        chr.to_ascii_lowercase()
    } else {
        chr.to_ascii_uppercase()
    }
}

fn react(polymer: &str) -> String {
    if polymer.is_empty() {
        return String::new();
    }
    polymer.chars().fold(
        String::with_capacity(polymer.len()),
        |mut acc, next_unit| {
            if acc.is_empty() {
                acc.push(next_unit);
            } else {
                let last_unit = acc.chars().last().unwrap();
                if last_unit == swap_ascii_case(&next_unit) {
                    acc.pop();
                } else {
                    acc.push(next_unit);
                }
            }
            acc
        },
    )
}

fn part2(polymer: &str) -> usize {
    const ASCII_ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
    // Try running the reaction with each letter removed
    ASCII_ALPHABET
        .chars()
        .map(|c| {
            let filtered_polymer = polymer
                .chars()
                .filter(|unit| !unit.eq_ignore_ascii_case(&c))
                .collect::<String>();
            if filtered_polymer.len() == polymer.len() {
                // The letter being tested isn't present so removing it won't make a difference
                polymer.len()
            } else {
                react(&filtered_polymer).len()
            }
        })
        .min()
        .unwrap()
}

fn main() {
    let input = include_str!("../inputs/day5.txt").trim();
    let reacted_polymer = react(&input);
    println!("Part 1: {}", reacted_polymer.len());
    // Save some time by providing the pre-reduced polymer to part 2
    println!("Part 2: {}", part2(&reacted_polymer));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_part1_input() {
        assert_eq!(0, react("").len());
        assert_eq!(0, react("aA").len());
        assert_eq!(0, react("abBA").len());
        assert_eq!(4, react("abAB").len());
        assert_eq!(6, react("aabAAB").len());
        assert_eq!(10, react("dabAcCaCBAcCcaDA").len());
    }

    #[test]
    fn given_part2_input() {
        assert_eq!(4, part2(&react("dabAcCaCBAcCcaDA")));
    }
}
