//! Solution to [AoC 2022 Day 10](https://adventofcode.com/2022/day/10)

use std::str::FromStr;

// Match specs for unit tests. Otherwise optimize for readability
#[cfg(not(test))]
const PIXEL_ON: char = '█';
#[cfg(test)]
const PIXEL_ON: char = '#';
#[cfg(not(test))]
const PIXEL_OFF: char = ' ';
#[cfg(test)]
const PIXEL_OFF: char = '.';

#[derive(Debug)]
enum OpCode {
    Addx(isize),
    Nop,
}

impl OpCode {
    // Execute command and return the new register value
    fn execute(&self, mut reg_x: isize) -> isize {
        match self {
            Self::Addx(step) => reg_x += step,
            Self::Nop => (),
        };

        reg_x
    }

    fn n_steps(&self) -> usize {
        match self {
            Self::Addx(_) => 2,
            Self::Nop => 1,
        }
    }
}

impl FromStr for OpCode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.trim().split(' ');
        let Some(op) = tokens.next() else {
            anyhow::bail!("No opcode");
        };
        match op {
            "addx" => Ok(Self::Addx(
                tokens
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("No argument"))?
                    .parse()?,
            )),
            "noop" => Ok(Self::Nop),
            _ => anyhow::bail!("Unknown opcode: {}", op),
        }
    }
}

fn part1(input: &[OpCode]) -> isize {
    let mut reg_x = 1;
    let mut cycle_counter = 1;
    let mut signal_checkpoints = [20_usize, 60, 100, 140, 180, 220].into_iter();
    let mut checkpoint = signal_checkpoints.next().unwrap();
    let mut signal_strengths = 0;

    for opcode in input {
        // Advance cycle counter to the start of the next cycle after the current opcode finishes
        // This is faster in isolation than iterating over each individual cycle
        // TODO(never): combine with part2 solution and solve both in one pass
        cycle_counter += opcode.n_steps();
        // Check if we passed one of the signal strength checkpoints
        // This assumes that we can't pass multiple checkpoints while processin one opcode, but the
        // max opcode cycle length is less than the gap between signal strength checkpoints so no
        // worries
        if cycle_counter > checkpoint {
            signal_strengths += checkpoint as isize * reg_x;
            match signal_checkpoints.next() {
                Some(val) => checkpoint = val,
                // No use processing more opcodes if we reached the last cycle count we care about
                None => break,
            };
        }
        reg_x = opcode.execute(reg_x);
    }
    signal_strengths
}

/// Check whether the given pixel overlaps the horizontal band occupied by the 3-pixel wide "sprite"
// Implementation is concise but definition is confusing so broken out to be testable
fn check_overlap(sprite_midpoint: isize, pixel_idx: usize) -> bool {
    // The pixels are arranged in rows of 40 pixels
    let horizontal_pixel_idx = pixel_idx as isize % 40;
    horizontal_pixel_idx.abs_diff(sprite_midpoint) <= 1
}

fn build_pixel_array(opcodes: &[OpCode]) -> [char; 40 * 6] {
    let mut reg_x = 1_isize;
    let mut pixel_idx = 0_usize;
    let mut crt = [PIXEL_OFF; 40 * 6];

    for opcode in opcodes {
        for _ in 0..opcode.n_steps() {
            if pixel_idx >= 240 {
                return crt;
            }
            if check_overlap(reg_x, pixel_idx) {
                crt[pixel_idx] = PIXEL_ON;
            }
            pixel_idx += 1;
        }
        reg_x = opcode.execute(reg_x);
    }

    crt
}

fn part2(input: &[OpCode]) -> String {
    let pixels = build_pixel_array(input);
    // Six rows of 40 characters + null-terminators
    let mut out = String::with_capacity(6 * (40 + 1));
    for row in pixels.chunks(40) {
        out.extend(row);
        out.push('\n');
    }
    out
}

fn parse_input(input: &str) -> Vec<OpCode> {
    input
        .trim()
        .lines()
        .map(|line| line.parse::<OpCode>().unwrap())
        .collect()
}

fn main() {
    let input = include_str!("../inputs/day10.txt");
    let input = parse_input(input);

    println!("Part 1: {}", part1(&input));
    println!("Part 2:\n{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
    "#;

    #[test]
    fn given_part1_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part1(&input), 13140);
    }

    #[test]
    fn given_part2_input() {
        let expected = r#"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."#;
        let input = parse_input(TEST_INPUT);
        assert_eq!(expected.trim(), part2(&input).trim());
    }

    #[test]
    fn test_overlap() {
        assert!(check_overlap(1, 0));
        assert!(check_overlap(1, 1));
        assert!(check_overlap(1, 2));
        assert!(!check_overlap(1, 3));
        // Pixel 2nd row
        assert!(check_overlap(1, 40));
        assert!(check_overlap(1, 41));
        assert!(check_overlap(1, 42));
        assert!(!check_overlap(1, 39));
        assert!(!check_overlap(1, 43));
        // Sprite partially off-screen left
        assert!(check_overlap(0, 0));
        assert!(check_overlap(0, 1));
        assert!(!check_overlap(0, 2));
        assert!(check_overlap(0, 40));
        assert!(check_overlap(0, 41));
        assert!(!check_overlap(0, 39));
        assert!(!check_overlap(0, 2));
        // Sprite a little more off-screen left
        assert!(check_overlap(-1, 0));
        assert!(!check_overlap(-1, 1));
        assert!(!check_overlap(-1, 39));
        assert!(check_overlap(-1, 40));
        assert!(!check_overlap(-1, 41));
        // Sprite partially off-screen right
        assert!(!check_overlap(40, 0));
        assert!(!check_overlap(40, 1));
        assert!(check_overlap(40, 39));
        assert!(!check_overlap(40, 38));
        assert!(!check_overlap(40, 40));
        assert!(!check_overlap(40, 41));
        // Sprite a little more off-screen right
        assert!(!check_overlap(41, 0));
        assert!(!check_overlap(41, 1));
        assert!(!check_overlap(41, 39));
        assert!(!check_overlap(41, 38));
        assert!(!check_overlap(41, 40));
        assert!(!check_overlap(41, 41));
    }
}
