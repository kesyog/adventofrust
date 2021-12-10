//! Solution to [AoC 2021 Day 8](https://adventofcode.com/2021/day/8)

use std::collections::{HashMap, HashSet};

type Reading = (Vec<String>, Vec<String>);

/// Solve via logical deduction
fn solve(inputs: &[String]) -> HashMap<String, usize> {
    let mut inputs = inputs.to_owned();
    let mut map = HashMap::new();
    // 7
    // only length 3
    let seven_idx = inputs.iter().position(|i| i.len() == 3).unwrap();
    let seven = inputs.swap_remove(seven_idx);
    let seven_set: HashSet<char> = seven.chars().collect();
    map.insert(seven, 7);
    // 4
    // only length 4
    let four_idx = inputs.iter().position(|i| i.len() == 4).unwrap();
    map.insert(inputs.swap_remove(four_idx), 4);
    // 1
    // only length 2
    let one_idx = inputs.iter().position(|i| i.len() == 2).unwrap();
    let one = inputs.swap_remove(one_idx);
    let one_set: HashSet<char> = one.chars().collect();
    map.insert(one, 1);
    // 8
    // only length 7
    let eight_idx = inputs.iter().position(|i| i.len() == 7).unwrap();
    map.insert(inputs.swap_remove(eight_idx), 8);
    // 6
    // find all length 6 (0, 6, 9). the one that doesn't overlap with 1 is 6
    let six_idx = inputs
        .iter()
        .position(|i| {
            if i.len() != 6 {
                return false;
            }
            let set: HashSet<char> = i.chars().collect();
            !set.is_superset(&one_set)
        })
        .unwrap();
    let six = inputs.swap_remove(six_idx);
    let six_set: HashSet<char> = six.chars().collect();
    map.insert(six, 6);
    // 3
    // find all length 5 (5, 2, 3). the one that does overlap with 1 is 3
    let three_idx = inputs
        .iter()
        .position(|i| {
            if i.len() != 5 {
                return false;
            }
            let set: HashSet<char> = i.chars().collect();
            set.is_superset(&one_set)
        })
        .unwrap();
    map.insert(inputs.swap_remove(three_idx), 3);
    // find all remaining length 5 (5, 2). the one that overlaps with 6 is 5
    let five_idx = inputs
        .iter()
        .position(|i| {
            if i.len() != 5 {
                return false;
            }
            let set: HashSet<char> = i.chars().collect();
            set.is_subset(&six_set)
        })
        .unwrap();
    let five = inputs.swap_remove(five_idx);
    let five_set: HashSet<char> = five.chars().collect();
    map.insert(five, 5);
    // 2
    // remaining length 5 is 2
    let two_idx = inputs.iter().position(|i| i.len() == 5).unwrap();
    map.insert(inputs.swap_remove(two_idx), 2);
    // 9
    // union of 5 + 7 = 9
    let nine: String = five_set.union(&seven_set).collect();
    let nine = sort_str(&nine);
    let nine_idx = inputs.iter().position(|i| i == &nine).unwrap();
    inputs.remove(nine_idx);
    map.insert(nine, 9);
    // 0
    // remaining length 6 is 0
    let zero_idx = inputs.iter().position(|i| i.len() == 6).unwrap();
    map.insert(inputs.swap_remove(zero_idx), 0);
    assert!(inputs.is_empty());
    map
}

/// Decode the output string using a decoder, the output of `solve`
fn decode(output: &[String], decoder: &HashMap<String, usize>) -> usize {
    let digits: String = output.iter().map(|i| decoder[i].to_string()).collect();
    digits.parse::<usize>().unwrap()
}

fn part1(readings: &[Reading]) -> usize {
    let mut total = 0;
    for reading in readings {
        total += reading
            .1
            .iter()
            .filter(|word| matches!(word.len(), 2 | 3 | 4 | 7))
            .count()
    }
    total
}

fn part2(readings: &[Reading]) -> usize {
    let mut total = 0;
    for reading in readings {
        let decoder = solve(&reading.0);
        total += decode(&reading.1, &decoder);
    }
    total
}

fn sort_str(input: &str) -> String {
    assert!(input.is_ascii());
    let mut bytes = input.as_bytes().to_owned();
    bytes.sort_unstable();
    String::from_utf8(bytes).unwrap()
}

fn parse_input(input: &str) -> Vec<Reading> {
    let mut ret = Vec::new();
    for line in utils::trim_and_split(input, "\n") {
        let (segments, output) = line.split_once(" | ").unwrap();
        let segments: Vec<String> = segments.split(' ').map(sort_str).collect();
        let output: Vec<String> = output.split(' ').map(sort_str).collect();
        ret.push((segments, output));
    }
    ret
}

fn main() {
    let input = include_str!("../inputs/day8.txt");
    let input = parse_input(input);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = r#"acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf
    "#;
    const TEST_INPUT2: &str = r#"
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
        "#;

    #[test]
    fn given_part1_input() {
        let input = parse_input(TEST_INPUT2);
        assert_eq!(part1(&input), 26);
    }

    #[test]
    fn given_part2_input() {
        let input = parse_input(TEST_INPUT1);
        assert_eq!(part2(&input), 5353);
        let input = parse_input(TEST_INPUT2);
        assert_eq!(part2(&input), 61229);
    }
}
