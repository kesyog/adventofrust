//! Solution to [AoC 2022 Day 18](https://adventofcode.com/2022/day/18)

use std::collections::HashSet;

use itertools::Itertools;
use rayon::prelude::*;

type Point = [isize; 3];

/// Return the points that are adjacent to the given point
fn neighbors(point: Point) -> impl Iterator<Item = Point> {
    let moves = [
        [0, 0, 1],
        [0, 0, -1],
        [0, 1, 0],
        [0, -1, 0],
        [1, 0, 0],
        [-1, 0, 0],
    ];
    moves
        .map(|delta| {
            [
                point[0] + delta[0],
                point[1] + delta[1],
                point[2] + delta[2],
            ]
        })
        .into_iter()
}

/// Returns whether two points are adjacent
fn is_adjacent(a: Point, b: Point) -> bool {
    // Two points are adjacent if they have the same coordinates in two dimensions and the
    // coordinates in the remaining dimension differ by by one
    let mut equal_per_dim = [false; 3];
    let mut adjacent_per_dim = [false; 3];
    for dim in 0..3 {
        equal_per_dim[dim] = a[dim] == b[dim];
        adjacent_per_dim[dim] = a[dim].abs_diff(b[dim]) == 1;
    }
    // Use some bitwise math to count booleans:
    // 0 ^ 0 ^ 0 = 0
    // 1 ^ 0 ^ 0 = 1
    // 1 ^ 1 ^ 0 = 0
    // 1 ^ 1 ^ 1 = 1
    let two_dims_equal = !equal_per_dim.iter().copied().reduce(|a, b| a ^ b).unwrap()
        && equal_per_dim.iter().copied().reduce(|a, b| a | b).unwrap();
    let one_side_adjacent = adjacent_per_dim
        .iter()
        .copied()
        .reduce(|a, b| a ^ b)
        .unwrap()
        && !equal_per_dim.iter().copied().reduce(|a, b| a & b).unwrap();

    two_dims_equal && one_side_adjacent
}

/// Count adjacent points within the two sets of points
fn count_adjacencies<I, J>(set1: I, set2: J) -> usize
where
    I: IntoParallelIterator<Item = Point>,
    J: IntoParallelIterator<Item = Point>,
    J::Iter: Clone + Sync,
{
    let iter2 = set2.into_par_iter();
    set1.into_par_iter()
        .map(|p1| iter2.clone().filter(|&p2| is_adjacent(p1, p2)).count())
        .sum()
}

fn part1(lava: &HashSet<Point>) -> usize {
    // DFS traverse through set of lava points and count the number of neighbors seen
    let mut stack = vec![lava.iter().copied().next().unwrap()];
    let mut n_adjacencies = 0;
    let mut visited = HashSet::new();
    while let Some(next) = stack.pop() {
        visited.insert(next);
        for neighbor in neighbors(next) {
            if !lava.contains(&neighbor) || visited.contains(&neighbor) {
                continue;
            }
            n_adjacencies += 1;
            stack.push(neighbor);
        }
    }
    lava.len() * 6 - 2 * n_adjacencies
}

/// Returns tuple of answer for parts 1 and 2
fn both_parts(lava: &[Point]) -> (usize, usize) {
    let lava: HashSet<Point> = lava.iter().copied().collect();

    let total_surface_area = part1(&lava);

    // Find the minimum and maximum bounds of lava coordinates in each dimension
    let mut mins = [isize::MAX; 3];
    let mut maxs = [isize::MIN; 3];
    for p in &lava {
        for dim in 0..3 {
            mins[dim] = mins[dim].min(p[dim]);
            maxs[dim] = maxs[dim].max(p[dim]);
        }
    }

    // Expand lava bounds by one in each direction for each dimension to create a "shell" of known
    // external air points. Pick one of the points in this shell and use DFS to find all air points
    // connected to this shell.
    let mut external_air: HashSet<Point> = HashSet::new();
    let mut stack = vec![mins.map(|i| i - 1)];
    while let Some(next) = stack.pop() {
        external_air.insert(next);
        for neighbor in neighbors(next) {
            if lava.contains(&neighbor) || external_air.contains(&neighbor) {
                continue;
            }
            // Constrain to points within outer shell
            if (0..3).any(|dim| neighbor[dim] < mins[dim] - 1 || neighbor[dim] > maxs[dim] + 1) {
                continue;
            }
            stack.push(neighbor);
        }
    }

    // The set of all points, air and lava, within the bounds of the shell
    let all_points: HashSet<Point> = (0..3)
        .into_iter()
        .map(|dim| (mins[dim] - 1)..=(maxs[dim] + 1))
        .multi_cartesian_product()
        .map(|point| point.try_into().unwrap())
        .collect();

    // Find the set of all air points not connected to the outside air
    let internal_air = &(&all_points - &external_air) - &lava;

    // Count faces between internal air cubes and lava cubes
    let internal_adjacencies = count_adjacencies(internal_air, lava.par_iter().copied());

    (
        total_surface_area,
        total_surface_area - internal_adjacencies,
    )
}

fn parse_input(input: &str) -> Vec<Point> {
    let mut out = Vec::new();
    for line in input.trim().lines() {
        out.push(
            line.splitn(3, ',')
                .map(|s| s.parse().unwrap())
                .collect_vec()
                .try_into()
                .unwrap(),
        );
    }
    out
}

fn main() {
    let input = include_str!("../inputs/day18.txt");
    let input = parse_input(input);

    let answer = both_parts(&input);
    println!("Part 1: {}", answer.0);
    println!("Part 2: {}", answer.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
    "#;

    #[test]
    fn given_part1_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(both_parts(&input).0, 64);
    }

    #[test]
    fn given_part2_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(both_parts(&input).1, 58);
    }
}
