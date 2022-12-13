//! Solution to [AoC 2022 Day 13](https://adventofcode.com/2022/day/13)

use anyhow::Error;
use itertools::{EitherOrBoth, Itertools};
use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Data {
    List(Vec<Data>),
    Integer(u32),
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Data::Integer(a), Data::Integer(b)) => a.partial_cmp(b),
            (Data::List(_), Data::Integer(int)) => {
                self.partial_cmp(&Data::List(vec![Data::Integer(*int)]))
            }
            (Data::Integer(int), Data::List(_)) => {
                Data::List(vec![Data::Integer(*int)]).partial_cmp(other)
            }
            (Data::List(list1), Data::List(list2)) => {
                for pair in list1.iter().zip_longest(list2) {
                    match pair {
                        EitherOrBoth::Both(i1, i2) => {
                            let ordering = i1.partial_cmp(i2).unwrap();
                            if !ordering.is_eq() {
                                return Some(ordering);
                            }
                        }
                        EitherOrBoth::Left(_) => return Some(Ordering::Greater),
                        EitherOrBoth::Right(_) => return Some(Ordering::Less),
                    };
                }
                Some(Ordering::Equal)
            }
        }
    }
}

// Shouldn't derive this since we have a custom PartialOrd implementation. Thanks Clippy.
impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// Could probably derive this, but defer to the custom PartialOrd implementation just in case
impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for Data {}

/// Parse a `Data::Integer` payload out of the input, returning the integer and the leftover bytes from the input.
fn parse_integer(input: &[u8]) -> (u32, &[u8]) {
    assert!(input[0].is_ascii_digit());
    let next = input
        .iter()
        .position(|c| !c.is_ascii_digit())
        .unwrap_or(input.len());
    let int = std::str::from_utf8(&input[..next])
        .unwrap()
        .parse()
        .unwrap();
    let rest = &input[next..];
    (int, rest)
}

/// Parse a `Data::List` payload out of the input, returning the list and the leftover bytes from the input.
fn parse_list(mut input: &[u8]) -> (Vec<Data>, &[u8]) {
    let mut out = Vec::new();
    loop {
        if input[0] == b']' {
            break;
        }
        if input[0] == b',' {
            input = &input[1..];
        }
        let data;
        (data, input) = parse_data(input);
        out.push(data);
    }
    (out, input)
}

/// Parse a `Data` out of the input, returning the Data and the leftover bytes from the input.
// Stole the shape of this parser function from nom so probably could just nom
fn parse_data(input: &[u8]) -> (Data, &[u8]) {
    if input[0] == b'[' {
        let (list, mut rest) = parse_list(&input[1..]);
        // Skip closing brace
        rest = &rest[1..];
        (Data::List(list), rest)
    } else if input[0].is_ascii_digit() {
        let (integer, rest) = parse_integer(input);
        (Data::Integer(integer), rest)
    } else {
        panic!("Unexpected input: {}", std::str::from_utf8(input).unwrap());
    }
}

impl FromStr for Data {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Convert to bytes and assume ASCII for sanity
        // TODO: find crates like `substring` to allow for seamless switching between str and chars
        let (data, rest) = parse_data(s.trim().as_bytes());
        assert!(rest.is_empty());
        Ok(data)
    }
}

fn part1(input: &[(Data, Data)]) -> usize {
    let mut out = 0;
    // Packets are 1-indexed
    for (i, (packet1, packet2)) in (1..).zip(input) {
        if packet1 < packet2 {
            out += i;
        }
    }
    out
}

fn part2(input: &[(Data, Data)]) -> usize {
    let decoder1: Data = "[[2]]".parse().unwrap();
    let decoder2: Data = "[[6]]".parse().unwrap();

    let mut packets: Vec<Data> = input
        .iter()
        .flat_map(|(data1, data2)| [data1, data2])
        .cloned()
        .chain(std::iter::once(decoder1.clone()))
        .chain(std::iter::once(decoder2.clone()))
        .collect();
    packets.sort_unstable();

    // Packets are 1-indexed
    (packets.iter().position(|i| i == &decoder1).unwrap() + 1)
        * (packets.iter().position(|i| i == &decoder2).unwrap() + 1)
}

fn parse_input(input: &str) -> Vec<(Data, Data)> {
    let mut out = Vec::new();
    for block in utils::trim_and_split(input, "\n\n") {
        let (packet1, packet2) = block.split_once('\n').unwrap();
        out.push((packet1.parse().unwrap(), packet2.parse().unwrap()));
    }
    out
}

fn main() {
    let input = include_str!("../inputs/day13.txt");
    let input = parse_input(input);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
    "#;

    #[test]
    fn compare() {
        let data1: Data = "[1,1,3,1,1]".parse().unwrap();
        let data2: Data = "[1,1,5,1,1]".parse().unwrap();
        assert!(data1 < data2);
        let data1: Data = "[[1],[2,3,4]]".parse().unwrap();
        let data2: Data = "[[1],4]".parse().unwrap();
        assert!(data1 < data2);
        let data1: Data = "[[4,4],4,4]".parse().unwrap();
        let data2: Data = "[[4,4],4,4,4]".parse().unwrap();
        assert!(data1 < data2);
        let data1: Data = "[7,7,7,7]".parse().unwrap();
        let data2: Data = "[7,7,7]".parse().unwrap();
        assert!(data1 > data2);
        let data1: Data = "[]".parse().unwrap();
        let data2: Data = "[3]".parse().unwrap();
        assert!(data1 < data2);
        let data1: Data = "[[[]]]".parse().unwrap();
        let data2: Data = "[[]]".parse().unwrap();
        assert!(data1 > data2);
        let data1: Data = "[1,[2,[3,[4,[5,6,7]]]],8,9]".parse().unwrap();
        let data2: Data = "[1,[2,[3,[4,[5,6,0]]]],8,9]".parse().unwrap();
        assert!(data1 > data2);
    }

    #[test]
    fn given_part1_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn given_part2_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part2(&input), 140);
    }
}
