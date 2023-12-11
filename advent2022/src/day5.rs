//! Solution to [AoC 2022 Day 5](https://adventofcode.com/2022/day/5)

fn get_output(stacks: Vec<Vec<char>>) -> String {
    let mut out = String::with_capacity(stacks.len());
    for mut stack in stacks {
        out.push(stack.pop().expect("Stack to be non-empty"));
    }
    out
}

fn part1(mut stacks: Vec<Vec<char>>, moves: &[Move]) -> String {
    for &Move { n, source, dest } in moves {
        for _ in 0..n {
            let transfer = stacks[source].pop().unwrap();
            stacks[dest].push(transfer);
        }
    }
    get_output(stacks)
}

fn part2(mut stacks: Vec<Vec<char>>, moves: &[Move]) -> String {
    for &Move { n, source, dest } in moves {
        let source_len = stacks[source].len();
        let transfer = stacks[source].split_off(source_len - n);
        stacks[dest].extend(transfer);
    }
    get_output(stacks)
}

#[derive(Copy, Clone, Debug)]
struct Move {
    n: usize,
    source: usize,
    dest: usize,
}

fn parse_moves(moves: &str) -> Vec<Move> {
    utils::find_all_integers::<usize>(moves)
        .chunks_exact(3)
        .map(|chunk| Move {
            n: chunk[0],
            // Convert from 1-indexing to 0-indexing
            source: chunk[1] - 1,
            dest: chunk[2] - 1,
        })
        .collect()
}

fn parse_stacks(stacks: &str) -> Vec<Vec<char>> {
    let max_height = stacks.lines().count() - 1;
    // Width of each row, including null-terminator
    let row_width = stacks.find('\n').unwrap() + 1;
    let n_stacks = row_width / 4;

    let mut out = Vec::new();
    // Build stacks one at a time, reading characters from their expected position in the string
    for stack_idx in 0..n_stacks {
        let mut stack = Vec::new();
        for row in (0..max_height).rev() {
            // Assuming ASCII allows for faster repeated indexing into the input string
            match stacks.as_bytes()[(stack_idx * 4 + 1) + (row * row_width)] {
                b' ' => break,
                c => stack.push(c.into()),
            }
        }
        out.push(stack);
    }
    out
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let (stacks, moves) = input.trim_matches('\n').split_once("\n\n").unwrap();
    (parse_stacks(stacks), parse_moves(moves))
}

#[cfg(not(test))]
fn main() {
    let input = include_str!("../inputs/day5.txt");
    let input = parse_input(input);

    println!("Part 1: {}", part1(input.0.clone(), &input.1));
    println!("Part 2: {}", part2(input.0, &input.1));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

    #[test]
    fn given_part1_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!("CMZ", part1(input.0, &input.1));
    }

    #[test]
    fn given_part2_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!("MCD", part2(input.0, &input.1));
    }
}
