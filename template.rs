//! Solution to [AoC YEAR Day DAY](https://adventofcode.com/YEAR/day/DAY)

type Parsed = todo!();

fn part1(input: &Parsed) -> isize {
    0
}

fn part2(input: &Parsed) -> isize {
    0
}

fn parse_input(input: &str) -> Parsed {
    unimplemented!()
}

fn main() {
    let input = include_str!("../inputs/dayDAY.txt");
    let input = parse_input(input);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
    "#;

    #[test]
    fn given_part1_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn given_part2_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part2(&input), 0);
    }
}
