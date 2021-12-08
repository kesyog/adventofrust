//! Solution to [AoC 2021 Day 6](https://adventofcode.com/2021/day/6)

const LIFETIME_RESET: usize = 6;
const LIFETIME_INIT: usize = 8;

fn tick(fish: &mut [usize; LIFETIME_INIT + 1]) {
    let new_fish = fish[0];
    fish.as_mut().rotate_left(1);
    fish[LIFETIME_RESET] += fish[LIFETIME_INIT];
    fish[LIFETIME_INIT] = new_fish;
}

fn solve(fish: &[usize; LIFETIME_INIT + 1], days: usize) -> usize {
    let mut fish = fish.to_owned();
    for _ in 0..days {
        tick(&mut fish);
    }
    fish.iter().sum()
}

/// Store spawn times in bins
fn parse_input(input: &str) -> [usize; LIFETIME_INIT + 1] {
    let mut counts = [0; LIFETIME_INIT + 1];
    for fish in input.trim().split(',') {
        let count = fish.parse::<usize>().unwrap();
        counts[count] += 1;
    }
    counts
}

fn main() {
    let input = include_str!("../inputs/day6.txt");
    let fish = parse_input(input);

    println!("Part 1: {}", solve(&fish, 80));
    println!("Part 2: {}", solve(&fish, 256));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn given_part1_input() {
        let fish = parse_input(TEST_INPUT);
        assert_eq!(solve(&fish, 18), 26);
        assert_eq!(solve(&fish, 80), 5934);
    }

    #[test]
    fn given_part2_input() {
        let fish = parse_input(TEST_INPUT);
        assert_eq!(solve(&fish, 256), 26984457539);
    }
}
