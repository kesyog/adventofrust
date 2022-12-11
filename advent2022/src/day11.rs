//! Solution to [AoC 2022 Day 11](https://adventofcode.com/2022/day/11)

use std::cell::Cell;
use std::ops::Mul;
use std::rc::Rc;

use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Clone)]
struct Monkey {
    items: Vec<usize>,
    // Use Rc to make Monkey cloneable
    inspect: Rc<dyn Fn(usize) -> usize>,
    find_target: Rc<dyn Fn(usize) -> usize>,
    divisor: usize,
}

// Need a dummy default implementation so we can use Cell::take while operating on the monkey array
impl Default for Monkey {
    fn default() -> Self {
        let dummy_fn = Rc::new(|_: usize| 0_usize);
        Self {
            items: Vec::default(),
            inspect: dummy_fn.clone(),
            find_target: dummy_fn,
            divisor: 0,
        }
    }
}

impl std::fmt::Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .finish()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum OpArg {
    Old,
    Usize(usize),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn exec(self, arg1: usize, arg2: usize) -> usize {
        match self {
            Operation::Add => arg1 + arg2,
            Operation::Multiply => arg1 * arg2,
        }
    }
}

// More or less re-implementing the unstable feature: cell_update
// https://github.com/rust-lang/rust/issues/50186
fn cell_update<T: Default, F>(cell: &Cell<T>, f: F)
where
    F: FnOnce(&mut T),
{
    let mut contents = cell.take();
    f(&mut contents);
    cell.set(contents);
}

// Common code from part1 and part2
fn helper(
    monkeys: &mut [Monkey],
    worry_factor: usize,
    modulo_base: usize,
    num_rounds: usize,
) -> usize {
    // We want to be able to mutate the active monkey while also mutating other monkeys in the
    // array (when throwing items to other monkeys). Treat the monkey Vec as a slice of Cells to
    // prove to the borrow checker that we aren't doing any unsafe monkey business.
    let monkeys = Cell::from_mut(monkeys).as_slice_of_cells();
    let mut counts = vec![0; monkeys.len()];
    for _ in 0..num_rounds {
        for (i, monkey) in monkeys.iter().enumerate() {
            cell_update(monkey, |monkey| {
                counts[i] += monkey.items.len();
                for item in monkey.items.drain(..) {
                    let worry_level = (monkey.inspect)(item) / worry_factor % modulo_base;
                    let new_monkey_idx = (monkey.find_target)(worry_level);
                    // We'd need extra logic if it were possible for a monkey to keep an item
                    // Lazily add this assert to make sure we don't need to account for that
                    // possibility
                    assert_ne!(new_monkey_idx, i);
                    cell_update(&monkeys[new_monkey_idx], |target| {
                        target.items.push(worry_level);
                    });
                }
            });
        }
    }
    counts.sort_unstable();
    counts.into_iter().rev().take(2).reduce(Mul::mul).unwrap()
}

fn part1(mut monkeys: Vec<Monkey>) -> usize {
    helper(&mut monkeys, 3, usize::MAX, 20)
}

fn part2(mut monkeys: Vec<Monkey>) -> usize {
    // Given:
    // 1. Modular multiplication and addition are distributive, i.e.
    //    (a + b) % n = ((a % n) + (b % n)) % n
    //    (a * b) % n = ((a % n) * (b % n)) % n
    // 2. x % a = (x % (a * b)) % a
    //    Proof:
    //    x % a = r1 -> a * k1 + r1 = x -> r1 = x - a * k1
    //    x & (a * b) = r2 -> a * b * k2 + r2 = x -> r2 = x - a * b * k2
    //    (x % (a * b)) % a = r2 % a = (x - a * b * k2) % a = x % a - (a * b * k2) % a = x % a
    // 3. All of the divisors are coprime
    //
    // Then:
    // We can do all of our math modulo the product of all of the divisors without changing the
    // outcome of any of the divisibility tests.
    let divisor_product: usize = monkeys.iter().map(|m| m.divisor).product();
    helper(&mut monkeys, 1, divisor_product, 10000)
}

/// Create a closure that implements a monkey's inspection operation
fn make_inspect_fn(arg1: OpArg, op: Operation, arg2: OpArg) -> Rc<dyn Fn(usize) -> usize> {
    fn make_arg(input: usize, arg: OpArg) -> usize {
        match arg {
            OpArg::Old => input,
            OpArg::Usize(val) => val,
        }
    }
    Rc::new(move |old| op.exec(make_arg(old, arg1), make_arg(old, arg2)))
}

/// Find first instance of a positive integer in a string
fn find_num(input: &str) -> usize {
    static POS_NUM_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());
    POS_NUM_REGEX.find(input).unwrap().as_str().parse().unwrap()
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let mut out = Vec::new();
    // Assume positive numbers only
    let op_regex = Regex::new(r"= (\w+) ([*+]) (\w+)").unwrap();
    for (monkey_id, mut block) in utils::trim_and_split(input, "\n\n")
        .map(str::lines)
        .enumerate()
    {
        // Parse monkey id
        let Some(line) = block.next() else {
            panic!("Invalid input");
        };
        // Assume monkeys are listed in order
        assert_eq!(find_num(line), monkey_id);
        // Parse starting items
        let Some(line) = block.next() else {
            panic!("Invalid input");
        };
        let items = utils::find_all_integers(line);
        // Parse operation
        let Some(line) = block.next() else {
            panic!("Invalid input");
        };
        let captures = op_regex.captures(line).unwrap();
        let arg1 = captures[1]
            .parse::<usize>()
            .map(OpArg::Usize)
            .unwrap_or(OpArg::Old);
        let operator = match &captures[2] {
            "*" => Operation::Multiply,
            "+" => Operation::Add,
            _ => panic!("invalid operation"),
        };
        let arg2 = captures[3]
            .parse::<usize>()
            .map(OpArg::Usize)
            .unwrap_or(OpArg::Old);
        let inspect = make_inspect_fn(arg1, operator, arg2);
        // Parse divisibility test
        let Some(line) = block.next() else {
            panic!("Invalid input");
        };
        let divisor = find_num(line);
        let Some(line) = block.next() else {
            panic!("Invalid input");
        };
        let true_target = find_num(line);
        let Some(line) = block.next() else {
            panic!("Invalid input");
        };
        let false_target = find_num(line);
        let find_target = Rc::new(move |num| {
            if num % divisor == 0 {
                true_target
            } else {
                false_target
            }
        });
        out.push(Monkey {
            items,
            inspect,
            find_target,
            divisor,
        });
    }
    out
}

fn main() {
    let input = include_str!("../inputs/day11.txt");
    let input = parse_input(input);

    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
    "#;

    #[test]
    fn given_part1_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part1(input), 10605);
    }

    #[test]
    fn given_part2_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part2(input), 2713310158);
    }
}
