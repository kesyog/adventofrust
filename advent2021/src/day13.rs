//! Solution to [AoC 2021 Day 13](https://adventofcode.com/2021/day/13)

use std::collections::HashSet;

type Points = HashSet<(usize, usize)>;

#[derive(Debug, Clone, Copy)]
enum Fold {
    /// Horizontal folds fold up over y = payload
    Horizontal(usize),
    /// Vertical folds fold left over x = payload
    Vertical(usize),
}

/// Perform a fold
fn fold(mut points: Points, fold: Fold) -> Points {
    let mut new_points = HashSet::with_capacity(points.len());
    match fold {
        Fold::Horizontal(dim) => {
            for (x, y) in points.drain() {
                if y > dim {
                    new_points.insert((x, dim - (y - dim)));
                } else {
                    new_points.insert((x, y));
                }
            }
        }
        Fold::Vertical(dim) => {
            for (x, y) in points.drain() {
                if x > dim {
                    new_points.insert((dim - (x - dim), y));
                } else {
                    new_points.insert((x, y));
                }
            }
        }
    }
    new_points
}

fn part1(mut points: Points, folds: &[Fold]) -> usize {
    for f in folds.iter().take(1) {
        points = fold(points, *f);
    }
    points.len()
}

fn part2(mut points: Points, folds: &[Fold]) {
    for f in folds.iter() {
        points = fold(points, *f);
    }

    // Plot points
    let max_x = points.iter().map(|i| i.0).max().unwrap();
    let max_y = points.iter().map(|i| i.1).max().unwrap();
    let mut grid = vec![vec![' '; max_x + 1]; max_y + 1];
    for (x, y) in points.drain() {
        grid[y][x] = '#';
    }

    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

fn parse_input(input: &str) -> (Points, Vec<Fold>) {
    let (points_str, folds_str) = input.trim().split_once("\n\n").unwrap();
    let points: Points = points_str
        .split('\n')
        .map(|i| serde_scan::from_str_skipping::<(usize, usize)>(",", i).unwrap())
        .collect();
    let mut folds = Vec::new();
    for fold in utils::trim_and_split(folds_str, "\n") {
        let dim = utils::find_all_integers::<usize>(fold)[0];
        if fold.contains('x') {
            folds.push(Fold::Vertical(dim))
        } else {
            folds.push(Fold::Horizontal(dim))
        }
    }
    (points, folds)
}

fn main() {
    let input = include_str!("../inputs/day13.txt");
    let (points, folds) = parse_input(input);

    println!("Part 1: {}", part1(points.clone(), &folds));
    part2(points, &folds);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
    "#;

    #[test]
    fn given_part1_input() {
        let (points, folds) = parse_input(TEST_INPUT);
        assert_eq!(part1(points, &folds), 17);
    }
}
