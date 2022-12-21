//! Solution to [AoC 2022 Day 21](https://adventofcode.com/2022/day/21)

use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Sub};

// Rust let's us safely borrow strings from the input without cloning ❤
type Tree<'a> = HashMap<&'a str, Children<'a>>;

/// A node's children/dependents
#[derive(Debug, Copy, Clone, PartialEq)]
enum Children<'a> {
    /// An integer. Represent integers as floats and use float arithmetic so that we can perform
    /// division without truncation, which is needed to solve the equation in part 2
    Integer(f64),
    /// Two child node ids and a char representing the arithmetic operator
    Nodes(&'a str, &'a str, char),
}

fn op<T>(c: char) -> fn(T, T) -> T
where
    T: Add<T>
        + Add<Output = T>
        + Sub<T>
        + Sub<Output = T>
        + Mul<T>
        + Mul<Output = T>
        + Div<T>
        + Div<Output = T>,
{
    match c {
        '+' => T::add,
        '-' => T::sub,
        '/' => T::div,
        '*' => T::mul,
        c => panic!("unexpected op: {c}"),
    }
}

/// Return the value produced at the given node by recursively descending the tree
fn traverse_p1(tree: &Tree, id: &str) -> f64 {
    match tree[id] {
        Children::Integer(n) => n,
        Children::Nodes(child1, child2, node_op) => {
            op(node_op)(traverse_p1(tree, child1), traverse_p1(tree, child2))
        }
    }
}

fn part1(tree: &Tree) -> i64 {
    traverse_p1(tree, "root") as i64
}

/// Return a closure representing the value produced at the given node as a function of what the
/// human ("humn") produces by recursively descending the tree and composing closures
fn traverse_p2<'a>(tree: &'a Tree, id: &str) -> Box<dyn Fn(f64) -> f64 + 'a> {
    if id == "humn" {
        return Box::new(|humn| humn);
    }

    match tree[id] {
        Children::Integer(n) => Box::new(move |_| n),
        Children::Nodes(child1, child2, node_op) => Box::new(move |humn| {
            op(node_op)(
                traverse_p2(tree, child1)(humn),
                traverse_p2(tree, child2)(humn),
            )
        }),
    }
}

/// Find the root of f(x). Requires a pair of initial guesses.
///
/// Implements the secant method from https://en.wikipedia.org/wiki/Secant_method
fn find_root<F: Fn(f64) -> f64>(f: F, mut x0: f64, mut x1: f64) -> Option<f64> {
    assert_ne!(x0, x1);
    // Bound the number of steps before we must converge
    let max_steps = 1000;

    for _ in 0..max_steps {
        let fx1 = f(x1);
        let x = x1 - fx1 * (x1 - x0) / (fx1 - f(x0));
        // Using such a small epsilon is overkill but doesn't appreciably change runtime over
        // larger episilon values
        if (x - x1).abs() <= f64::EPSILON {
            return Some(x);
        }
        (x1, x0) = (x, x1);
    }
    None
}

fn part2(tree: &Tree) -> i64 {
    let Children::Nodes(child1, child2, _) = tree["root"] else {
        panic!("root has no children");
    };
    let f1 = traverse_p2(tree, child1);
    let f2 = traverse_p2(tree, child2);
    // Solve f1(humn) == f2(humn) with a pair of arbitrary initial guesses
    find_root(|x| f1(x) - f2(x), 0.0, 10.0)
        .expect("solution to converge")
        .round() as i64
}

fn parse_input(input: &str) -> Tree {
    let mut points = HashMap::new();
    for line in input.trim().lines() {
        let (id, rest) = line.split_once(": ").unwrap();
        if rest.chars().next().unwrap().is_ascii_digit() {
            points.insert(id, Children::Integer(rest.parse().unwrap()));
        } else {
            let mut tokens = rest.split(' ');
            let child1 = tokens.next().unwrap();
            let op = tokens.next().unwrap().chars().next().unwrap();
            let child2 = tokens.next().unwrap();
            points.insert(id, Children::Nodes(child1, child2, op));
        }
    }
    points
}

fn main() {
    let input = include_str!("../inputs/day21.txt");
    let input = parse_input(input);

    let (p1, p2) = rayon::join(|| part1(&input), || part2(&input));
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
    "#;

    #[test]
    fn given_part1_input() {
        let input = parse_input(TEST_INPUT);
        dbg!(&input);
        assert_eq!(part1(&input), 152);
    }

    #[test]
    fn given_part2_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part2(&input), 301);
    }
}
