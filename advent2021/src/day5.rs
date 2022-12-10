//! Solution to [AoC 2021 Day 5](https://adventofcode.com/2021/day/5)

use counter::Counter;
use std::cmp;

type Point = (isize, isize);
type Line = (Point, Point);

fn points((p1, p2): &Line, include_diag: bool) -> Vec<Point> {
    let mut points = Vec::new();
    let (p1, p2) = (cmp::min(p1, p2), cmp::max(p1, p2));
    if p1.0 == p2.0 {
        for y in p1.1..=p2.1 {
            points.push((p1.0, y));
        }
    } else if p1.1 == p2.1 {
        for x in p1.0..=p2.0 {
            points.push((x, p1.1));
        }
    } else if include_diag {
        let x_iter = p1.0..=p2.0;
        let y_iter = cmp::min(p1.1, p2.1)..=cmp::max(p1.1, p2.1);
        if p1.1 < p2.1 {
            for point in x_iter.zip(y_iter) {
                points.push(point);
            }
        } else {
            for point in x_iter.zip(y_iter.rev()) {
                points.push(point);
            }
        }
    }
    points
}

fn part1(lines: &[Line]) -> usize {
    let counts: Counter<_> = lines.iter().flat_map(|i| points(i, false)).collect();
    counts.into_iter().filter(|&(_, count)| count > 1).count()
}

fn part2(lines: &[Line]) -> usize {
    let counts: Counter<_> = lines.iter().flat_map(|i| points(i, true)).collect();
    counts.into_iter().filter(|&(_, count)| count > 1).count()
}

fn parse_input(input: &str) -> Vec<Line> {
    let mut points = Vec::new();
    for line in utils::trim_and_split(input, "\n") {
        let nums = utils::find_all_integers(line);
        points.push(((nums[0], nums[1]), (nums[2], nums[3])));
    }
    points
}

fn main() {
    let input = include_str!("../inputs/day5.txt");
    let lines = parse_input(input);

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
    "#;

    #[test]
    fn given_part1_input() {
        let lines = parse_input(TEST_INPUT);
        assert_eq!(part1(&lines), 5);
    }

    #[test]
    fn given_part2_input() {
        let lines = parse_input(TEST_INPUT);
        assert_eq!(part2(&lines), 12);
    }
}
