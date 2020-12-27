//! Solution to [AoC 2018 Day 6](https://adventofcode.com/2018/day/6)

use itertools::Itertools;
use nom::{character::complete::multispace0, combinator::iterator, sequence::delimited};
use std::collections::{HashMap, HashSet};
use std::iter::{Extend, IntoIterator, Iterator};
use utils::nom_parsers;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(input: (i32, i32)) -> Self {
        Self {
            x: input.0,
            y: input.1,
        }
    }

    fn manhattan_distance(&self, other: &Self) -> u32 {
        (self.x - other.x).abs() as u32 + (self.y - other.y).abs() as u32
    }

    fn total_manhattan_distance(&self, others: &[Self]) -> u32 {
        others
            .iter()
            .map(|other| other.manhattan_distance(self))
            .sum()
    }
}

#[allow(clippy::comparison_chain)]
/// Return a HashMap mapping a Point to the number of points in the given search space that it is
/// the exclusively closest point to. The HashMap will only contain points if the corresponding
/// count is non-zero.
fn count_closest_points<T>(search_space: T, points: &[Point]) -> HashMap<Point, u32>
where
    T: IntoIterator<Item = Point>,
{
    let mut counter: HashMap<Point, u32> = HashMap::with_capacity(points.len());

    for target in search_space {
        let mut min_distance = u32::MAX;
        let mut closest_points: Vec<Point> = Vec::new();
        for reference in points {
            let d = reference.manhattan_distance(&target);
            if d < min_distance {
                closest_points.clear();
                closest_points.push(*reference);
                min_distance = d;
            } else if d == min_distance {
                closest_points.push(*reference);
            }
        }
        assert!(!closest_points.is_empty());
        if closest_points.len() == 1 {
            *counter.entry(closest_points[0]).or_insert(0) += 1
        }
    }

    counter
}

fn part1(points: &[Point], bounds: &[Point; 2]) -> u32 {
    let (min_x, min_y) = (bounds[0].x, bounds[0].y);
    let (max_x, max_y) = (bounds[1].x, bounds[1].y);

    // Track which points have a non-infinite "closest coordinate" space. Pre-populate the list
    // with all points so the known infinite points can be filtered out later.
    let mut non_infinite: HashSet<Point> = HashSet::with_capacity(points.len());
    non_infinite.extend(points);

    // The closest point to any coordinate on the bounding box must have an infinite "closest
    // coordinate" space
    let infinite_points = count_closest_points(
        (min_x..=max_x)
            .cartesian_product(&[min_y, max_y])
            .map(|(x, &y)| Point::new((x, y)))
            .chain(
                (min_y + 1..max_y)
                    .cartesian_product(&[min_x, max_x])
                    .map(|(y, &x)| Point::new((x, y))),
            ),
        points,
    );
    for infinite_point in infinite_points.keys() {
        non_infinite.remove(infinite_point);
    }

    // Brute-force calculation of manhattan distances for each coordinate within the bounding box
    // (non-inclusive) vs. all the reference points
    let counter = count_closest_points(
        (min_x + 1..max_x)
            .cartesian_product(min_y + 1..max_y)
            .map(Point::new),
        points,
    );
    *counter
        .iter()
        .filter_map(|(k, v)| {
            if non_infinite.contains(k) {
                Some(v)
            } else {
                None
            }
        })
        .max()
        .unwrap()
}

fn part2(points: &[Point], target: usize, mean: &Point) -> usize {
    // When a point (x, y) is at the most extreme x value alowed by the target manhattan distance,
    // `target = n_points * |x - mean_x|` if we conservatively ignore the contribution in the y
    // direction. Thus, x is bounded by `mean_x ± target / n_points`. A similar argument can be
    // made in the y direction. Use this to constrain the search space.
    let delta = (target / points.len()) as i32;
    let x_range = mean.x - delta..=mean.x + delta;
    let y_range = mean.y - delta..=mean.y + delta;

    x_range
        .cartesian_product(y_range)
        .map(|pt| Point::new(pt).total_manhattan_distance(points))
        .filter(|&d| d < target as u32)
        .count()
}

// Use nom to parse the coordinates just to get a little familiar with it
fn parse_coordinates(input: &str) -> Vec<Point> {
    iterator(
        input,
        delimited(multispace0, nom_parsers::coordinates, multispace0),
    )
    .map(Point::new)
    .collect()
}

/// Calculates the bounding box of the given points as well as the mean of all of the
/// points and returns them as a tuple. The bounding box is represented as an array of the top left
/// and bottom right coordinates of the box.
fn calculate_basic_stats(points: &[Point]) -> ([Point; 2], Point) {
    let mut min_x = points[0].x;
    let mut max_x = min_x;
    let mut sum_x = min_x;

    let mut min_y = points[0].y;
    let mut max_y = min_y;
    let mut sum_y = min_y;
    for point in points[1..].iter() {
        sum_x += point.x;
        sum_y += point.y;
        if point.x < min_x {
            min_x = point.x
        }
        if point.x > max_x {
            max_x = point.x
        }
        if point.y < min_y {
            min_y = point.y
        }
        if point.y > max_y {
            max_y = point.y
        }
    }
    (
        [Point::new((min_x, min_y)), Point::new((max_x, max_y))],
        Point::new((sum_x / points.len() as i32, sum_y / points.len() as i32)),
    )
}

fn main() {
    let input = include_str!("../inputs/day6.txt");
    // Gratuitous use of nom for the sake of learning how nom parsers work
    let coordinates = parse_coordinates(input);
    let stats = calculate_basic_stats(&coordinates);

    println!("Part 1: {}", part1(&coordinates, &stats.0));
    println!("Part 2: {}", part2(&coordinates, 10000, &stats.1));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"1, 1
    1, 6
    8, 3
    3, 4
    5, 5
    8, 9
    "#;

    #[test]
    fn given_part1_input() {
        let coordinates = parse_coordinates(TEST_INPUT);
        assert_eq!(coordinates.len(), 6);
        let stats = calculate_basic_stats(&coordinates);
        assert_eq!(17, part1(&coordinates, &stats.0));
    }

    #[test]
    fn given_part2_input() {
        let coordinates = parse_coordinates(TEST_INPUT);
        assert_eq!(coordinates.len(), 6);
        let stats = calculate_basic_stats(&coordinates);
        assert_eq!(16, part2(&coordinates, 32, &stats.1));
    }
}
