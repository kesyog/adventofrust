//! Solution to [AoC 2018 Day 7](https://adventofcode.com/2018/day/7)

use anyhow::{bail, Result};
use std::collections::{BTreeSet, HashMap, HashSet};

#[derive(Debug, Clone)]
struct Graph {
    /// The nodes without any prerequisites
    head: Vec<char>,
    /// A mapping of nodes to the nodes that depend on it
    dependents_map: HashMap<char, Vec<char>>,
    /// A mapping of nodes to their prequisite nodes
    prerequisite_map: HashMap<char, Vec<char>>,
}

impl Graph {
    fn create_from_rules(rules: &[(char, char)]) -> Self {
        let mut dependents_map: HashMap<char, Vec<char>> = HashMap::new();
        let mut prerequisite_map: HashMap<char, Vec<char>> = HashMap::new();
        let mut parents: HashSet<char> = HashSet::new();
        let mut children: HashSet<char> = HashSet::new();

        for &(parent, child) in rules {
            children.insert(child);
            parents.insert(parent);
            dependents_map
                .entry(parent)
                .or_insert_with(Vec::new)
                .push(child);
            prerequisite_map
                .entry(child)
                .or_insert_with(Vec::new)
                .push(parent);
        }
        let head = &parents - &children;
        let head = head.into_iter().collect();
        Self {
            head,
            dependents_map,
            prerequisite_map,
        }
    }
}

fn part1(recipe: &Graph) -> String {
    let mut output = String::new();
    let mut candidates: BTreeSet<char> = recipe.head.iter().copied().collect();
    while let Some(&active) = candidates.iter().next() {
        candidates.remove(&active);
        output.push(active);

        for &child in recipe.dependents_map.get(&active).into_iter().flatten() {
            if recipe
                .prerequisite_map
                .get(&child)
                .into_iter()
                .flatten()
                .all(|&i| output.contains(i))
            {
                candidates.insert(child);
            }
        }
    }
    output
}

#[cfg(test)]
fn work_cost(work: char) -> u32 {
    u32::from(work.to_ascii_uppercase()) - u32::from('A') + 1
}

#[cfg(not(test))]
fn work_cost(work: char) -> u32 {
    u32::from(work.to_ascii_uppercase()) - u32::from('A') + 1 + 60
}

#[derive(Default, Clone, Copy, Debug)]
struct Worker {
    work: Option<char>,
    remaining_time: u32,
}

impl Worker {
    fn new() -> Self {
        Self::default()
    }

    fn busy(&self) -> bool {
        self.work.is_some()
    }

    fn tick(&mut self) -> Option<char> {
        match self.work {
            Some(_) => {
                self.remaining_time -= 1;
                if self.remaining_time == 0 {
                    self.work.take()
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn assign(&mut self, node: char) -> Result<()> {
        if self.busy() {
            bail!("busy");
        }

        self.work = Some(node);
        self.remaining_time = work_cost(node);
        Ok(())
    }
}

#[derive(Debug)]
struct WorkQueue {
    workers: Vec<Worker>,
}

impl WorkQueue {
    fn new(n_workers: usize) -> Self {
        Self {
            workers: vec![Worker::new(); n_workers],
        }
    }

    fn tick(&mut self) -> Option<char> {
        let ready: Vec<char> = self.workers.iter_mut().filter_map(|i| i.tick()).collect();
        assert!(ready.len() <= 1);
        ready.first().copied()
    }

    /// Assign a node to the work queue, returning `Ok` if the node was assigned or `Err` otherwise
    fn assign(&mut self, node: char) -> Result<()> {
        for worker in &mut self.workers {
            if worker.assign(node).is_ok() {
                return Ok(());
            }
        }
        bail!("busy");
    }

    fn pending(&self) -> bool {
        self.workers.iter().any(|i| i.busy())
    }
}

fn part2(recipe: &Graph, n_workers: usize) -> usize {
    let mut workers = WorkQueue::new(n_workers);
    let mut time: usize = 0;
    let mut output = String::new();
    let mut candidates: BTreeSet<char> = recipe.head.iter().copied().collect();
    while !candidates.is_empty() || workers.pending() {
        // Assign as much work as possible to work queue
        for candidate in candidates.clone() {
            match workers.assign(candidate) {
                Ok(_) => candidates.remove(&candidate),
                Err(_) => break,
            };
        }

        if let Some(active) = workers.tick() {
            output.push(active);
            // Check if the new character unlocked any new candidate work
            for &child in recipe.dependents_map.get(&active).into_iter().flatten() {
                if recipe
                    .prerequisite_map
                    .get(&child)
                    .into_iter()
                    .flatten()
                    .all(|&i| output.contains(i))
                {
                    candidates.insert(child);
                }
            }
        }
        time += 1;
    }
    time
}

fn parse_input(input: &str) -> Vec<(char, char)> {
    utils::trim_and_split(input, "\n")
        // Hack: Use hard-coded indexes to parse out node names since they're all one character and
        // in the same spot on every line
        .map(|line| (line.chars().nth(5).unwrap(), line.chars().nth(36).unwrap()))
        .collect()
}

#[cfg(not(test))]
fn main() {
    let input = include_str!("../inputs/day7.txt");
    let rules = parse_input(input);
    let recipe = Graph::create_from_rules(&rules);

    println!("Part 1: {}", part1(&recipe));
    println!("Part 2: {}", part2(&recipe, 5));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
    Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
    "#;

    #[test]
    fn given_part1_input() {
        let rules = parse_input(TEST_INPUT);
        let recipe = Graph::create_from_rules(&rules);
        assert_eq!(part1(&recipe), "CABDFE");
    }

    #[test]
    fn given_part2_input() {
        let rules = parse_input(TEST_INPUT);
        let recipe = Graph::create_from_rules(&rules);
        assert_eq!(part2(&recipe, 2), 15);
    }
}
