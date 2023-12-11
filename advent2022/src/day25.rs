//! Solution to [AoC 2022 Day 25](https://adventofcode.com/2022/day/25)

use anyhow::Result;
use std::fmt::Display;
use std::iter::Sum;
use std::str::FromStr;

fn most_significant_place(num: usize) -> usize {
    // The sum of (2 * 5^k) for k=0 to k=n is (5^(n+1) - 1) / 2
    // The most significant place (5^n) for a given number will be the lowest n such that
    // (5^(n+1) - 1) / 2 >= number --> log5 (2 * number + 1) - 1 <= n
    (2.0 * num as f64 + 1.0).log(5.0).ceil() as usize - 1
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Snafu {
    decimal: isize,
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // We'll only support positive integers. Fuel requirements can't be negative anyway :)
        assert!(self.decimal >= 0);

        let mut remaining = self.decimal;
        let highest_place = most_significant_place(remaining as usize);

        // Calculate each "digit" from the most significant to the least significant and repeatedly
        // subtract out the magnitude provided by that digit until we're left with 0.
        for i in (0..=highest_place).rev() {
            let place_value = 5_isize.pow(i as u32);
            // We need to pick the digit that leaves a remaining value within the reachable range
            // of the rest of the smaller places (±limit ≡ ±(5^n - 1) / 2).
            // remaining - digit * power > -limit
            // remaining - digit * power < limit
            // ∴ digit = remaining / power
            let digit = ((remaining as f64) / (place_value as f64)).round() as isize;
            remaining -= digit * place_value;
            write!(f, "{}", snafu_digit(digit).unwrap())?;
        }
        assert_eq!(remaining, 0);
        Ok(())
    }
}

impl FromStr for Snafu {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut decimal = 0;
        for c in s.chars() {
            decimal = decimal * 5 + parse_snafu_digit(c)?;
        }
        Ok(Snafu { decimal })
    }
}

impl Sum for Snafu {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Snafu {
            decimal: iter.map(|i| i.decimal).sum(),
        }
    }
}

fn snafu_digit(c: isize) -> Result<char> {
    Ok(match c {
        2 => '2',
        1 => '1',
        0 => '0',
        -1 => '-',
        -2 => '=',
        _ => anyhow::bail!("invalid char: {c}"),
    })
}

fn parse_snafu_digit(c: char) -> Result<isize> {
    Ok(match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => anyhow::bail!("invalid char: {c}"),
    })
}

fn part1(input: &[Snafu]) -> String {
    input.iter().copied().sum::<Snafu>().to_string()
}

fn parse_input(input: &str) -> Vec<Snafu> {
    input
        .trim()
        .lines()
        .map(|i| Snafu::from_str(i).unwrap())
        .collect()
}

#[cfg(not(test))]
fn main() {
    let input = include_str!("../inputs/day25.txt");
    let input = parse_input(input);

    println!("Part 1: {}", part1(&input));
    // Day 25 has no part 2
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
    "#;

    #[test]
    fn given_part1_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part1(&input), "2=-1=0");
    }

    const TEST_MAPPING: &str = r#"
1=-0-2     1747
 12111      906
  2=0=      198
    21       11
  2=01      201
   111       31
 20012     1257
   112       32
 1=-1=      353
  1-12      107
    12        7
    1=        3
   122       37"#;

    #[test]
    fn parsing() {
        for line in TEST_MAPPING.trim().lines() {
            let mut tokens = line.trim().split_whitespace();
            let snafu = tokens.next().unwrap().parse::<Snafu>().unwrap();
            let decimal = tokens.next().unwrap().parse::<isize>().unwrap();
            assert_eq!(snafu.decimal, decimal);
        }
    }

    #[test]
    fn printing() {
        for line in TEST_MAPPING.trim().lines() {
            let mut tokens = line.trim().split_whitespace();
            let snafu_str = tokens.next().unwrap();
            let decimal = tokens.next().unwrap().parse::<isize>().unwrap();
            assert_eq!(snafu_str, Snafu { decimal }.to_string());
        }
    }
}
