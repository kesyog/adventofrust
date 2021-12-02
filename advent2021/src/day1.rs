fn part1(readings: &[i32]) -> usize {
    let mut n_increases = 0;
    for (reading, prev) in readings.iter().skip(1).zip(readings) {
        if reading > prev {
            n_increases += 1;
        }
    }
    n_increases
}

fn part2(readings: &[i32]) -> usize {
    let mut n_increases = 0;
    for (reading, prev) in readings.iter().skip(3).zip(readings) {
        if reading > prev {
            n_increases += 1;
        }
    }
    n_increases
}

fn main() {
    let input = include_str!("../inputs/day1.txt");
    let input: Vec<i32> = utils::find_all_integers(input);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [i32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn given_part1_input() {
        assert_eq!(part1(&INPUT), 7);
    }

    #[test]
    fn given_part2_input() {
        assert_eq!(part2(&INPUT), 5);
    }
}
