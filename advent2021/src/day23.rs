//! Solution to [AoC 2021 Day 23](https://adventofcode.com/2021/day/23)
//! DFS with pruning of search tree

use anyhow::Result;
use arrayvec::ArrayVec;
use std::cmp::Reverse;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Copy, Clone, PartialEq, EnumIter, Eq)]
enum AmphipodKind {
    A,
    B,
    C,
    D,
}

impl AmphipodKind {
    const fn cost(self, n_moves: usize) -> usize {
        match self {
            AmphipodKind::A => n_moves,
            AmphipodKind::B => 10 * n_moves,
            AmphipodKind::C => 100 * n_moves,
            AmphipodKind::D => 1000 * n_moves,
        }
    }
}

impl TryFrom<char> for AmphipodKind {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::A),
            'B' => Ok(Self::B),
            'C' => Ok(Self::C),
            'D' => Ok(Self::D),
            _ => anyhow::bail!("Invalid character"),
        }
    }
}

#[derive(Debug, Clone)]
struct Room<const N: usize>(ArrayVec<AmphipodKind, N>);

impl<const N: usize> Room<N> {
    /// bottom-up order
    fn new(start: [AmphipodKind; N]) -> Self {
        Self(start.into())
    }

    fn full_of(&self, kind: AmphipodKind) -> bool {
        self.0.is_full() && self.0.iter().all(|&i| i == kind)
    }

    fn would_accept(&self, kind: AmphipodKind) -> bool {
        !self.0.is_full() && self.0.iter().all(|&i| i == kind)
    }

    // Return number of moves from hallway into room
    fn insert(&mut self, kind: AmphipodKind) -> Result<usize> {
        if !self.would_accept(kind) {
            anyhow::bail!("Invalid move of {:?} into room", kind);
        }
        let n_moves = self.0.capacity() - self.0.len();
        self.0.try_push(kind)?;
        Ok(n_moves)
    }

    // Return number of moves to move amphipod into hallway
    fn pop(&mut self) -> Option<(AmphipodKind, usize)> {
        let n_moves = 1 + self.0.capacity() - self.0.len();
        self.0.pop().map(|kind| (kind, n_moves))
    }

    fn peek(&self) -> Option<AmphipodKind> {
        self.0.iter().last().copied()
    }

    const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn iter(&self) -> impl Iterator<Item = &AmphipodKind> {
        self.0.iter().rev()
    }
}

#[derive(Debug, Clone)]
struct Burrow<const N: usize> {
    hallway: [Option<AmphipodKind>; 11],
    rooms: [Room<N>; 4],
}

impl<const N: usize> Burrow<N> {
    const VALID_HALLWAY_SLOT_IDXS: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];

    // Bottom-up order
    fn new(rooms: [[AmphipodKind; N]; 4]) -> Self {
        Self {
            rooms: rooms.map(Room::new),
            hallway: Default::default(),
        }
    }

    /// Return index into Self.hallway of the entrance cell outside the given room
    const fn entrance_idx(kind: AmphipodKind) -> usize {
        match kind {
            AmphipodKind::A => 2,
            AmphipodKind::B => 4,
            AmphipodKind::C => 6,
            AmphipodKind::D => 8,
        }
    }

    /// Return target room of the given type
    const fn room(&self, kind: AmphipodKind) -> &Room<N> {
        match kind {
            AmphipodKind::A => &self.rooms[0],
            AmphipodKind::B => &self.rooms[1],
            AmphipodKind::C => &self.rooms[2],
            AmphipodKind::D => &self.rooms[3],
        }
    }

    fn room_mut(&mut self, kind: AmphipodKind) -> &mut Room<N> {
        match kind {
            AmphipodKind::A => &mut self.rooms[0],
            AmphipodKind::B => &mut self.rooms[1],
            AmphipodKind::C => &mut self.rooms[2],
            AmphipodKind::D => &mut self.rooms[3],
        }
    }

    /// Estimate the remaining cost to solve the board. Guaranteed to not exceed the actual cost.
    fn estimate_remaining_cost(&self) -> usize {
        let mut estimate = 0;
        // Cost to move each amphipod in the hallway to the top of its target room
        for (i, pod) in self
            .hallway
            .iter()
            .enumerate()
            .filter_map(|(i, cell)| cell.as_ref().map(|cell| (i, cell)))
        {
            let entrance_idx = Self::entrance_idx(*pod);
            estimate += pod.cost(entrance_idx.abs_diff(i) + 2);
        }
        // Approximate cost to move amphipods from their current incorrect room to the correct room
        for source_room_id in AmphipodKind::iter() {
            let source_room = self.room(source_room_id);
            if source_room.would_accept(source_room_id) {
                // Source room doesn't contain misplaced amphipods
                continue;
            }
            for pod in source_room.iter().filter(|i| **i != source_room_id) {
                estimate += pod.cost(
                    Self::entrance_idx(source_room_id).abs_diff(Self::entrance_idx(*pod)) + 2,
                );
            }
        }

        estimate
    }

    /// For the given type of amphipod, see if there are any of that type in the hallway that can
    /// be directly moved into its target room and perform that operation if possible.
    /// Performs at most one such operation and returns the added cost of that operation.
    fn pack_room(&mut self, target_kind: AmphipodKind) -> Option<usize> {
        let target_entrance_idx = Self::entrance_idx(target_kind);
        // Search hallway to the left of the target room
        for i in (0..target_entrance_idx)
            .rev()
            .filter(|i| Self::VALID_HALLWAY_SLOT_IDXS.contains(i))
        {
            match self.hallway[i] {
                Some(pod) if pod == target_kind => {
                    self.hallway[i] = None;
                    return Some(pod.cost(
                        (target_entrance_idx - i) + self.room_mut(target_kind).insert(pod).unwrap(),
                    ));
                }
                Some(_) => break,
                None => continue,
            }
        }
        // Search hallway to the right of the target room
        for i in ((target_entrance_idx + 1)..self.hallway.len())
            .filter(|i| Self::VALID_HALLWAY_SLOT_IDXS.contains(i))
        {
            match self.hallway[i] {
                Some(pod) if pod == target_kind => {
                    self.hallway[i] = None;
                    return Some(pod.cost(
                        (i - target_entrance_idx) + self.room_mut(target_kind).insert(pod).unwrap(),
                    ));
                }
                Some(_) => break,
                None => continue,
            }
        }
        // Search other rooms
        for source_room_kind in AmphipodKind::iter() {
            if source_room_kind == target_kind {
                continue;
            }
            if self.room(source_room_kind).peek() != Some(target_kind) {
                continue;
            }
            let source_entrance_idx = Self::entrance_idx(source_room_kind);
            let min_entrance_idx = source_entrance_idx.min(target_entrance_idx);
            let max_entrance_idx = source_entrance_idx.max(target_entrance_idx);
            // Check if hallway is clear between rooms
            if (min_entrance_idx..max_entrance_idx).all(|i| self.hallway[i].is_none()) {
                let (pod, n_moves_pop) = self.room_mut(source_room_kind).pop().unwrap();
                debug_assert!(pod == target_kind);
                let n_moves_insert = self.room_mut(target_kind).insert(target_kind).unwrap();
                let n_moves_hallway = target_entrance_idx.abs_diff(source_entrance_idx);
                return Some(target_kind.cost(n_moves_pop + n_moves_hallway + n_moves_insert));
            }
        }
        None
    }

    /// Move amphipods directly into their target hallway, if possible, and return added cost
    fn pack_rooms(&mut self) -> usize {
        let mut added_cost = 0;
        loop {
            let mut changed = false;
            for kind in AmphipodKind::iter() {
                if self.room(kind).would_accept(kind) {
                    if let Some(op_cost) = self.pack_room(kind) {
                        added_cost += op_cost;
                        changed = true;
                    }
                }
            }
            if !changed {
                break;
            }
        }
        added_cost
    }

    /// Find all valid moves into hallway and their associated cost
    fn hallway_moves(&self) -> Vec<(Self, usize)> {
        // Amphipod won't move if it's in its filled target
        // Amphipod can't cross paths with another
        let mut moves = Vec::new();
        for room_kind in AmphipodKind::iter() {
            let entrance_idx = Self::entrance_idx(room_kind);
            let room = self.room(room_kind);
            // Can't move an amphipod out of a room if the room is empty or its already settled
            // into its target room
            if room.iter().all(|&i| i == room_kind) || room.is_empty() {
                continue;
            }

            // Add unobstructed hallway spots to the left of the source room
            for i in (0..entrance_idx)
                .rev()
                .filter(|i| Self::VALID_HALLWAY_SLOT_IDXS.contains(i))
                .take_while(|&i| self.hallway[i].is_none())
            {
                let mut new_burrow = self.clone();
                let (pod, n_moves_pop) = new_burrow.room_mut(room_kind).pop().unwrap();
                new_burrow.hallway[i] = Some(pod);
                let n_moves = n_moves_pop + entrance_idx - i;
                moves.push((new_burrow, pod.cost(n_moves)));
            }
            for i in ((entrance_idx + 1)..self.hallway.len())
                .filter(|i| Self::VALID_HALLWAY_SLOT_IDXS.contains(i))
                .take_while(|&i| self.hallway[i].is_none())
            {
                let mut new_burrow = self.clone();
                let (pod, n_moves_pop) = new_burrow.room_mut(room_kind).pop().unwrap();
                new_burrow.hallway[i] = Some(pod);
                let n_moves = n_moves_pop + i - entrance_idx;
                moves.push((new_burrow, pod.cost(n_moves)));
            }
        }
        moves
    }

    fn solved(&self) -> bool {
        AmphipodKind::iter().all(|kind| self.room(kind).full_of(kind))
    }
}

fn solve<const N: usize>(burrow: Burrow<N>) -> Option<usize> {
    let mut optimal: Option<usize> = None;
    let mut to_check: Vec<(Burrow<N>, usize)> = vec![(burrow, 0)];
    while let Some((mut board, cost)) = to_check.pop() {
        if let Some(optimal) = optimal {
            // Optimization: prune branches where the eventual cost can be shown to be
            if cost + board.estimate_remaining_cost() >= optimal {
                continue;
            }
        }
        let cost = cost + board.pack_rooms();
        if board.solved() {
            if cost < optimal.unwrap_or(usize::MAX) {
                optimal = Some(cost);
            }
            continue;
        }
        let mut new_moves = Vec::new();
        for (new_board, added_cost) in board.hallway_moves() {
            let new_cost = cost + added_cost;
            if optimal.unwrap_or(usize::MAX) <= new_cost {
                continue;
            }
            new_moves.push((new_board, new_cost));
        }
        new_moves.sort_unstable_by_key(|&(_, cost)| Reverse(cost));
        to_check.extend(new_moves);
    }
    optimal
}

/// For debugging. Track history.
fn _solve_verbose<const N: usize>(burrow: Burrow<N>) -> Option<usize> {
    let mut optimal: Option<usize> = None;
    let mut to_check: Vec<Vec<(Burrow<N>, usize)>> = vec![vec![(burrow, 0)]];
    while let Some(history) = to_check.pop() {
        let (mut board, cost) = history.iter().last().unwrap().clone();
        if let Some(optimal) = optimal {
            if cost + board.estimate_remaining_cost() >= optimal {
                continue;
            }
        }
        let pack_cost = board.pack_rooms();
        let cost = cost + pack_cost;
        if board.solved() {
            if cost < optimal.unwrap_or(usize::MAX) {
                optimal = Some(cost);
                println!("new optimal: {}", cost);
                for i in history {
                    println!("{:?}", i);
                }
                println!("{:?}", (board, pack_cost));
            }
            continue;
        }
        let mut new_moves = Vec::new();
        for (new_board, added_cost) in board.hallway_moves() {
            let new_cost = cost + added_cost;
            if optimal.unwrap_or(usize::MAX) <= new_cost {
                continue;
            }
            let mut new_item = history.clone();
            new_item.push((new_board, new_cost));
            new_moves.push(new_item);
        }
        new_moves.sort_unstable_by_key(|history| Reverse(history.iter().last().unwrap().1));
        to_check.extend(new_moves);
    }
    optimal
}

fn part1(input: [[AmphipodKind; 2]; 4]) -> usize {
    let burrow = Burrow::new(input);
    solve(burrow).expect("No solution found")
}

fn part2(input: [[AmphipodKind; 2]; 4]) -> usize {
    // This is in the order given in the problem, but the Burrow constructor expects bottom-up
    // order
    let insertion = [
        [AmphipodKind::D, AmphipodKind::D],
        [AmphipodKind::C, AmphipodKind::B],
        [AmphipodKind::B, AmphipodKind::A],
        [AmphipodKind::A, AmphipodKind::C],
    ];
    let mut burrow = [[AmphipodKind::A; 4]; 4];
    for i in 0..4 {
        burrow[i] = [input[i][0], insertion[i][1], insertion[i][0], input[i][1]];
    }
    let burrow = Burrow::new(burrow);
    solve(burrow).expect("No solution found")
}

fn parse_input(input: &str) -> [[AmphipodKind; 2]; 4] {
    let pods: Vec<AmphipodKind> = input
        .trim()
        .chars()
        .flat_map(AmphipodKind::try_from)
        .collect();
    let mut result = [[AmphipodKind::A; 2]; 4];
    for i in 0..4 {
        result[i] = [pods[i + 4], pods[i]];
    }
    result
}

#[cfg(not(test))]
fn main() {
    let input = include_str!("../inputs/day23.txt");
    let input = parse_input(input);

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
    "#;

    #[test]
    fn given_part1_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part1(input), 12521);
    }

    #[test]
    fn given_part2_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part2(input), 44169);
    }
}
