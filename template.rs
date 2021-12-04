//! Solution to [AoC YEAR Day DAY](https://adventofcode.com/YEAR/day/DAY)

fn part1() -> isize {
    0
}

fn part2() -> isize {
    0
}

fn parse_input(input: &str) {
    unimplemented!()
}

fn main() {
    let input = include_str!("../inputs/dayDAY.txt");
    let input = parse_input(input);

    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
    "#;

    #[test]
    fn given_part1_input() {
        let input = parse_input(TEST_INPUT);
    }

    #[test]
    fn given_part2_input() {
        let input = parse_input(TEST_INPUT);
    }
}
