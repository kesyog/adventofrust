//! Solution to [AoC 2021 Day 2](https://adventofcode.com/2021/day/2)

struct Position {
    x: isize,
    depth: isize,
    aim: isize,
}

impl Position {
    fn new() -> Self {
        Self {
            x: 0,
            depth: 0,
            aim: 0,
        }
    }

    /// Update function for part 1
    fn update(&mut self, direction: &str, n: isize) {
        match direction {
            "forward" => self.x += n,
            "up" => self.depth -= n,
            "down" => self.depth += n,
            _ => (),
        }
    }

    /// Update function for part 2
    fn update2(&mut self, direction: &str, n: isize) {
        match direction {
            "forward" => {
                self.x += n;
                self.depth += self.aim * n
            }
            "up" => self.aim -= n,
            "down" => self.aim += n,
            _ => (),
        }
    }
}

fn part1(readings: &[(&str, isize)]) -> isize {
    let mut state = Position::new();
    for reading in readings {
        state.update(reading.0, reading.1);
    }
    state.x * state.depth
}

fn part2(readings: &[(&str, isize)]) -> isize {
    let mut state = Position::new();
    for reading in readings {
        state.update2(reading.0, reading.1);
    }
    state.x * state.depth
}

fn main() {
    let input = include_str!("../inputs/day2.txt");
    let input: Vec<(&str, isize)> = input
        .trim()
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(direction, n)| (direction, n.parse::<isize>().unwrap()))
        .collect();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
forward 5
down 5
forward 8
up 3
down 8
forward 2
"#;

    #[test]
    fn given_part1_input() {
        let input: Vec<(&str, isize)> = INPUT
            .trim()
            .lines()
            .map(|line| line.split_once(" ").unwrap())
            .map(|(direction, n)| (direction, n.parse::<isize>().unwrap()))
            .collect();
        assert_eq!(part1(&input), 150);
    }

    #[test]
    fn given_part2_input() {
        let input: Vec<(&str, isize)> = INPUT
            .trim()
            .lines()
            .map(|line| line.split_once(" ").unwrap())
            .map(|(direction, n)| (direction, n.parse::<isize>().unwrap()))
            .collect();
        assert_eq!(part2(&input), 900);
    }
}
