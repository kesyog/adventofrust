//! Solution to [AoC 2018 Day 10](https://adventofcode.com/2018/day/10)

use std::cmp;
use std::convert::{TryFrom, TryInto};
use std::default::Default;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
struct Point {
    pub x: i32,
    pub y: i32,
    dx: i32,
    dy: i32,
}

impl Point {
    fn new(x: i32, y: i32, dx: i32, dy: i32) -> Self {
        Self { x, y, dx, dy }
    }

    fn step(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }

    fn step_back(&mut self) {
        self.x -= self.dx;
        self.y -= self.dy;
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Limits {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Limits {
    fn update(&mut self, point: &Point) {
        self.min_x = cmp::min(self.min_x, point.x);
        self.min_y = cmp::min(self.min_y, point.y);
        self.max_x = cmp::max(self.max_x, point.x);
        self.max_y = cmp::max(self.max_y, point.y);
    }
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            min_x: i32::MAX,
            max_x: i32::MIN,
            min_y: i32::MAX,
            max_y: i32::MIN,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Points {
    points: Vec<Point>,
    limits: Limits,
}

impl Points {
    fn new() -> Self {
        Self {
            points: Vec::new(),
            limits: Default::default(),
        }
    }

    fn push(&mut self, point: Point) {
        self.points.push(point);
        self.limits.update(&point);
    }

    fn step(&mut self) {
        self.limits = Default::default();
        for point in &mut self.points {
            point.step();
            self.limits.update(&point);
        }
    }

    fn step_back(&mut self) {
        self.limits = Default::default();
        for point in &mut self.points {
            point.step_back();
            self.limits.update(&point);
        }
    }

    /// Number of points wide the bounding box around all points is
    fn width(&self) -> usize {
        usize::try_from(self.limits.max_x - self.limits.min_x).unwrap() + 1
    }

    /// Number of points high the bounding box around all points is
    fn height(&self) -> usize {
        usize::try_from(self.limits.max_y - self.limits.min_y).unwrap() + 1
    }

    /// Calculate the area of the bounding box around all the points
    fn bounding_area(&self) -> usize {
        self.width() * self.height()
    }
}

impl fmt::Display for Points {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (width, height) = (self.width(), self.height());
        let mut grid: Vec<Vec<char>> = Vec::with_capacity(height);
        let row_template: Vec<char> = [' '].iter().cycle().take(width).copied().collect();
        for _ in 0..height {
            grid.push(row_template.clone());
        }
        assert_eq!(grid.len(), height);
        for point in &self.points {
            let row: usize = (point.y - self.limits.min_y).try_into().unwrap();
            let col: usize = (point.x - self.limits.min_x).try_into().unwrap();
            grid[row][col] = '#';
        }
        for row in grid.iter() {
            let row_str: String = row.iter().collect();
            writeln!(f, "{}", row_str)?;
        }
        Ok(())
    }
}

/// Advance until the points start diverging and returns how many seconds (iterations) it took to
/// get there. `points` is left in the converged state and can be printed directly to see the
/// message.
///
/// Uses the size of the points bounding box to gauge whether points are converging or diverging.
fn converge_points(points: &mut Points) -> usize {
    for seconds_elapsed in 1.. {
        let prev_area = points.bounding_area();
        points.step();
        let new_area = points.bounding_area();
        if new_area > prev_area {
            points.step_back();
            return seconds_elapsed - 1;
        }
    }
    unreachable!();
}

fn parse(input: &str) -> Points {
    let mut points: Points = Points::new();
    for chunk in utils::find_all_integers::<i32>(input).chunks_exact(4) {
        if let [x, y, dx, dy] = chunk {
            points.push(Point::new(*x, *y, *dx, *dy));
        }
    }
    points
}

fn main() {
    let input = include_str!("../inputs/day10.txt");
    let mut points = parse(input);
    let elapsed = converge_points(&mut points);
    println!("Message:\n\n{}\n\nSeconds elapsed: {}", points, elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>
    "#;

    #[test]
    fn test_input() {
        let mut points = parse(TEST_INPUT);
        let elapsed = converge_points(&mut points);
        println!("Message:\n\n{}\n\nSeconds elapsed: {}", points, elapsed);
        // Need to run `cargo test -- --nocapture` to see output
    }
}
