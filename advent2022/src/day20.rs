//! Solution to [AoC 2022 Day 20](https://adventofcode.com/2022/day/20)

// Weird function signature is to make it work for lists of values or (index, value) pairs for
// debugging
fn mix<T>(list: &mut [T], idx: usize, value: i64) {
    // Shifting (length - 1) times returns you to the same ordering
    let final_pos = (idx as i64 + value).rem_euclid(list.len() as i64 - 1);
    let shift = final_pos - idx as i64;
    for i in 0..shift.abs() {
        list.swap(
            // Be extra cautious about signed->unsigned cast since this math is non-trivial
            usize::try_from(idx as i64 + i * shift.signum()).unwrap(),
            usize::try_from(idx as i64 + (i + 1) * shift.signum()).unwrap(),
        );
    }
}

fn grove_coordinates(input: &[(usize, i64)]) -> i64 {
    let zero_idx = input.iter().position(|&(_, val)| val == 0).unwrap();
    [1000, 2000, 3000]
        .into_iter()
        .map(|i| input[(i + zero_idx) % input.len()].1)
        .sum()
}

fn part1(mut input: Vec<(usize, i64)>) -> i64 {
    for i in 0..input.len() {
        let j = input
            .iter()
            .position(|&(initial_pos, _)| i == initial_pos)
            .unwrap();
        let value = input[j].1;
        mix(&mut input, j, value);
    }
    grove_coordinates(&input)
}

fn part2(mut input: Vec<(usize, i64)>) -> i64 {
    input.iter_mut().for_each(|i| i.1 *= 811_589_153);
    for _ in 0..10 {
        for i in 0..input.len() {
            let j = input
                .iter()
                .position(|&(initial_pos, _)| i == initial_pos)
                .unwrap();
            let value = input[j].1;
            mix(&mut input, j, value);
        }
    }
    grove_coordinates(&input)
}

fn parse_input(input: &str) -> Vec<(usize, i64)> {
    input
        .trim()
        .lines()
        .enumerate()
        .map(|(i, s)| (i, s.parse().unwrap()))
        .collect()
}

#[cfg(not(test))]
fn main() {
    let input = include_str!("../inputs/day20.txt");
    let input = parse_input(input);

    let (p1, p2) = rayon::join(|| part1(input.clone()), || part2(input.clone()));
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
1
2
-3
3
-2
0
4
    "#;

    #[test]
    fn given_part1_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part1(input), 3);
    }

    #[test]
    fn given_part2_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part2(input), 1623178306);
    }
}
