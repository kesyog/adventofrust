//! Solution to [AoC 2022 Day 4](https://adventofcode.com/2022/day/4)

fn part1(input: &[[usize; 4]]) -> usize {
    input
        .iter()
        .filter(|[a, b, c, d]| (a <= c && b >= d) || (c <= a && d >= b))
        .count()
}

fn part2(input: &[[usize; 4]]) -> usize {
    input
        .iter()
        .filter(|[a, b, c, d]| (a >= c && a <= d) || (c >= a && c <= b))
        .count()
}

fn parse_input(input: &str) -> Vec<[usize; 4]> {
    let mut out = Vec::new();
    for line in utils::trim_and_split(input, "\n") {
        let nums: Vec<usize> = line
            .split(|c| c == '-' || c == ',')
            .map(|num| num.parse::<usize>().unwrap())
            .collect();
        out.push(nums.try_into().unwrap());
    }
    out
}

#[cfg(not(test))]
fn main() {
    let input = include_str!("../inputs/day4.txt");
    let input = parse_input(input);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
    "#;

    #[test]
    fn given_part1_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn given_part2_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part2(&input), 4);
    }
}
