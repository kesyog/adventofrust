//! Solution to [AoC 2021 Day 7](https://adventofcode.com/2021/day/7)

fn cost_function2(positions: &[isize], position: isize) -> isize {
    positions
        .iter()
        .map(|i| {
            // Sum of 1..=n = n * (n+1) / 2
            let abs_diff = isize::abs(i - position);
            abs_diff * (abs_diff + 1) / 2
        })
        .sum()
}

fn cost_function1(positions: &[isize], position: isize) -> isize {
    positions.iter().map(|i| isize::abs(i - position)).sum()
}

/// Calculate floor(median) of a list
fn median(items: &[isize]) -> isize {
    let mid = items.len() / 2;
    if items.len() % 2 == 1 {
        items[mid]
    } else {
        (items[mid] + items[mid - 1]) / 2
    }
}

// The optimal position must be at the median of the list, as the median balances the number of
// elements on either side
fn part1(sorted_pos: &[isize]) -> isize {
    let optimal_pos = median(sorted_pos);
    cost_function1(sorted_pos, optimal_pos)
}

fn part2(sorted_pos: &[isize]) -> isize {
    let mut last = (isize::MIN, isize::MAX);
    // TODO: Binary search
    for i in *sorted_pos.first().unwrap()..=*sorted_pos.last().unwrap() {
        let cost = cost_function2(sorted_pos, i);
        if cost < last.1 {
            last = (i, cost);
        } else {
            return last.1;
        }
    }
    cost_function2(sorted_pos, *sorted_pos.last().unwrap())
}

fn parse_input(input: &str) -> Vec<isize> {
    let mut positions: Vec<isize> = input
        .trim()
        .split(',')
        .map(|i| i.parse::<isize>().unwrap())
        .collect();
    positions.sort_unstable();
    positions
}

#[cfg(not(test))]
fn main() {
    let input = include_str!("../inputs/day7.txt");
    let sorted_positions = parse_input(input);

    println!("Part 1: {}", part1(&sorted_positions));
    println!("Part 2: {}", part2(&sorted_positions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn given_part1_input() {
        let positions = parse_input(TEST_INPUT);
        assert_eq!(part1(&positions), 37);
    }

    #[test]
    fn given_part2_input() {
        let positions = parse_input(TEST_INPUT);
        assert_eq!(part2(&positions), 168);
    }

    #[test]
    fn cost1() {
        let positions = parse_input(TEST_INPUT);
        assert_eq!(cost_function1(&positions, 2), 37);
        assert_eq!(cost_function1(&positions, 1), 41);
        assert_eq!(cost_function1(&positions, 3), 39);
    }

    #[test]
    fn cost2() {
        let positions = parse_input(TEST_INPUT);
        assert_eq!(cost_function2(&positions, 2), 206);
        assert_eq!(cost_function2(&positions, 5), 168);
    }
}
