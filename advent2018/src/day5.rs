//! Solution to [AoC 2018 Day 5](https://adventofcode.com/2018/day/5)

fn swap_ascii_case(chr: &char) -> char {
    assert!(chr.is_ascii_alphabetic());
    if chr.is_ascii_uppercase() {
        chr.to_ascii_lowercase()
    } else {
        chr.to_ascii_uppercase()
    }
}

/// Perform one reduction iteration of the reaction and return the result
fn perform_reaction_step(polymer: &str) -> String {
    // Iterate over tuples containing each character in the string as well as the next character
    let mut iter = polymer
        .chars()
        .zip(polymer[1..].chars().map(Some).chain(std::iter::once(None)));
    let mut new_polymer = String::with_capacity(polymer.len());
    loop {
        match iter.next() {
            Some((unit, Some(next_unit))) => {
                if unit == swap_ascii_case(&next_unit) {
                    // Skip over the current unit and the next unit since they have reacted
                    iter.next();
                } else {
                    // The current unit did not react with its neighbor
                    new_polymer.push(unit);
                }
            }
            // Edge case for the last character in the string
            Some((unit, None)) => {
                new_polymer.push(unit);
                break;
            }
            // Reached end-of-string
            None => break,
        }
    }
    new_polymer
}

/// Repeatedly run the reaction until the polymer string stops changing
fn run_reaction_to_steady_state(polymer: &str) -> String {
    let mut polymer = polymer.to_string();
    while !polymer.is_empty() {
        let new_polymer = perform_reaction_step(&polymer);
        if new_polymer.len() == polymer.len() {
            assert_eq!(new_polymer, polymer);
            break;
        }
        polymer = new_polymer;
    }
    polymer
}

fn part2(polymer: &str) -> usize {
    const ASCII_ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
    // Try running the reaction to completion with each letter removed
    // Potential optimization: track which letters are in the polymer and only bother iterating over
    // those
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
                run_reaction_to_steady_state(&filtered_polymer).len()
            }
        })
        .min()
        .unwrap()
}

fn main() {
    let input = include_str!("../inputs/day5.txt").trim();
    let reacted_polymer = run_reaction_to_steady_state(&input);
    println!("Part 1: {}", reacted_polymer.len());
    println!("Part 2: {}", part2(&reacted_polymer));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reaction_step() {
        assert_eq!("a", perform_reaction_step("a"));
        assert_eq!("A", perform_reaction_step("A"));
        assert_eq!("", perform_reaction_step("aA"));
        assert_eq!("", perform_reaction_step("Aa"));
        assert_eq!("aA", perform_reaction_step("abBA"));
        assert_eq!("aA", perform_reaction_step("aBbA"));
        assert_eq!("abba", perform_reaction_step("abba"));
        assert_eq!("ABBA", perform_reaction_step("ABBA"));
        assert_eq!("Aa", perform_reaction_step("AbBa"));
    }

    #[test]
    fn given_part1_input() {
        assert_eq!(0, run_reaction_to_steady_state("aA").len());
        assert_eq!(0, run_reaction_to_steady_state("abBA").len());
        assert_eq!(4, run_reaction_to_steady_state("abAB").len());
        assert_eq!(6, run_reaction_to_steady_state("aabAAB").len());
        assert_eq!(10, run_reaction_to_steady_state("dabAcCaCBAcCcaDA").len());
    }

    #[test]
    fn given_part2_input() {
        assert_eq!(4, part2(&run_reaction_to_steady_state("dabAcCaCBAcCcaDA")));
    }
}
