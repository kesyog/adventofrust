//! Solution to [AoC 2021 Day 9](https://adventofcode.com/2021/day/9)

use std::cmp::Ordering;

type NodeHandle = usize;

#[derive(Default, Debug, Clone, Copy)]
struct Node {
    prev: NodeHandle,
    next: NodeHandle,
}

/// Representation of the circle of marbles. This is represented by a doubly-linked list, and the
/// active marble is cached such that insertions and deletions performed relative to the active
/// marble can be done in O(n) time where n is the offset from the active marble. For this problem,
/// the offsets are always the same so insertions and deletions are performed in constant time.
#[derive(Debug)]
struct Circle {
    /// List of nodes where each node represents a marble. The index of each element indicates the
    /// value of that marble.
    nodes: Vec<Node>,
    active_idx: NodeHandle,
}

impl Circle {
    fn new(last_marble: usize) -> Self {
        Self {
            nodes: vec![Node::default(); last_marble + 1],
            active_idx: 0,
        }
    }

    /// Insert marble at the given relative clockwise position to the active marble. After
    /// insertion, the new marble will be at the given relative position to the active marble.
    fn insert(&mut self, offset: usize, value: usize) {
        let to_add = self.walk(self.active_idx, offset as isize);
        let prev = self.walk(to_add, -1);

        self.nodes[value] = Node { prev, next: to_add };
        self.nodes[prev].next = value;
        self.nodes[to_add].prev = value;
    }

    /// Remove the marble at the given position relative to the active marble.
    ///
    /// Positive offsets represent the clockwise direction
    /// Negative offsets represent the counter-clockwise direction
    fn remove(&mut self, offset: isize) -> usize {
        assert!(!self.nodes.is_empty());
        let to_remove = self.walk(self.active_idx, offset);
        let prev = self.walk(to_remove, -1);
        let next = self.walk(to_remove, 1);

        self.nodes[prev].next = next;
        self.nodes[next].prev = prev;
        to_remove
    }

    /// Set the marble at the given position relative to the current active marble as the new active marble.
    ///
    /// Positive offsets represent the clockwise direction
    /// Negative offsets represent the counter-clockwise direction
    fn set_active(&mut self, offset: isize) {
        self.active_idx = self.walk(self.active_idx, offset);
    }

    /// Get the index of the node at the given position relative to the given start node
    ///
    /// Positive offsets represent the clockwise direction
    /// Negative offsets represent the counter-clockwise direction
    fn walk(&self, start: NodeHandle, offset: isize) -> NodeHandle {
        let mut cursor = start;
        match offset.cmp(&0) {
            Ordering::Greater => {
                for _ in 0..offset {
                    cursor = self.nodes[cursor].next;
                }
            }
            Ordering::Less => {
                for _ in 0..-offset {
                    cursor = self.nodes[cursor].prev;
                }
            }
            Ordering::Equal => (),
        }
        cursor
    }
}

fn solve(n_players: usize, last_marble: usize) -> usize {
    let mut circle = Circle::new(last_marble);
    let mut scores = vec![0; n_players];
    for (marble, player) in (1_usize..=last_marble).zip((0_usize..n_players).cycle()) {
        if marble % 23 == 0 {
            scores[player] += marble;
            scores[player] += circle.remove(-7);
            // Set the marble clockwise of the removed marble as the new active marble
            circle.set_active(-6);
        } else {
            circle.insert(2, marble);
            circle.set_active(2);
        }
    }
    scores.into_iter().max().unwrap()
}

fn parse_input(input: &str) -> (usize, usize) {
    let nums = utils::find_all_integers(input);
    (nums[0], nums[1])
}

#[cfg(not(test))]
fn main() {
    let input = include_str!("../inputs/day9.txt");
    let (n_players, last_marble) = parse_input(input);

    println!("Part 1: {}", solve(n_players, last_marble));
    println!("Part 2: {}", solve(n_players, last_marble * 100));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_part1_input() {
        assert_eq!(solve(9, 25), 32);
        assert_eq!(solve(10, 1618), 8317);
        assert_eq!(solve(13, 7999), 146373);
        assert_eq!(solve(17, 1104), 2764);
        assert_eq!(solve(21, 6111), 54718);
        assert_eq!(solve(30, 5807), 37305);
    }
}
