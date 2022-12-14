//! Solution to [AoC 2022 Day 14](https://adventofcode.com/2022/day/14)

use std::collections::{HashMap, HashSet, VecDeque};

const START: Point = (500, 0);

type Point = (isize, isize);

struct Grid {
    set: HashSet<Point>,
    bottom_row: isize,
}

impl Grid {
    fn new(set: HashSet<Point>) -> Self {
        let bottom_row = set.iter().max_by_key(|(_x, y)| y).unwrap().1;
        Self { set, bottom_row }
    }
}

fn add_line(set: &mut HashSet<Point>, (x1, y1): Point, (x2, y2): Point) {
    if x1 == x2 {
        for y in y1.min(y2)..=y1.max(y2) {
            set.insert((x1, y));
        }
    } else if y1 == y2 {
        for x in x1.min(x2)..=x1.max(x2) {
            set.insert((x, y1));
        }
    }
}

fn children((x, y): Point) -> impl DoubleEndedIterator<Item = Point> {
    [(0, 1), (-1, 1), (1, 1)]
        .into_iter()
        .map(move |(dx, dy)| (x + dx, y + dy))
}

// After some reading on DFS, I realized we can solve this by doing a post-order DFS search,
// counting the number of nodes visited until we find a spot where sand will overflow. Each
// node has three children representing the three potential paths, though we prune paths that lead
// to rocks. We keep track of the number of nodes visited before we reach the bottom row of rocks.
// Iterative post-order DFS pseudocode stolen from https://www.geeksforgeeks.org/postorder-traversal-binary-tree-without-recursion-without-stack
// and adapted to work with a ternary tree.
fn part1(rocks: &Grid) -> usize {
    let mut node = START;
    let mut n_visited = 0;
    // Child -> parent map used both to keep track of parent-child relationships and to keep track
    // of visited nodes
    let mut parent_map: HashMap<Point, Point> = HashMap::new();

    while node.1 < rocks.bottom_row {
        // Traverse to unvisited child nodes
        if let Some(neighbor) =
            children(node).find(|c| !rocks.set.contains(c) && !parent_map.contains_key(c))
        {
            parent_map.insert(neighbor, node);
            node = neighbor;
            continue;
        }
        // There are no more valid child nodes i.e sand will settle here
        n_visited += 1;
        // Backtrack to the parent node
        node = parent_map[&node];
    }
    n_visited
}

// Use BFS to find all the squares visitable from the start
// I stole this clever idea of using BFS from someone else. My initial solution was just a naive
// 1:1 simulation of the problem.
fn part2(mut occupied: Grid) -> usize {
    let floor = occupied.bottom_row + 2;

    let mut queue = VecDeque::new();
    queue.push_back(START);

    let mut n_visited = 0;
    while let Some(next) = queue.pop_front() {
        if occupied.set.contains(&next) {
            continue;
        }
        n_visited += 1;
        occupied.set.insert(next);
        queue.extend(children(next).filter(|p| p.1 < floor));
    }
    n_visited
}

fn parse_point(s: &str) -> Point {
    let (s1, s2) = s.split_once(',').unwrap();
    (s1.parse().unwrap(), s2.parse().unwrap())
}

fn parse_input(input: &str) -> Grid {
    let mut set = HashSet::new();
    for line in input.trim().lines() {
        let mut iter_points = line.split(" -> ").peekable();
        while let Some(p) = iter_points.next() {
            let Some(next) = iter_points.peek() else {
                break;
            };
            add_line(&mut set, parse_point(p), parse_point(next));
        }
    }
    Grid::new(set)
}

fn main() {
    let input = include_str!("../inputs/day14.txt");
    let input = parse_input(input);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
    "#;

    #[test]
    fn given_part1_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part1(&input), 24);
    }

    #[test]
    fn given_part2_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part2(input), 93);
    }
}
