//! Solution to [AoC 2021 Day 4](https://adventofcode.com/2021/day/4)

use std::str::FromStr;
use std::{collections::HashSet, convert::TryInto};

use anyhow::anyhow;

fn find_first_winner(draws: &[u32], mut boards: Vec<Board>) -> Option<Board> {
    for &draw in draws {
        // Iterate over indices to make satisfying the borrow checker easier
        for i in 0..boards.len() {
            let board = &mut boards[i];
            if board.draw(draw) == BoardState::Bingo {
                return Some(boards.remove(i));
            }
        }
    }
    None
}

fn part1(draws: &[u32], boards: Vec<Board>) -> u32 {
    let winner = find_first_winner(draws, boards).expect("No winners");
    winner.score()
}

fn find_last_winner(draws: &[u32], mut boards: Vec<Board>) -> Option<Board> {
    for &draw in draws {
        // Iterate over indices to make satisfying the borrow checker easier
        for i in 0..boards.len() {
            let board = &mut boards[i];
            if board.draw(draw) == BoardState::Bingo
                && boards.iter().all(|i| i.state == BoardState::Bingo)
            {
                return Some(boards.remove(i));
            }
        }
    }
    None
}

fn part2(draws: &[u32], boards: Vec<Board>) -> u32 {
    let winner = find_last_winner(draws, boards).expect("No winners");
    winner.score()
}

#[derive(PartialOrd, PartialEq, Eq, Debug, Clone, Copy)]
enum BoardState {
    Unfilled,
    Bingo,
}

#[derive(Debug, Clone)]
struct Board {
    board: [u32; 25],
    indices_drawn: HashSet<usize>,
    last_drawn: Option<u32>,
    state: BoardState,
}

impl Board {
    fn draw(&mut self, n: u32) -> BoardState {
        self.last_drawn = Some(n);
        if let Some(idx) = self.board.iter().position(|&item| item == n) {
            self.indices_drawn.insert(idx);
        }

        if self.state == BoardState::Unfilled && self.bingo() {
            self.state = BoardState::Bingo
        }
        self.state
    }

    fn bingo(&self) -> bool {
        'cols: for col in 0..5 {
            for row in 0..5 {
                if !self.indices_drawn.contains(&Board::index_1d(row, col)) {
                    continue 'cols;
                }
            }
            return true;
        }

        'rows: for row in 0..5 {
            for col in 0..5 {
                if !self.indices_drawn.contains(&Board::index_1d(row, col)) {
                    continue 'rows;
                }
            }
            return true;
        }

        false
    }

    const fn index_1d(row: usize, col: usize) -> usize {
        col + 5 * row
    }

    fn score(&self) -> u32 {
        let all_indices: HashSet<usize> = (0..25).into_iter().collect();
        let indices_not_drawn = &all_indices - &self.indices_drawn;
        let undrawn_sum: u32 = indices_not_drawn.into_iter().map(|i| self.board[i]).sum();
        self.last_drawn.unwrap() * undrawn_sum
    }
}

impl FromStr for Board {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let board = utils::find_all_integers(s)
            .try_into()
            .map_err(|_| anyhow!("Parse error"))?;
        Ok(Self {
            board,
            indices_drawn: HashSet::new(),
            last_drawn: None,
            state: BoardState::Unfilled,
        })
    }
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<Board>) {
    let mut input = utils::trim_and_split(input, "\n\n");
    let draws: Vec<u32> = input
        .next()
        .unwrap()
        .split(',')
        .map(|i| i.parse::<u32>().unwrap())
        .collect();
    let boards: Vec<Board> = input.map(|i| i.parse::<Board>().unwrap()).collect();
    (draws, boards)
}

fn main() {
    let input = include_str!("../inputs/day4.txt");
    let (draws, boards) = parse_input(input);

    println!("Part 1: {}", part1(&draws, boards.clone()));
    println!("Part 2: {}", part2(&draws, boards));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
    "#;

    #[test]
    fn given_part1_input() {
        let (draws, boards) = parse_input(TEST_INPUT);
        assert_eq!(part1(&draws, boards), 4512);
    }

    #[test]
    fn given_part2_input() {
        let (draws, boards) = parse_input(TEST_INPUT);
        assert_eq!(part2(&draws, boards), 1924);
    }
}
