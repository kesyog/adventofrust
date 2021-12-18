//! Solution to [AoC 2021 Day 17](https://adventofcode.com/2021/day/17)
//!
//! #### Math
//! x will stabilize at t = |dx| to a value of dx * (dx + 1) / 2
//! y = (t * (2 * dy + 1) - t*t) / 2
//! dy/dt = dy - t + 1/2
//! dy = y/t + t/2 - 1/2
//!
//! dy, t, y are all integers and t >= 0
//!
//! Therefore: y / t = c / 2 where c is an integer
//! t = 2 * y / c
//! t <= | 2 * y |

use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
};

#[derive(Debug, Clone, Copy, Hash, PartialOrd, Ord, PartialEq, Eq)]
enum TimeConstraint {
    Exact(usize),
    Min(usize),
}

impl TimeConstraint {
    fn iter(&self) -> Box<dyn Iterator<Item = usize>> {
        match self {
            TimeConstraint::Exact(value) => Box::new(std::iter::once(*value)),
            TimeConstraint::Min(min_bound) => Box::new(*min_bound..),
        }
    }
}

/// Calculate y at a given time given an initial velocity
fn sim_y(t: usize, dy_init: isize) -> isize {
    // y = y0 + t * y - (t-1) * t / 2
    ((t as isize) * (2 * dy_init + 1) - (t * t) as isize) / 2
}

/// Check if there is a valid solution with the given parameters and the given t constraint
fn search(y_min: isize, y_max: isize, dy_init: isize, t: TimeConstraint) -> bool {
    assert!(y_min < y_max);
    for t in t.iter() {
        let y = sim_y(t, dy_init);
        let dy_dt = dy_init - (t as isize);
        if y >= y_min && y <= y_max {
            return true;
        } else if dy_dt <= 0 && y < y_min {
            break;
        }
    }
    false
}

/// Return initial x velocities that land in the target area (in terms of the x_min/x_max
/// constraints) and the corresponding time at which they reach the target area. Note that in some
/// cases, the x velocity will drop to zero within the target area, so they will remain there
/// indefinitely.
fn find_valid_dx_t(x_min: isize, x_max: isize) -> Vec<(isize, TimeConstraint)> {
    assert!(x_min > 0);
    assert!(x_max > 0);
    assert!(x_min < x_max);
    let mut valid_t = Vec::new();
    'dx: for dx_init in 1..=x_max {
        let mut dx = dx_init;
        let mut x = 0;
        for t in 1.. {
            x += dx;
            if x >= x_min && x <= x_max {
                if dx.abs() <= 1 {
                    valid_t.push((dx_init, TimeConstraint::Min(t)));
                    continue 'dx;
                } else {
                    valid_t.push((dx_init, TimeConstraint::Exact(t)));
                }
            }
            dx -= dx.signum();
            if x > x_max || dx == 0 {
                break;
            }
        }
    }
    valid_t
}

/// Find (dx, dy) velocity pairs that result in the ball reaching the target area
fn find_valid_velocities(bounding_box: &[isize; 4]) -> HashSet<(isize, isize)> {
    let [x_min, x_max, y_min, y_max] = *bounding_box;
    // 1. find valid dx and t combinations
    let xt_search_space = find_valid_dx_t(x_min, x_max);
    let mut valid: HashSet<(isize, isize)> = HashSet::new();
    let mut cache: HashMap<TimeConstraint, Vec<isize>> = HashMap::new();
    for (dx_init, t) in xt_search_space {
        // 2. Find dy's that satisfy the given t constraint
        if let Some(valid_y) = cache.get(&t) {
            valid.extend(valid_y.iter().map(|dy| (dx_init, *dy)))
        } else {
            let dy_upper_bound = -2 * y_min;
            for dy_init in y_min..dy_upper_bound {
                if search(y_min, y_max, dy_init, t) {
                    valid.insert((dx_init, dy_init));
                    cache.entry(t).or_insert_with(Vec::new).push(dy_init);
                }
            }
        }
    }
    valid
}

fn part1(velocities: &HashSet<(isize, isize)>) -> isize {
    let valid_dy: HashSet<isize> = velocities.iter().map(|i| i.1).collect();
    let mut max_y = isize::MIN;
    for dy in valid_dy {
        if dy <= 0 {
            continue;
        }
        // dy/dt = dy_init - t + 1/2
        // Hand-waving over edge cases where there are no solutions where dy_init > 0,
        // the max height is reached around t = dy_init + 1/2
        max_y = max_y.max(sim_y(dy as usize, dy));
        max_y = max_y.max(sim_y(dy as usize + 1, dy));
    }
    max_y
}

fn part2(velocities: &HashSet<(isize, isize)>) -> usize {
    velocities.len()
}

fn parse_input(input: &str) -> [isize; 4] {
    utils::find_all_integers(input).try_into().unwrap()
}

fn main() {
    let input = include_str!("../inputs/day17.txt");
    let input = parse_input(input);
    let velocities = find_valid_velocities(&input);

    println!("Part 1: {}", part1(&velocities));
    println!("Part 2: {}", part2(&velocities));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn given_part1_input() {
        let input = parse_input(TEST_INPUT);
        let velocities = find_valid_velocities(&input);
        assert_eq!(45, part1(&velocities));
    }

    #[test]
    fn given_part2_input() {
        let input = parse_input(TEST_INPUT);
        let velocities = find_valid_velocities(&input);
        assert_eq!(112, part2(&velocities));
    }
}
