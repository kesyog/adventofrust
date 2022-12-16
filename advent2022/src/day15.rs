//! Solution to [AoC 2022 Day 15](https://adventofcode.com/2022/day/15)

use itertools::Itertools;

type Point = (isize, isize);
type Parsed = Vec<Sensor>;

fn manhattan_distance(a: Point, b: Point) -> isize {
    (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as isize
}

#[derive(Debug, Clone)]
struct Sensor {
    loc: Point,
    beacon: Point,
}

impl Sensor {
    fn new(loc: Point, beacon: Point) -> Self {
        Sensor { loc, beacon }
    }

    fn manhattan_distance(&self, other: Point) -> isize {
        manhattan_distance(self.loc, other)
    }

    fn radius(&self) -> isize {
        self.manhattan_distance(self.beacon)
    }

    /// Return bounds of the diamond-shaped region where there can be no other beacons other than
    /// the associated beacon
    fn beacon_keepout_bounds(&self) -> [Point; 4] {
        let man_radius = self.radius();
        [
            // Right
            (self.loc.0 + man_radius, self.loc.1),
            // Left
            (self.loc.0 - man_radius, self.loc.1),
            // Bottom
            (self.loc.0, self.loc.1 + man_radius),
            // Top
            (self.loc.0, self.loc.1 - man_radius),
        ]
    }

    /// Returns the left and right bounds of the slice of the keepout region for the given beacon in
    /// a given row
    fn beacon_keepout_bounds_on_row(&self, row: isize) -> Option<(isize, isize)> {
        if !self.constrains_row(row) {
            return None;
        }
        let [right, left, _bottom, _top] = self.beacon_keepout_bounds();
        let incut = left.1.abs_diff(row) as isize;
        Some((left.0 + incut, right.0 - incut))
    }

    /// Whether the sensor imparts any beacon constraints on the given row
    /// This is true iff the row is closer than the sensor's beacon
    fn constrains_row(&self, row: isize) -> bool {
        self.manhattan_distance((self.loc.0, row)) <= self.radius()
    }
}

fn part1(mut sensors: Parsed, row: isize) -> isize {
    sensors.sort_unstable_by_key(|sensor| {
        sensor
            .beacon_keepout_bounds_on_row(row)
            // Irrelevant sensors will be filtered out anyway, but throw them at the end of the
            // array. We'll probably never reach them since we scan left-to-right
            .map_or(isize::MAX, |(left_bound, _right_bound)| left_bound)
    });

    let mut count = 0;
    let mut x: Option<isize> = None;
    // Similar to part2 strategy, scan through intervals left to right, keeping track of the size
    // of the keepout area
    for sensor in &sensors {
        let Some((left_bound, right_bound)) = sensor.beacon_keepout_bounds_on_row(row) else {
            continue;
        };
        if x.is_none() {
            x = Some(left_bound);
        }
        if right_bound < x.unwrap() {
            continue;
        }
        if left_bound > x.unwrap() {
            x = Some(left_bound);
        }
        count += right_bound - x.unwrap() + 1;
        x = Some(right_bound + 1);
    }
    let n_beacons_in_row = sensors
        .iter()
        .filter_map(|sensor| {
            if sensor.beacon.1 == row {
                Some(sensor.beacon)
            } else {
                None
            }
        })
        .unique()
        .count() as isize;
    count - n_beacons_in_row
}

fn tuning_frequency(p: Point) -> isize {
    p.0 * 4_000_000 + p.1
}

// Idea:
// Scan each row to find if any points are outside every sensor's "no beacon possible" keepout area.
// There are too many points to check each point individually. We can do better by sorting the array
// of sensors by the start of their keepout interval from left to right and iterating over the
// intervals.
//
// I re-sorted the sensors for each row since the relative ordering may change. It actually ran
// faster and still gave a correct answer if I only sorted once, came up with a list of candidate
// solutions, and then rigorously re-checking each candidate against all the sensors. But that's
// even more janky.
fn part2(mut sensors: Parsed, search_bound: isize) -> isize {
    'row: for row in 0..=search_bound {
        let mut x = 0;
        // Tried cloning and pre-filtering the list before sorting but it ends up being slower
        sensors.sort_unstable_by_key(|sensor| {
            sensor
                .beacon_keepout_bounds_on_row(row)
                // Irrelevant sensors will be filtered out anyway, but throw them at the end of the
                // array. We'll probably never reach them since we scan left-to-right
                .map_or(isize::MAX, |(left_bound, _right_bound)| left_bound)
        });
        for sensor in &sensors {
            let Some((left_bound, right_bound)) = sensor.beacon_keepout_bounds_on_row(row) else {
                // This sensor doesn't constrain this row
                continue;
            };
            if x >= right_bound {
                // We've already skipped over this sensor's bounds in our left-to-right scan. They
                // must have been overlapped with a previous sensor's bounds. It's irrelevant now.
                continue;
            }
            if x < left_bound {
                // The next sensor that would impose a keepout constraint on this point doesn't. We
                // found our lucky winner.
                return tuning_frequency((x, row));
            }
            // Skip ahead to the next point unbounded by this sensor
            x = right_bound + 1;
            if x > search_bound {
                continue 'row;
            }
        }
        // We've searched through all of our sensors and we're still good? We found a winner!
        // This means that our point is on the right edge of the search space
        if x <= search_bound {
            return tuning_frequency((x, row));
        }
    }
    panic!("Could not find point");
}

fn parse_input(input: &str) -> Parsed {
    utils::find_all_integers(input)
        .chunks(4)
        .map(|chunk| Sensor::new((chunk[0], chunk[1]), (chunk[2], chunk[3])))
        .collect()
}

fn main() {
    let input = include_str!("../inputs/day15.txt");
    let input = parse_input(input);

    println!("Part 1: {}", part1(input.clone(), 2_000_000));
    println!("Part 2: {}", part2(input, 4_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
    "#;

    #[test]
    fn given_part1_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part1(input, 10), 26);
    }

    #[test]
    fn given_part2_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part2(input, 20), 56000011);
    }
}
