//! Solution to [AoC YEAR Day DAY](https://adventofcode.com/YEAR/day/DAY)

mod part1 {
    enum ParseState {
        Header,
        Metadata,
    }

    /// Minimally parse the input to find the metadata fields
    /// Return the sum of all metadata fields
    pub fn solve(tree: &[i32]) -> i32 {
        let mut stack = vec![ParseState::Header];
        let mut metadata_sum = 0;
        let mut tree = tree.iter();
        loop {
            match stack.pop() {
                Some(ParseState::Header) => {
                    let n_child_nodes = *tree.next().unwrap();
                    let n_metadata = *tree.next().unwrap();
                    for _ in 0..n_metadata {
                        stack.push(ParseState::Metadata);
                    }
                    for _ in 0..n_child_nodes {
                        stack.push(ParseState::Header);
                    }
                }
                Some(ParseState::Metadata) => metadata_sum += tree.next().unwrap(),
                None => break,
            }
        }
        metadata_sum
    }
}

mod part2 {
    /// A node in the tree structure defined in the problem.
    #[derive(Default, Debug)]
    pub struct Node {
        /// The children of the given node, given as indices into some other data structure
        children: Vec<usize>,
        /// Metadata attached to the given node
        metadata: Vec<i32>,
    }

    /// Calculate the "value" of the node at index idx using the definition in the problem
    /// statement. The root node is at index 0.
    fn value(tree: &[Node], idx: usize) -> i32 {
        let head = &tree[idx];
        if head.children.is_empty() {
            return head.metadata.iter().sum();
        }
        let mut sum = 0;
        for child_idx in &head.metadata {
            // Need to subtract 1 as the data provided is 1-indexed
            let child_idx = usize::try_from(*child_idx).unwrap() - 1;
            if let Some(node) = head.children.get(child_idx) {
                sum += value(tree, *node);
            }
        }
        sum
    }

    enum ParseState {
        /// The parser is at the start of a header belonging to a node at the given index
        Header(usize),
        /// The parser is at a single piece of metadata belonging to a node at the given index
        Metadata(usize),
    }

    /// Parse the input into a tree represented by a `Vec<Node>`
    pub fn parse_nodes(tree: &[i32]) -> Vec<Node> {
        let mut nodes = vec![Node::default()];
        let mut stack = vec![ParseState::Header(0)];
        let mut tree = tree.iter();
        loop {
            match stack.pop() {
                Some(ParseState::Header(idx)) => {
                    let n_child_nodes = *tree.next().unwrap();
                    let n_metadata = *tree.next().unwrap();
                    for _ in 0..n_metadata {
                        stack.push(ParseState::Metadata(idx));
                    }
                    for _ in 0..n_child_nodes {
                        nodes.push(Node::default());
                        let child_idx = nodes.len() - 1;
                        nodes[idx].children.push(child_idx);
                        stack.push(ParseState::Header(child_idx));
                    }
                    // Because of the LIFO nature of the stack, the children end up in reverse
                    // order
                    nodes[idx].children.reverse();
                }
                Some(ParseState::Metadata(idx)) => nodes[idx].metadata.push(*tree.next().unwrap()),
                None => break,
            }
        }
        nodes
    }

    pub fn part1(tree: &[Node]) -> i32 {
        tree.iter().flat_map(|node| &node.metadata).sum()
    }

    pub fn part2(tree: &[Node]) -> i32 {
        value(tree, 0)
    }
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .trim()
        .split_whitespace()
        .map(|i| i.parse::<i32>().unwrap())
        .collect()
}

fn main() {
    let input = include_str!("../inputs/day8.txt");
    let input = parse_input(input);
    let tree = part2::parse_nodes(&input);

    // Part 1 is solved in two different ways. `part1::solve` is simpler in isolation, but
    // `part2::part1` is simpler once we've already parsed the tree structure out of the input for
    // part 2.
    println!("Part 1: {}", part1::solve(&input));
    println!("Part 1: {}", part2::part1(&tree));
    println!("Part 2: {}", part2::part2(&tree));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2
    "#;

    #[test]
    fn given_part1_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part1::solve(&input), 138);
    }

    #[test]
    fn given_part2_input() {
        let input = parse_input(TEST_INPUT);
        let tree = part2::parse_nodes(&input);

        assert_eq!(part2::part1(&tree), 138);
        assert_eq!(part2::part2(&tree), 66);
    }
}
