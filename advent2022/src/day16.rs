//! Solution to [AoC 2022 Day 16](https://adventofcode.com/2022/day/16)

use anyhow::Error;
use once_cell::sync::Lazy;
use petgraph::{graph::NodeIndex, Graph, Undirected};
use rayon::prelude::*;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

/// A wrapper around petgraph's `Graph` type with some handy pre-computed metadata
struct CaveGraph {
    graph: Graph<Room, usize, Undirected>,
    start: NodeIndex,
    /// Locations of the valves
    valves: Vec<NodeIndex>,
    /// The shortest path between each node and each valve
    /// room -> (valve room, cost)
    shortest_paths: HashMap<NodeIndex, Vec<(NodeIndex, usize)>>,
}

impl CaveGraph {
    fn max_flow_rate(&self) -> usize {
        flow_rate(self, self.valves.iter().copied())
    }

    #[cfg(test)]
    fn find_node_by_name(&self, name: &str) -> NodeIndex {
        self.graph
            .node_indices()
            .find(|&i| self.graph.node_weight(i).unwrap().name == name)
            .unwrap()
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Room {
    // TODO: String -> char array
    name: String,
    valve: Option<usize>,
    tunnels: Vec<String>,
}

impl FromStr for Room {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static PARSER: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"Valve ([A-Z]{2}).*rate=(\d+).*valves? (.*)$").unwrap());
        let captures = PARSER.captures(s).unwrap();
        let name = captures[1].to_string();
        let valve = match captures[2].parse::<usize>()? {
            0 => None,
            rate => Some(rate),
        };
        let tunnels = captures[3].split(", ").map(ToString::to_string).collect();
        Ok(Self {
            name,
            valve,
            tunnels,
        })
    }
}

/// Calculate the flow rate given a list of opened valves
fn flow_rate(graph: &CaveGraph, opened: impl Iterator<Item = NodeIndex>) -> usize {
    opened
        .filter_map(|i| graph.graph.node_weight(i).unwrap().valve)
        .sum()
}

/// Calculate the flow released if we tried to open the given sequence of valves
fn simulate(graph: &CaveGraph, valve_sequence: &[NodeIndex], mut time_left: usize) -> usize {
    let mut node = graph.start;
    let mut opened: Vec<NodeIndex> = Vec::new();
    let mut released = 0;

    for valve in valve_sequence.iter().copied() {
        let cost = graph.shortest_paths[&node]
            .iter()
            .find(|&(dest, _)| *dest == valve)
            .unwrap()
            .1;
        let flow_rate = flow_rate(graph, opened.iter().copied());
        if cost + 1 > time_left {
            return released + time_left * flow_rate;
        }
        time_left -= cost + 1;
        opened.push(valve);
        released += flow_rate * (cost + 1);
        node = valve;
    }
    if time_left > 0 {
        released += time_left * flow_rate(graph, opened.iter().copied());
    }
    released
}

// BFS search
// One should only ever be heading straight to a valve, opening the current valve, or waiting
// around for time to end because there is nothing productive to do. So at each step, we do one of
// those (rather than simply simulating one time step at a time).
fn part1(graph: &CaveGraph) -> usize {
    let max_flow_rate = graph.max_flow_rate();
    let time_limit = 30;

    let mut queue = VecDeque::new();
    queue.push_back((graph.start, time_limit, HashSet::<NodeIndex>::new(), 0));
    let mut best = usize::MIN;

    while let Some((room, time_left, mut opened, released)) = queue.pop_back() {
        // Check what would happen if we sat around
        let do_nothing_total = released + time_left * flow_rate(graph, opened.iter().copied());
        best = best.max(do_nothing_total);

        // All valves are open. No need to branch further
        if opened.len() == graph.valves.len() {
            let total = released + max_flow_rate * time_left;
            if total > best {
                best = total;
            }
            continue;
        }
        if time_left == 0 {
            if released > best {
                best = released;
            }
            continue;
        }
        // Add a branch to visit each reachable valve
        // Good thing we pre-calculated the shortest paths to each valve.
        for (destination, cost) in &graph.shortest_paths[&room] {
            // Valve is already open
            if opened.contains(destination) {
                continue;
            }
            // We don't have the time to reach and open this valve
            if *cost + 1 >= time_left {
                continue;
            }
            let mut newly_opened = opened.clone();
            newly_opened.insert(*destination);
            queue.push_back((
                *destination,
                time_left - (cost + 1),
                newly_opened,
                released + (cost + 1) * flow_rate(graph, opened.iter().copied()),
            ));
        }
        // Add a branch to open the valve in the current room if not already open
        if graph.valves.contains(&room) && !opened.contains(&room) {
            let flow = flow_rate(graph, opened.iter().copied());
            opened.insert(room);
            queue.push_back((room, time_left - 1, opened, released + flow));
        }
    }
    best
}

// Generate all possible sequences of visited valves feasible for one person
// Basically just re-use the part 1 solution with some early returns removed
fn all_valve_sequences(graph: &CaveGraph, time_limit: usize) -> HashSet<Vec<NodeIndex>> {
    let mut out = HashSet::new();

    let mut queue: VecDeque<(NodeIndex, usize, Vec<NodeIndex>, usize)> = VecDeque::new();
    queue.push_back((graph.start, time_limit, Vec::<NodeIndex>::new(), 0));

    while let Some((room, time_left, mut opened, released)) = queue.pop_back() {
        if opened.len() == graph.valves.len() {
            out.insert(opened);
            continue;
        }
        if time_left == 0 {
            continue;
        }
        // Visit paths
        for (destination, cost) in &graph.shortest_paths[&room] {
            if opened.contains(destination) {
                continue;
            }
            if *cost + 1 >= time_left {
                continue;
            }
            let mut newly_opened = opened.clone();
            newly_opened.push(*destination);
            out.insert(newly_opened.clone());
            queue.push_back((
                *destination,
                time_left - (cost + 1),
                newly_opened,
                released + (cost + 1) * flow_rate(graph, opened.iter().copied()),
            ));
        }
        if graph.valves.contains(&room) && !opened.contains(&room) {
            let flow = flow_rate(graph, opened.iter().copied());
            opened.push(room);
            queue.push_back((room, time_left - 1, opened, released + flow));
        }
    }
    out
}

// Flow release is "linear" i.e. we can add up the flow released by us and the flow released by the
// elephant.
//
// Gross O(n^2) solution :( Runs slowly (~10s parallelized or ~1min single-threaded with
// memoization) but still finishes.
fn part2(graph: &CaveGraph) -> usize {
    let time_left = 26;
    let options = all_valve_sequences(graph, time_left);

    options
        .par_iter()
        .map(|me| {
            let my_flow = simulate(graph, me, time_left);

            // It'd be nice if we could prune this list efficiently somehow.
            // TODO: I'd _like_ to build up permutations of valves from the leftover valves and prune the
            // list by backtracking once we've reached the limit, but I haven't figured out how to
            // structure that.
            options
                .par_iter()
                .filter_map(|elephant| {
                    // The search space is symmetric. Cut it in half.
                    if elephant.len() > me.len() {
                        return None;
                    }
                    // The elephant shouldn't be opening any valves we're planning to open
                    if elephant.iter().any(|valve| me.contains(valve)) {
                        return None;
                    }

                    let elephant_flow = simulate(graph, elephant, time_left);
                    Some(elephant_flow + my_flow)
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

fn parse_input(input: &str) -> CaveGraph {
    let mut graph = Graph::<Room, usize, Undirected>::new_undirected();
    let mut node_map: HashMap<String, NodeIndex> = HashMap::new();
    let mut valves: Vec<NodeIndex> = Vec::new();

    for line in input.trim().lines() {
        let room = line.parse::<Room>().unwrap();
        let name = room.name.clone();
        let has_valve = room.valve.is_some();
        let node = graph.add_node(room);
        if has_valve {
            valves.push(node);
        }
        node_map.insert(name, node);
    }

    for (name, idx) in &node_map {
        let room = graph.node_weight(*idx).unwrap();
        let destinations = room.tunnels.clone();
        for destination in destinations {
            if name > &destination {
                graph.add_edge(*idx, node_map[&destination], 1);
            }
        }
    }
    // Let petgraph calculate the shortest path between
    let mut shortest_paths = HashMap::new();
    for node in graph.node_indices() {
        for (dest, cost) in petgraph::algo::dijkstra::dijkstra(&graph, node, None, |e| *e.weight())
        {
            if dest == node || !valves.contains(&dest) {
                continue;
            }
            shortest_paths
                .entry(node)
                .or_insert_with(Vec::new)
                .push((dest, cost));
        }
    }

    CaveGraph {
        graph,
        start: node_map["AA"],
        valves,
        shortest_paths,
    }
}

fn main() {
    let input = include_str!("../inputs/day16.txt");
    let input = parse_input(input);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
    "#;

    #[test]
    fn given_part1_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part1(&input), 1651);
        let path = vec!["DD", "BB", "JJ", "HH", "EE", "CC"];
        assert_eq!(
            simulate(
                &input,
                &path
                    .into_iter()
                    .map(|name| input.find_node_by_name(name))
                    .collect::<Vec<NodeIndex>>(),
                30
            ),
            1651
        );
    }

    #[test]
    fn given_part2_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part2(&input), 1707);
    }
}
