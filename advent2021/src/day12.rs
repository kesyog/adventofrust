//! Solution to [AoC 2021 Day 12](https://adventofcode.com/2021/day/12)

use std::collections::HashMap;

type Graph = HashMap<&'static str, Vec<&'static str>>;

fn traverse(
    graph: &Graph,
    nodes_visited: Vec<&'static str>,
    small_nodes_repeated: usize,
    small_node_repeat_limit: usize,
) -> usize {
    let mut n_paths = 0;
    for &node in &graph[nodes_visited.last().unwrap()] {
        if node == "end" {
            n_paths += 1;
        } else if node.chars().next().unwrap().is_uppercase() || !nodes_visited.contains(&node) {
            let mut visited = nodes_visited.clone();
            visited.push(node);
            n_paths += traverse(
                graph,
                visited,
                small_nodes_repeated,
                small_node_repeat_limit,
            );
        } else if small_nodes_repeated < small_node_repeat_limit {
            let mut visited = nodes_visited.clone();
            visited.push(node);
            n_paths += traverse(
                graph,
                visited,
                small_nodes_repeated + 1,
                small_node_repeat_limit,
            );
        }
    }
    n_paths
}

fn part1(graph: &Graph) -> usize {
    traverse(graph, vec!["start"], 0, 0)
}

fn part2(graph: &Graph) -> usize {
    traverse(graph, vec!["start"], 0, 1)
}

fn parse_input(input: &'static str) -> Graph {
    let mut graph = HashMap::new();
    for line in utils::trim_and_split(input, "\n") {
        let (node1, node2) = line.split_once('-').unwrap();
        if node1 != "end" && node2 != "start" {
            graph.entry(node1).or_insert_with(Vec::new).push(node2);
        }
        if node2 != "end" && node1 != "start" {
            graph.entry(node2).or_insert_with(Vec::new).push(node1);
        }
    }
    graph
}

fn main() {
    let input = include_str!("../inputs/day12.txt");
    let input = parse_input(input);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end
    "#;

    #[test]
    fn given_part1_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part1(&input), 10);
    }

    #[test]
    fn given_part2_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part2(&input), 36);
    }
}
