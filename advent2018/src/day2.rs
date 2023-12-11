//! Solution to [AoC 2018 Day 2](https://adventofcode.com/2018/day/2)

use std::collections::HashMap;

fn part1(ids: &[&str]) -> usize {
    let mut n_twos = 0;
    let mut n_threes = 0;
    for id in ids {
        // Count the occurrences of each character in the string
        let mut counter: HashMap<char, usize> = HashMap::with_capacity(ids.len());
        for chr in id.chars() {
            let count = counter.entry(chr).or_insert(0);
            *count += 1_usize;
        }
        let mut has_two = false;
        let mut has_three = false;

        for value in counter.values() {
            if *value == 2_usize && !has_two {
                has_two = true;
                n_twos += 1;
                if has_three {
                    break;
                }
            } else if *value == 3_usize && !has_three {
                has_three = true;
                n_threes += 1;
                if has_two {
                    break;
                }
            }
        }
    }
    n_twos * n_threes
}

/// Find the two ids that differ by only one character when compared element-by-element
fn part2(ids: &[&str]) -> String {
    for (i, id1) in ids.iter().take(ids.len() - 1).enumerate() {
        for id2 in ids[i + 1..].iter() {
            assert_eq!(id1.len(), id2.len());

            let mut n_differences = 0;
            let mut common = String::with_capacity(id1.len());
            for (chr1, chr2) in id1.chars().zip(id2.chars()) {
                if chr1 == chr2 {
                    common.push(chr1)
                } else {
                    n_differences += 1;
                    // If there are two or more differences, bail early since these can't be the
                    // correct IDs
                    if n_differences > 1 {
                        break;
                    }
                }
            }
            if n_differences == 1 {
                return common;
            }
        }
    }
    panic!("No solution found");
}

fn parse_ids(input: &str) -> Vec<&str> {
    utils::trim_and_split(input, "\n").collect()
}

#[cfg(not(test))]
fn main() {
    let input = include_str!("../inputs/day2.txt");
    let ids = parse_ids(input);

    println!("Part 1: {}", part1(&ids));
    println!("Part 2: {}", part2(&ids));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_part1_input() {
        const TEST_INPUT: &str = r#"
            abcdef
            bababc
            abbcde
            abcccd
            aabcdd
            abcdee
            ababab
            "#;
        let ids = parse_ids(TEST_INPUT);
        assert_eq!(12, part1(&ids));
    }

    #[test]
    fn given_part2_input() {
        const TEST_INPUT: &str = r#"
            abcde
            fghij
            klmno
            pqrst
            fguij
            axcye
            wvxyz
            "#;
        let ids = parse_ids(TEST_INPUT);
        assert_eq!("fgij", part2(&ids));
    }
}
