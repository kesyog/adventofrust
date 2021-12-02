//! Solution to [AoC 2018 Day 3](https://adventofcode.com/2018/day/3)

use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Claim {
    id: u32,
    /// x, y coordinates of top left corner
    loc: (usize, usize),
    height: usize,
    width: usize,
}

/// Return set of x,y coordinates of all contested areas
fn get_contested_land(claims: &[Claim]) -> HashSet<(usize, usize)> {
    let mut all_claims: HashSet<(usize, usize)> = HashSet::new();
    let mut overlapping: HashSet<(usize, usize)> = HashSet::new();

    for claim in claims {
        for (x, y) in (claim.loc.0..)
            .take(claim.width)
            .cartesian_product((claim.loc.1..).take(claim.height))
        {
            if !all_claims.insert((x, y)) {
                overlapping.insert((x, y));
            }
        }
    }
    overlapping
}

fn part1(overlapping: &HashSet<(usize, usize)>) -> usize {
    overlapping.len()
}

fn part2(claims: &[Claim], overlapping: &HashSet<(usize, usize)>) -> u32 {
    'outer: for claim in claims {
        for (x, y) in (claim.loc.0..)
            .take(claim.width)
            .cartesian_product((claim.loc.1..).take(claim.height))
        {
            if overlapping.contains(&(x, y)) {
                continue 'outer;
            }
        }
        return claim.id;
    }
    panic!("No intact claims found");
}

/// Parse input file into a list of [Claims](Claim). Each line of the file describes a different [Claim].
fn parse_claims(input: &str) -> Vec<Claim> {
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    utils::trim_and_split(input, "\n")
        .map(|line| {
            let captures = re.captures(line).unwrap();
            Claim {
                id: captures.get(1).unwrap().as_str().parse().unwrap(),
                loc: (
                    captures.get(2).unwrap().as_str().parse().unwrap(),
                    captures.get(3).unwrap().as_str().parse().unwrap(),
                ),
                height: captures.get(5).unwrap().as_str().parse().unwrap(),
                width: captures.get(4).unwrap().as_str().parse().unwrap(),
            }
        })
        .collect()
}

fn main() {
    let input = include_str!("../inputs/day3.txt");
    let claims = parse_claims(input);
    let overlapping = get_contested_land(&claims);
    println!("Part 1: {}", part1(&overlapping));
    println!("Part 2: {}", part2(&claims, &overlapping));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_parsing() {
        const TEST_INPUT: &str = r#"
            #1 @ 1,3: 4x4
            #2 @ 3,1: 2x3
            #3 @ 5,5: 2x2
            "#;
        let claims = parse_claims(TEST_INPUT);
        assert_eq!(
            claims[0],
            Claim {
                id: 1,
                loc: (1, 3),
                height: 4,
                width: 4
            }
        );
        assert_eq!(
            claims[1],
            Claim {
                id: 2,
                loc: (3, 1),
                height: 3,
                width: 2
            }
        );
        assert_eq!(
            claims[2],
            Claim {
                id: 3,
                loc: (5, 5),
                height: 2,
                width: 2
            }
        );
        assert_eq!(claims.len(), 3);
    }

    #[test]
    fn given_part1_input() {
        const TEST_INPUT: &str = r#"
            #1 @ 1,3: 4x4
            #2 @ 3,1: 4x4
            #3 @ 5,5: 2x2
            "#;
        let claims = parse_claims(TEST_INPUT);
        let overlapping = get_contested_land(&claims);
        assert_eq!(4, part1(&overlapping));
    }

    #[test]
    fn given_part2_input() {
        const TEST_INPUT: &str = r#"
            #1 @ 1,3: 4x4
            #2 @ 3,1: 4x4
            #3 @ 5,5: 2x2
            "#;
        let claims = parse_claims(TEST_INPUT);
        let overlapping = get_contested_land(&claims);
        assert_eq!(3, part2(&claims, &overlapping));
    }
}
