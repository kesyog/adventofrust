//! Solution to [AoC 2021 Day 18](https://adventofcode.com/2021/day/18)

use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::{
    collections::VecDeque,
    mem,
    ops::{Add, Index, IndexMut},
};

mod parser {
    use super::Snailfish;
    use anyhow::{anyhow, bail, Result};
    use std::{iter::Peekable, str::FromStr};

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    enum LexToken {
        Paren,
        Number(isize),
    }

    impl FromStr for Snailfish {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            let lexed = lex(s)?;
            let (first, input) = lexed
                .split_first()
                .ok_or_else(|| anyhow!("Input too small: {:?}", lexed))?;
            if first != &LexToken::Paren {
                bail!("First character must be a bracket");
            }
            Ok(parse_pair(input)?.0)
        }
    }

    fn get_number<T: Iterator<Item = char>>(iter: &mut Peekable<T>) -> Result<isize> {
        let mut digits = String::new();
        while let Some(&c) = iter.peek() {
            if c.is_ascii_digit() || c == '-' {
                digits.push(c);
            } else {
                break;
            }
            iter.next();
        }
        Ok(digits.parse()?)
    }

    fn lex(input: &str) -> Result<Vec<LexToken>> {
        let mut chars = input.trim().chars().peekable();
        let mut result = Vec::with_capacity(input.len());
        while let Some(c) = chars.peek() {
            match c {
                '[' | ']' => {
                    result.push(LexToken::Paren);
                    chars.next();
                }
                x if x.is_ascii_digit() || *x == '-' => {
                    result.push(LexToken::Number(get_number(&mut chars)?));
                }
                ',' | ' ' => {
                    chars.next();
                }
                _ => panic!("Unexpected character"),
            }
        }
        Ok(result)
    }

    /// Parse a pair of numbers into a Snailfish::Pair, returning the pair and the remaining
    /// unparsed lex tokens. Expects the opening brace to have already been consumed.
    fn parse_pair(input: &[LexToken]) -> Result<(Snailfish, &[LexToken])> {
        let (first, rest) = match input.split_first() {
            Some((LexToken::Paren, rest)) => parse_pair(rest)?,
            Some((LexToken::Number(i), rest)) => (Snailfish::Single(*i), rest),
            None => bail!("bad input: {:?}", input),
        };
        let (second, rest) = match rest.split_first() {
            Some((LexToken::Paren, rest)) => parse_pair(rest)?,
            Some((LexToken::Number(i), rest)) => (Snailfish::Single(*i), rest),
            None => bail!("bad input: {:?}", rest),
        };
        // Split off end bracket
        let (end_bracket, rest) = rest
            .split_first()
            .ok_or_else(|| anyhow!("expected ]: {:?}", rest))?;
        if end_bracket != &LexToken::Paren {
            bail!("Unexpected char: {:?}. Expected ]", end_bracket);
        }
        Ok((Snailfish::Pair(Box::new([first, second])), rest))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn lex_test() {
            assert_eq!(
                lex("[123, -434]").unwrap(),
                vec![
                    LexToken::Paren,
                    LexToken::Number(123),
                    LexToken::Number(-434),
                    LexToken::Paren,
                ]
            );
            assert_eq!(
                lex("[123, [-434, 1000]]").unwrap(),
                vec![
                    LexToken::Paren,
                    LexToken::Number(123),
                    LexToken::Paren,
                    LexToken::Number(-434),
                    LexToken::Number(1000),
                    LexToken::Paren,
                    LexToken::Paren,
                ]
            );
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Snailfish {
    Single(isize),
    Pair(Box<[Snailfish; 2]>),
}

impl Snailfish {
    fn reduce(&mut self) {
        while self.explode().is_ok() || self.split().is_ok() {}
    }

    /// Find the index and value of the first regular number to the left of the given index
    fn first_regular_left(&self, base_index: &[Direction]) -> Option<(Vec<Direction>, isize)> {
        // To find the first regular number to the left:
        // Backtrack up index. Each time a Right is encountered, try going Left and DFS
        // right-first for a single value.
        let mut base_index = base_index.to_owned();
        let mut to_search: VecDeque<Vec<Direction>> = VecDeque::new();
        while let Some(dir) = base_index.pop() {
            if dir == Direction::Right {
                to_search.push_front([&base_index, [Direction::Left].as_slice()].concat());
            }
        }
        // Right-first DFS
        while let Some(index) = to_search.pop_back() {
            match self.get(&index) {
                Some(Self::Pair(_)) => {
                    to_search.push_back([&index, [Direction::Left].as_slice()].concat());
                    to_search.push_back([&index, [Direction::Right].as_slice()].concat());
                }
                Some(Self::Single(value)) => return Some((index.to_owned(), *value)),
                None => continue,
            }
        }
        None
    }

    /// Find the index and value of the first regular number to the right of the given index
    fn first_regular_right(&self, base_index: &[Direction]) -> Option<(Vec<Direction>, isize)> {
        // To find the first regular number to the right:
        // Backtrack up index. Each time a Left is encountered, try going Right and DFS
        // left-first for a single value.
        let mut base_index = base_index.to_owned();
        let mut to_search: VecDeque<Vec<Direction>> = VecDeque::new();
        while let Some(dir) = base_index.pop() {
            if dir == Direction::Left {
                to_search.push_front([&base_index, [Direction::Right].as_slice()].concat());
            }
        }
        // Left-first DFS
        while let Some(index) = to_search.pop_back() {
            match self.get(&index) {
                Some(Self::Pair(_)) => {
                    to_search.push_back([&index, [Direction::Right].as_slice()].concat());
                    to_search.push_back([&index, [Direction::Left].as_slice()].concat());
                }
                Some(Self::Single(value)) => return Some((index.to_owned(), *value)),
                None => continue,
            }
        }
        None
    }

    fn find_explodable_pair(&self) -> Option<Vec<Direction>> {
        // Find first explodable pair via left-first DFS
        let mut to_search = vec![vec![Direction::Right], vec![Direction::Left]];
        while let Some(index) = to_search.pop() {
            match self.get(&index) {
                Some(Self::Pair(_)) => {
                    if index.len() >= 4 {
                        return Some(index);
                    }
                    to_search.push([&index, [Direction::Right].as_slice()].concat());
                    to_search.push([&index, [Direction::Left].as_slice()].concat());
                }
                Some(Self::Single(_)) | None => continue,
            }
        }
        None
    }

    /// Find and perform the first valid explode action, returning `Ok` if successful or `Err` if no
    /// explode action is possible.
    fn explode(&mut self) -> Result<()> {
        let to_explode = self
            .find_explodable_pair()
            .ok_or_else(|| anyhow!("No explodable pairs"))?;
        let old_pair = match mem::replace(&mut self[&to_explode], Self::Single(0)) {
            Self::Pair(pair) => match *pair {
                [Self::Single(first), Self::Single(second)] => (first, second),
                _ => panic!("Exploding pair does not contain regular numbers"),
            },
            _ => panic!("trying to explode a single value"),
        };
        if let Some((idx, value)) = self.first_regular_left(&to_explode) {
            self[&idx] = Self::Single(value + old_pair.0);
        }
        if let Some((idx, value)) = self.first_regular_right(&to_explode) {
            self[&idx] = Self::Single(value + old_pair.1);
        }

        Ok(())
    }

    fn find_splittable(&self) -> Option<(Vec<Direction>, isize)> {
        // Find first regular number >= 10 pair via left-first DFS
        let mut to_search = vec![vec![Direction::Right], vec![Direction::Left]];
        while let Some(index) = to_search.pop() {
            match self.get(&index) {
                Some(Self::Pair(_)) => {
                    to_search.push([&index, [Direction::Right].as_slice()].concat());
                    to_search.push([&index, [Direction::Left].as_slice()].concat());
                }
                Some(Self::Single(value)) if *value >= 10 => return Some((index, *value)),
                _ => continue,
            }
        }
        None
    }

    /// Find and perform the first valid split action, returning `Ok` if successful or `Err` if no
    /// split action is possible.
    fn split(&mut self) -> Result<()> {
        let (idx, old_value) = self
            .find_splittable()
            .ok_or_else(|| anyhow!("No splittable numbers"))?;
        self[&idx] = Self::Pair(Box::new([
            Self::Single(old_value / 2),
            Self::Single(((old_value as f64) / 2.0 + 0.5) as isize),
        ]));

        Ok(())
    }

    fn magnitude(&self) -> isize {
        match self {
            Self::Single(i) => *i,
            Self::Pair(pair) => 3 * pair[0].magnitude() + 2 * pair[1].magnitude(),
        }
    }

    fn get(&self, index: &[Direction]) -> Option<&Self> {
        let mut cursor = self;
        for direction in index {
            cursor = match (cursor, direction) {
                (Self::Pair(pair), Direction::Left) => &pair[0],
                (Self::Pair(pair), Direction::Right) => &pair[1],
                _ => return None,
            }
        }
        Some(cursor)
    }

    fn get_mut(&mut self, index: &[Direction]) -> Option<&mut Self> {
        let mut cursor = self;
        for direction in index {
            cursor = match (cursor, direction) {
                (Self::Pair(pair), Direction::Left) => &mut pair[0],
                (Self::Pair(pair), Direction::Right) => &mut pair[1],
                _ => return None,
            }
        }
        Some(cursor)
    }
}

impl Index<&[Direction]> for Snailfish {
    type Output = Self;

    fn index(&self, index: &[Direction]) -> &Self::Output {
        self.get(index).expect("Index out of bounds")
    }
}

impl IndexMut<&[Direction]> for Snailfish {
    fn index_mut(&mut self, index: &[Direction]) -> &mut Self::Output {
        self.get_mut(index).expect("Index out of bounds")
    }
}

impl Add for Snailfish {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut sum = Self::Pair(Box::new([self, other]));
        sum.reduce();
        sum
    }
}

fn part1(input: Vec<Snailfish>) -> isize {
    input.into_iter().reduce(Add::add).unwrap().magnitude()
}

fn part2(input: Vec<Snailfish>) -> isize {
    input
        .into_iter()
        .permutations(2)
        .map(|i| i.into_iter().reduce(Add::add).unwrap().magnitude())
        .max()
        .unwrap()
}

fn parse_input(input: &str) -> Vec<Snailfish> {
    input
        .trim()
        .lines()
        .map(|i| i.parse::<Snailfish>().unwrap())
        .collect()
}

#[cfg(not(test))]
fn main() {
    let input = include_str!("../inputs/day18.txt");
    let input = parse_input(input);

    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
    "#;

    #[test]
    fn given_part1_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(4140, part1(input));
    }

    #[test]
    fn given_part2_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(3993, part2(input));
    }

    #[test]
    fn magnitude() {
        assert_eq!(29, "[9,1]".parse::<Snailfish>().unwrap().magnitude());
        assert_eq!(
            129,
            "[[9,1],[1,9]]".parse::<Snailfish>().unwrap().magnitude()
        );
        assert_eq!(
            143,
            "[[1,2],[[3,4],5]]"
                .parse::<Snailfish>()
                .unwrap()
                .magnitude()
        );
        assert_eq!(
            3488,
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
                .parse::<Snailfish>()
                .unwrap()
                .magnitude()
        );
    }

    #[test]
    fn explode() {
        let mut snailfish = "[[[[0,7],4],[15,[0,13]]],[1,1]]"
            .parse::<Snailfish>()
            .unwrap();
        assert!(snailfish.explode().is_err());

        snailfish = "[[[[[9,8],1],2],3],4]".parse::<Snailfish>().unwrap();
        snailfish.explode().unwrap();
        assert_eq!(snailfish, "[[[[0,9],2],3],4]".parse::<Snailfish>().unwrap());

        snailfish = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"
            .parse::<Snailfish>()
            .unwrap();
        snailfish.explode().unwrap();
        assert_eq!(
            snailfish,
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
                .parse::<Snailfish>()
                .unwrap()
        );

        snailfish = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
            .parse::<Snailfish>()
            .unwrap();
        snailfish.explode().unwrap();
        assert_eq!(
            snailfish,
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"
                .parse::<Snailfish>()
                .unwrap()
        );
    }

    #[test]
    fn split() {
        let mut snailfish = "[[[[0,7],4],[15,[0,13]]],[1,1]]"
            .parse::<Snailfish>()
            .unwrap();
        snailfish.split().unwrap();
        assert_eq!(
            snailfish,
            "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"
                .parse::<Snailfish>()
                .unwrap()
        );
        snailfish.split().unwrap();
        assert_eq!(
            snailfish,
            "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"
                .parse::<Snailfish>()
                .unwrap()
        );
    }
}
