//! Solution to [AoC 2021 Day 10](https://adventofcode.com/2021/day/10)

enum SyntaxError {
    Corrupt(char),
    Incomplete(Vec<char>),
}

fn median(items: &[usize]) -> usize {
    let mid = items.len() / 2;
    if items.len() % 2 == 1 {
        items[mid]
    } else {
        (items[mid] + items[mid - 1]) / 2
    }
}

fn flip(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!(),
    }
}

fn parse_delimiters(line: &str) -> SyntaxError {
    let mut stack: Vec<char> = Vec::new();
    for c in line.chars() {
        match c {
            open @ ('(' | '[' | '{' | '<') => {
                stack.push(flip(open));
            }
            close @ (')' | ']' | '}' | '>') => match stack.pop() {
                Some(expected) if expected == close => (),
                _ => return SyntaxError::Corrupt(close),
            },
            _ => panic!(),
        };
    }
    SyntaxError::Incomplete(stack)
}

fn part1(input: &[&str]) -> usize {
    fn score(c: char) -> usize {
        match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!(),
        }
    }

    let mut total = 0;
    for line in input {
        if let SyntaxError::Corrupt(bad_char) = parse_delimiters(line) {
            total += score(bad_char);
        }
    }
    total
}

fn part2(input: &[&str]) -> usize {
    fn score(c: char) -> usize {
        match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!(),
        }
    }

    let mut scores = Vec::new();
    for line in input {
        if let SyntaxError::Incomplete(mut leftover) = parse_delimiters(line) {
            let mut total = 0;
            while let Some(c) = leftover.pop() {
                total = 5 * total + score(c);
            }
            scores.push(total);
        }
    }
    scores.sort_unstable();
    median(&scores)
}

fn parse_input(input: &str) -> Vec<&str> {
    utils::trim_and_split(input, "\n").collect()
}

fn main() {
    let input = include_str!("../inputs/day10.txt");
    let input = parse_input(input);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
    "#;

    #[test]
    fn given_part1_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part1(&input), 26397);
    }

    #[test]
    fn given_part2_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part2(&input), 288957);
    }
}
