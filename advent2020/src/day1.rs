//! Solution to [AoC 2020 Day 1](https://adventofcode.com/2020/day/1)

use num_integer::Integer;
use std::collections::HashSet;
use std::hash::Hash;

#[allow(dead_code)]
const TEST_INPUT: &str = r#"
1721
979
366
299
675
1456
"#;

const TARGET_SUM: i32 = 2020;

/// Finds the two numbers in the list, if any, that add to the given sum.
///
/// The numbers are returned in the order in which they appear in the list
fn find_two_addends<T: Integer + Hash + Copy>(list: &[T], sum: T) -> Option<[T; 2]> {
    let mut cache = HashSet::with_capacity(list.len());
    for number in list {
        if let Some(other_number) = cache.get(&(sum - *number)) {
            return Some([*other_number, *number]);
        }
        cache.insert(*number);
    }
    None
}

fn part1(input: &[i32]) -> i32 {
    find_two_addends(input, TARGET_SUM)
        .expect("No solution found")
        .iter()
        .product()
}

fn part2(input: &[i32]) -> i32 {
    for (i, first) in input.iter().take(input.len() - 2).enumerate() {
        if let Some([second, third]) = find_two_addends(&input[i + 1..], TARGET_SUM - first) {
            return first * second * third;
        }
    }
    panic!("No solution found");
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
        let input_numbers = utils::find_all_integers(TEST_INPUT);
        assert_eq!(514579, part1(&input_numbers));
    }

    #[test]
    fn given_part2_input() {
        let input_numbers = utils::find_all_integers(TEST_INPUT);
        assert_eq!(241861950, part2(&input_numbers));
    }

    #[test]
    fn test_find_two_addends() {
        let numbers = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(Some([1721, 299]), find_two_addends(&numbers, 2020));
    }
}
