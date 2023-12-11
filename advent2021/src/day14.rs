//! Solution to [AoC 2021 Day 14](https://adventofcode.com/2021/day/14)
//! Runs in ~8 seconds 💣 Apparently caching pair counts was the efficient approach

use counter::Counter;
use std::collections::HashMap;
use std::convert::TryInto;

/// Run one step in the polymer generation process
fn tick(template: &[u8], rules: &HashMap<[u8; 2], u8>) -> Vec<u8> {
    let mut new = Vec::new();
    for pair in template.windows(2) {
        new.push(pair[0]);
        new.push(rules[pair]);
    }
    new.push(*template.last().unwrap());
    new
}

/// Build a polymer from a template
fn polymerize(template: &[u8], rules: &HashMap<[u8; 2], u8>, n_steps: usize) -> Vec<u8> {
    let mut template = template.to_owned();
    for _ in 0..n_steps {
        template = tick(&template, rules);
    }
    template
}

/// Count the number of *new* materials added to a template in order to build a polymer after the
/// given number of steps
fn count_insertions(template: &[u8], rules: &HashMap<[u8; 2], u8>, n_steps: usize) -> Counter<u8> {
    let polymer_count: Counter<u8> = polymerize(template, rules, n_steps).into_iter().collect();
    let template_count = Counter::init(template.iter().copied());
    polymer_count - template_count
}

/// Generate a lookup cache of material insertion counts for all possible pairs
fn build_count_cache(
    rules: &HashMap<[u8; 2], u8>,
    n_steps: usize,
) -> HashMap<[u8; 2], Counter<u8>> {
    rules
        .keys()
        .map(|pair| (*pair, count_insertions(pair, rules, n_steps)))
        .collect()
}

/// Solve the problem for the given number of steps
///
/// Simulating 40 steps takes roughly 2^40=~10^12 memory and cycles, way too much
/// Simulating 20 steps takes roughly 2^40=~10^6 memory and cycles, which is more manageable to
/// calculate and store in memory.
fn solve(template: &str, rules: &HashMap<[u8; 2], u8>, n_steps: usize) -> usize {
    assert!(n_steps % 2 == 0);
    let template: Vec<u8> = template.as_bytes().to_vec();
    // 1. Figure out what each pair expands to after half the number of steps to build a cache
    let count_cache = build_count_cache(rules, n_steps / 2);
    // 2. Expand the template halfway
    let half_polymer = polymerize(&template, rules, n_steps / 2);
    // Use the cache and the half-built polymer to figure out the counts for the full polymer
    // without ever building the full polymer
    let mut counter: Counter<u8> = Counter::init(half_polymer.iter().copied());
    for pair in half_polymer.windows(2) {
        counter += count_cache[pair].clone();
    }
    let counts = counter.most_common();
    let most_common = counts.first().unwrap().1;
    let least_common = counts.last().unwrap().1;
    most_common - least_common
}

fn part1(template: &str, rules: &HashMap<[u8; 2], u8>) -> usize {
    solve(template, rules, 10)
}

fn part2(template: &str, rules: &HashMap<[u8; 2], u8>) -> usize {
    solve(template, rules, 40)
}

/// Parse input into a tuple of (template, mapping of pairs to insertion char)
fn parse_input(input: &str) -> (&str, HashMap<[u8; 2], u8>) {
    // Treat chars as bytes for a tiny speedup
    assert!(input.is_ascii());
    let mut rules_map = HashMap::new();
    let (template, rules) = input.trim().split_once("\n\n").unwrap();
    for rule in rules.split('\n') {
        let (pair, insertion) = rule.split_once(" -> ").unwrap();
        let pair = pair.as_bytes().try_into().unwrap();
        let insertion = insertion.as_bytes()[0];
        rules_map.insert(pair, insertion);
    }
    (template, rules_map)
}

#[cfg(not(test))]
fn main() {
    let input = include_str!("../inputs/day14.txt");
    let (template, rules) = parse_input(input);

    println!("Part 1: {}", part1(template, &rules));
    println!("Part 2: {}", part2(template, &rules));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
    "#;

    #[test]
    fn given_part1_input() {
        let (template, rules) = parse_input(TEST_INPUT);
        assert_eq!(1588, part1(template, &rules));
    }

    #[test]
    fn given_part2_input() {
        let (template, rules) = parse_input(TEST_INPUT);
        assert_eq!(2188189693529, part2(template, &rules));
    }
}
