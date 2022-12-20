//! Solution to [AoC 2022 Day 19](https://adventofcode.com/2022/day/19)

use rayon::prelude::*;

// Polyfill for nightly `int_roundings` feature
fn div_ceil(numerator: usize, denominator: usize) -> usize {
    numerator / denominator + usize::from(numerator % denominator != 0)
}

#[derive(Copy, Clone, Debug)]
struct Blueprint {
    // Every blueprint happens to be of the same format so we can be pretty rigid with our types
    ore_robot_cost: usize,               // Ore
    clay_robot_cost: usize,              // Ore
    obsidian_robot_cost: (usize, usize), // Ore, Clay
    geode_robot_cost: (usize, usize),    // Ore, Obsidian
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Action {
    DoNothing,
    BuildOreRobot,
    BuildClayRobot,
    BuildObsidianRobot,
    BuildGeodeRobot,
}

impl Action {
    fn build_actions() -> impl Iterator<Item = Self> {
        const BUILD_ACTIONS: [Action; 4] = [
            Action::BuildOreRobot,
            Action::BuildClayRobot,
            Action::BuildObsidianRobot,
            Action::BuildGeodeRobot,
        ];
        BUILD_ACTIONS.into_iter()
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct State {
    time_left: usize,
    // Resources
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
    // Robots
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,
}

impl State {
    fn new(time_left: usize) -> Self {
        State {
            time_left,
            ore_robots: 1,
            ..Default::default()
        }
    }

    /// Perform the given action `n` times
    fn do_action(&mut self, blueprint: &Blueprint, action: Action, n: usize) {
        // TODO: add some bounds checking on resources
        assert!(self.time_left >= n);

        // Let the robots do their work
        self.ore += n * self.ore_robots;
        self.clay += n * self.clay_robots;
        self.obsidian += n * self.obsidian_robots;
        self.geode += n * self.geode_robots;

        match action {
            Action::DoNothing => (),
            Action::BuildOreRobot => {
                self.ore -= n * blueprint.ore_robot_cost;
                self.ore_robots += n;
            }
            Action::BuildClayRobot => {
                self.ore -= n * blueprint.clay_robot_cost;
                self.clay_robots += n;
            }
            Action::BuildObsidianRobot => {
                self.ore -= n * blueprint.obsidian_robot_cost.0;
                self.clay -= n * blueprint.obsidian_robot_cost.1;
                self.obsidian_robots += n;
            }
            Action::BuildGeodeRobot => {
                self.ore -= n * blueprint.geode_robot_cost.0;
                self.obsidian -= n * blueprint.geode_robot_cost.1;
                self.geode_robots += n;
            }
        }

        self.time_left -= n;
    }

    /// Return how long we have to wait around until the action becomes available, or `None` if
    /// impossible.
    fn time_until_action_available(&self, blueprint: &Blueprint, action: Action) -> Option<usize> {
        /// Helper function to calculate the required tiem for a single resource
        fn required_steps(needed: usize, have: usize, generators: usize) -> Option<usize> {
            if have >= needed {
                Some(0)
            } else if generators == 0 {
                None
            } else {
                Some(div_ceil(needed - have, generators))
            }
        }

        match action {
            Action::DoNothing => Some(0),
            Action::BuildOreRobot => {
                required_steps(blueprint.ore_robot_cost, self.ore, self.ore_robots)
            }
            Action::BuildClayRobot => {
                required_steps(blueprint.clay_robot_cost, self.ore, self.ore_robots)
            }
            Action::BuildObsidianRobot => {
                let steps1 =
                    required_steps(blueprint.obsidian_robot_cost.0, self.ore, self.ore_robots);
                let steps2 =
                    required_steps(blueprint.obsidian_robot_cost.1, self.clay, self.clay_robots);
                steps1.zip(steps2).map(|(a, b)| a.max(b))
            }
            Action::BuildGeodeRobot => {
                let steps1 =
                    required_steps(blueprint.geode_robot_cost.0, self.ore, self.ore_robots);
                let steps2 = required_steps(
                    blueprint.geode_robot_cost.1,
                    self.obsidian,
                    self.obsidian_robots,
                );
                steps1.zip(steps2).map(|(a, b)| a.max(b))
            }
        }
    }
}

/// Find the maximum possible geodes one can have when time runs out given the inputs
fn max_geodes(state: State, blueprint: &Blueprint) -> usize {
    let mut best: usize = 0;

    // Use DFS so that we can prune branches once we have a candidate best
    let mut stack = vec![state];
    while let Some(state) = stack.pop() {
        if state.geode_robots > 0 {
            let geodes_at_end = state.geode + state.geode_robots * state.time_left;
            if geodes_at_end > best {
                best = geodes_at_end;
            }
        }
        if state.time_left == 0 {
            continue;
        }
        // Find a upper bound on the number of geodes we can end up with given our current state by
        // pretending we can build geode robots for the rest of our turns. These robots will add
        // Σ (k - 1) from k=1→n, or n * (n - 1) / 2 (where n = time left) more geodes in addition
        // to whatever the existing geode robots would create in that time. We can use this upper
        // bound to prune branches that can't beat our current best.
        if state.geode
            + state.time_left * state.geode_robots
            + (state.time_left - 1) * state.time_left / 2
            < best
        {
            continue;
        }
        for action in Action::build_actions() {
            let Some(time) = state.time_until_action_available(blueprint, action) else {
                continue;
            };
            if time + 1 > state.time_left {
                continue;
            }
            let mut next_state = state;
            next_state.do_action(blueprint, Action::DoNothing, time);
            next_state.do_action(blueprint, action, 1);
            stack.push(next_state);
        }
    }
    best
}

fn part1(blueprints: &[Blueprint]) -> usize {
    blueprints
        .into_par_iter()
        .enumerate()
        .map(|(i, blueprint)| {
            let state = State::new(24);
            (i + 1) * max_geodes(state, blueprint)
        })
        .sum()
}

fn part2(blueprints: &[Blueprint]) -> usize {
    blueprints
        .into_par_iter()
        .take(3)
        .map(|blueprint| {
            let state = State::new(32);
            max_geodes(state, blueprint)
        })
        .product()
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    let mut out = Vec::new();

    for line in input.trim().lines() {
        // Lazy parsing
        let numbers = utils::find_all_integers(line);
        out.push(Blueprint {
            ore_robot_cost: numbers[1],
            clay_robot_cost: numbers[2],
            obsidian_robot_cost: (numbers[3], numbers[4]),
            geode_robot_cost: (numbers[5], numbers[6]),
        });
    }
    out
}

fn main() {
    let input = include_str!("../inputs/day19.txt");
    let input = parse_input(input);

    let (p1, p2) = rayon::join(|| part1(&input), || part2(&input));
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
    "#;

    #[test]
    fn given_part1_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part1(&input), 33);
    }

    #[test]
    fn given_part2_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part2(&input), 56 * 62);
    }
}
