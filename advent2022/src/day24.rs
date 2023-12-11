//! Solution to [AoC 2022 Day 24](https://adventofcode.com/2022/day/24)

use anyhow::Result;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

type Turn = u32;

/// An (x, y) coordinate
// Using small integer types since A* is memory inefficient. It doesn't really matter anymore, but
// this was an unsuccessful attempt to keep my initial totally naive A* solution from blowing up in
// memory
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(u8, u8);

impl Point {
    fn shift(self, direction: Direction) -> Self {
        let Point(col, row) = self;
        match direction {
            Direction::Up => Point(col, row.checked_sub(1).unwrap()),
            Direction::Down => Point(col, row.checked_add(1).unwrap()),
            Direction::Left => Point(col.checked_sub(1).unwrap(), row),
            Direction::Right => Point(col.checked_add(1).unwrap(), row),
        }
    }
}

fn manhattan_distance(a: Point, b: Point) -> u16 {
    u16::from(a.0.abs_diff(b.0)) + u16::from(a.1.abs_diff(b.1))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const fn sign(self) -> isize {
        match self {
            Direction::Up | Direction::Left => -1,
            Direction::Down | Direction::Right => 1,
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self> {
        Ok(match c {
            'v' => Self::Down,
            '^' => Self::Up,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => anyhow::bail!("invalid direction: {c}"),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Blizzard {
    /// Location at t=0
    loc: Point,
    direction: Direction,
}

#[derive(Debug, Clone)]
struct Maze {
    start: Point,
    end: Point,
    /// Including walls
    n_cols: u8,
    /// Including walls
    n_rows: u8,
    // TODO: we should represent these as 1-d `Vec`s to remove one layer of indirection, but in
    // practice, this is only read from once to create a `BlizzardMap` and then never read again
    /// List of left- and right-facing blizzards in each row
    left_right_blizzards: Vec<Vec<Blizzard>>,
    /// List of up- and down-facing blizzards in each column
    up_down_blizzards: Vec<Vec<Blizzard>>,
}

impl Maze {
    /// Helper function to do wrapping arithmetic of current + delta where numbers wrap within the
    /// given bounds
    fn wrap(current: u8, delta: isize, left_bound: u8, right_bound: u8) -> u8 {
        assert!(right_bound > left_bound);
        let n_spaces = right_bound - left_bound + 1;
        u8::try_from(
            (isize::try_from(current).unwrap() + delta - isize::try_from(left_bound).unwrap())
                .rem_euclid(isize::try_from(n_spaces).unwrap()),
        )
        .unwrap()
            + left_bound
    }

    fn lr_blizzards_in_row(&self, row: u8, turn: Turn) -> impl Iterator<Item = u8> + '_ {
        self.left_right_blizzards[usize::from(row)]
            .iter()
            .map(move |blizzard| {
                Self::wrap(
                    blizzard.loc.0,
                    blizzard.direction.sign() * isize::try_from(turn).unwrap(),
                    1,
                    self.n_cols - 2,
                )
            })
    }

    fn ud_blizzards_in_col(&self, col: u8, turn: Turn) -> impl Iterator<Item = u8> + '_ {
        self.up_down_blizzards[usize::from(col)]
            .iter()
            .map(move |blizzard| {
                Self::wrap(
                    blizzard.loc.1,
                    blizzard.direction.sign() * isize::try_from(turn).unwrap(),
                    1,
                    self.n_rows - 2,
                )
            })
    }
}

/// Node for A* search
#[derive(Debug, Clone, Copy)]
struct Node {
    loc: Point,
    turn: Turn,
    trips_remaining: u8,
    // Store cost in node to save repeated calculations
    cost: u16,
}

impl Node {
    /// Create an A* node
    /// `trips_remaining` is the number of whole, *not started* trips remaining where a trip is from
    /// either start->end or end->start.
    /// For example:
    /// * Part 1 has one (immediately started) trip from start->end, so `trips_remaining` = 0
    /// * Part 2 has three trips: start->end (immediately started), end->start, start->end, so
    /// `trips_remaining` = 2
    fn new(loc: Point, turn: Turn, start: Point, end: Point, trips_remaining: u8) -> Self {
        let next_goal = if trips_remaining % 2 == 0 { end } else { start };
        // A* cost = distance traveled (turn) + lower bound estimate of remaining distance to final
        // goal
        let cost = u16::try_from(turn).unwrap()
            + manhattan_distance(start, end) * u16::from(trips_remaining)
            + manhattan_distance(loc, next_goal);
        Self {
            loc,
            turn,
            trips_remaining,
            cost,
        }
    }

    /// A contrived helper function to copy a node to a new location for the next turn
    fn copy_to(self, new_loc: Point, start: Point, end: Point) -> Self {
        let trips_remaining = self.trips_remaining;
        let next_goal = if trips_remaining % 2 == 0 { end } else { start };
        let turn = self.turn + 1;
        // A* cost = distance traveled (turn) + lower bound estimate of remaining distance to final
        // goal
        let cost = u16::try_from(turn).unwrap()
            + manhattan_distance(start, end) * u16::from(self.trips_remaining)
            + manhattan_distance(new_loc, next_goal);
        Self {
            loc: new_loc,
            turn,
            trips_remaining,
            cost,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// This node is used with std::collections::BinaryHeap, a max-heap, so we reverse the ordering of
// the cost so that we pop off the node with the smallest cost.
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.cost.cmp(&other.cost) {
            ord @ (Ordering::Less | Ordering::Greater) => ord.reverse(),
            // As a tiebreaker, prefer larger turns over smaller turn so that we tend to expand
            // nodes at the fringes.
            Ordering::Equal => self.turn.cmp(&other.turn),
        }
    }
}

/// Map of all blizzards, which can be used to look up where blizzards are on a given turn
struct BlizzardMap {
    /// Map of (turn, row) -> set of columns where left/right blizzards are present
    lr_blizzards_per_row_per_turn: HashMap<(Turn, u8), HashSet<u8>>,
    /// Map of (turn, columns) -> set of rows where up/down blizzards are present
    ud_blizzards_per_col_per_turn: HashMap<(Turn, u8), HashSet<u8>>,
    n_rows: u8,
    n_cols: u8,
}

impl BlizzardMap {
    fn new(maze: &Maze) -> Self {
        let mut lr_blizzards_per_row_per_turn: HashMap<(Turn, u8), HashSet<u8>> = HashMap::new();
        // left- and right-facing blizzards return to their start every (maze.n_cols - 2) turns
        for turn in 0..(Turn::from(maze.n_cols) - 2) {
            for row in 1..(maze.n_rows - 1) {
                lr_blizzards_per_row_per_turn
                    .insert((turn, row), maze.lr_blizzards_in_row(row, turn).collect());
            }
        }

        let mut ud_blizzards_per_col_per_turn: HashMap<(Turn, u8), HashSet<u8>> = HashMap::new();
        // up- and down-facing blizzards return to their start every (maze.n_rows - 2) turns
        for turn in 0..(Turn::from(maze.n_rows) - 2) {
            for col in 1..(maze.n_cols - 1) {
                ud_blizzards_per_col_per_turn
                    .insert((turn, col), maze.ud_blizzards_in_col(col, turn).collect());
            }
        }

        Self {
            lr_blizzards_per_row_per_turn,
            ud_blizzards_per_col_per_turn,
            n_rows: maze.n_rows - 2,
            n_cols: maze.n_cols - 2,
        }
    }

    /// Return whether there will be a blizzard at the given location on the given turn
    fn has_blizzard(&self, Point(col, row): Point, turn: Turn) -> bool {
        if row == 0 || col == 0 || row > self.n_rows || col > self.n_cols {
            return false;
        }
        self.lr_blizzards_per_row_per_turn[&(turn % Turn::from(self.n_cols), row)].contains(&col)
            || self.ud_blizzards_per_col_per_turn[&(turn % Turn::from(self.n_rows), col)]
                .contains(&row)
    }
}

/// Wrapper around a HashSet to make checking whether a node has been visited more ergonomic
struct VisitedCache {
    cache: HashSet<(Point, u8, Turn)>,
    /// The number of unique blizzard states, which repeats every `n_unique_turns` turns
    n_unique_turns: Turn,
}

impl VisitedCache {
    fn new(n_unique_turns: Turn) -> Self {
        Self {
            cache: HashSet::new(),
            n_unique_turns,
        }
    }

    /// Insert a node into the cache and return whether the node was newly inserted
    ///
    /// We only need to store the number of turns modulo the number of unique blizzard states
    fn insert(&mut self, loc: Point, turn: Turn, trips_remaining: u8) -> bool {
        self.cache
            .insert((loc, trips_remaining, turn.rem_euclid(self.n_unique_turns)))
    }

    /// Return whether a node is in the cache
    ///
    /// We only need to store the number of turns modulo the number of unique blizzard states
    fn contains(&self, loc: Point, turn: Turn, trips_remaining: u8) -> bool {
        self.cache
            .contains(&(loc, trips_remaining, turn.rem_euclid(self.n_unique_turns)))
    }
}

/// Helper function to run A* for parts 1 and 2
// I tried A* (to learn what it was) and dynamic approaching approaches. Both worked for part 1 but
// A* was easier to adapt to part 2. The key insight for either approach is that blizzard positions
// repeat on a fixed interval so we can prune out nodes that we have already visited with a given
// blizzard position.
fn helper(maze: &Maze, trips_remaining: u8) -> Turn {
    let mut pqueue = BinaryHeap::from([Node::new(
        maze.start,
        0,
        maze.start,
        maze.end,
        trips_remaining,
    )]);

    let blizzard_map = BlizzardMap::new(maze);
    // The number of unique blizzard positions. We can take the current turn % n_unique_turns to
    // find an identifier for the current blizzard positions.
    let n_unique_turns = num::integer::lcm(blizzard_map.n_rows, blizzard_map.n_cols);
    let mut visited = VisitedCache::new(Turn::from(n_unique_turns));

    // Pop the node with the smallest cost
    while let Some(next) = pqueue.pop() {
        if !visited.insert(next.loc, next.turn, next.trips_remaining) {
            continue;
        }

        // Special case returning to the start if that's our current goal
        if next.loc == Point(maze.start.0, maze.start.1 + 1) && next.trips_remaining % 2 == 1 {
            pqueue.push(Node::new(
                maze.start,
                next.turn + 1,
                maze.start,
                maze.end,
                next.trips_remaining - 1,
            ));
        } else if next.loc == Point(maze.end.0, maze.end.1 - 1) && next.trips_remaining % 2 == 0 {
            // End condition. A* guarantees that we will take the optimal path to the end so we can
            // return immediately
            if next.trips_remaining == 0 {
                return next.turn + 1;
            }
            // Special case entering the end square if that's our current goal
            pqueue.push(Node::new(
                maze.end,
                next.turn + 1,
                maze.start,
                maze.end,
                next.trips_remaining - 1,
            ));
        }

        // Add a node for each possible, unvisited move

        // Stay put
        if !blizzard_map.has_blizzard(next.loc, next.turn + 1)
            && !visited.contains(next.loc, next.turn + 1, next.trips_remaining)
        {
            pqueue.push(Node::new(
                next.loc,
                next.turn + 1,
                maze.start,
                maze.end,
                next.trips_remaining,
            ));
        }

        // Move left
        if next.loc.0 > 1
            && next.loc.1 > 0
            && next.loc.1 < maze.n_rows - 1
            && !blizzard_map.has_blizzard(next.loc.shift(Direction::Left), next.turn + 1)
            && !visited.contains(
                next.loc.shift(Direction::Left),
                next.turn + 1,
                next.trips_remaining,
            )
        {
            pqueue.push(next.copy_to(next.loc.shift(Direction::Left), maze.start, maze.end));
        }

        // Move right
        if next.loc.0 < maze.n_cols - 2
            && next.loc.1 > 0
            && next.loc.1 < maze.n_rows - 1
            && !blizzard_map.has_blizzard(next.loc.shift(Direction::Right), next.turn + 1)
            && !visited.contains(
                next.loc.shift(Direction::Right),
                next.turn + 1,
                next.trips_remaining,
            )
        {
            pqueue.push(next.copy_to(next.loc.shift(Direction::Right), maze.start, maze.end));
        }

        // Move up
        if next.loc.1 > 1
            && !blizzard_map.has_blizzard(next.loc.shift(Direction::Up), next.turn + 1)
            && !visited.contains(
                next.loc.shift(Direction::Up),
                next.turn + 1,
                next.trips_remaining,
            )
        {
            pqueue.push(next.copy_to(next.loc.shift(Direction::Up), maze.start, maze.end));
        }

        // Move down
        if next.loc.1 < maze.n_rows - 2
            && !blizzard_map.has_blizzard(next.loc.shift(Direction::Down), next.turn + 1)
            && !visited.contains(
                next.loc.shift(Direction::Down),
                next.turn + 1,
                next.trips_remaining,
            )
        {
            pqueue.push(next.copy_to(next.loc.shift(Direction::Down), maze.start, maze.end));
        }
    }
    panic!("no path found");
}

fn part1(maze: &Maze) -> Turn {
    helper(maze, 0)
}

fn part2(maze: &Maze) -> Turn {
    helper(maze, 2)
}

fn parse_input(input: &str) -> Result<Maze> {
    let first_line = input.trim().lines().next().unwrap();
    let n_cols = first_line.len();
    let n_rows = input.trim().lines().count();
    let start_column = first_line
        .chars()
        .position(|c| c == '.')
        .unwrap()
        .try_into()?;
    let end_column = input
        .trim()
        .lines()
        .next_back()
        .unwrap()
        .chars()
        .position(|c| c == '.')
        .unwrap()
        .try_into()?;
    let mut out = Maze {
        start: Point(start_column, 0),
        end: Point(end_column, u8::try_from(n_rows - 1)?),
        n_cols: n_cols.try_into()?,
        n_rows: n_rows.try_into()?,
        left_right_blizzards: vec![Vec::with_capacity(n_cols); n_rows],
        up_down_blizzards: vec![Vec::with_capacity(n_rows); n_cols],
    };
    for (y, line) in input.trim().lines().enumerate().skip(1) {
        for (x, c) in line.chars().enumerate().skip(1) {
            if let Ok(direction) = Direction::try_from(c) {
                let blizzard = Blizzard {
                    loc: Point(x.try_into()?, y.try_into()?),
                    direction,
                };
                match direction {
                    Direction::Up | Direction::Down => out.up_down_blizzards[x].push(blizzard),
                    Direction::Left | Direction::Right => {
                        out.left_right_blizzards[y].push(blizzard);
                    }
                };
            }
        }
    }
    Ok(out)
}

#[cfg(not(test))]
fn main() {
    let input = include_str!("../inputs/day24.txt");
    let input = parse_input(input).unwrap();

    let (p1, p2) = rayon::join(|| part1(&input), || part2(&input));
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    const TEST_INPUT: &str = r#"
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
    "#;

    #[test]
    fn given_part1_input() {
        let input = parse_input(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 18);
    }

    #[test]
    fn given_part2_input() {
        let input = parse_input(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 54);
    }

    #[test]
    fn up_down_blizzards() {
        let maze = parse_input(TEST_INPUT).unwrap();
        assert_eq!(maze.lr_blizzards_in_row(0, 0).count(), 0);
        assert_eq!(
            maze.lr_blizzards_in_row(1, 0).collect::<HashSet<u8>>(),
            HashSet::from([1, 2, 4, 6])
        );
        assert_eq!(
            maze.lr_blizzards_in_row(1, 1).collect::<HashSet<u8>>(),
            HashSet::from([2, 3, 3, 5])
        );
    }
}
