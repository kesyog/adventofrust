//! Solution to [AoC 2018 Day 4](https://adventofcode.com/2018/day/4)

use regex::Regex;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::default::Default;
use std::iter::Iterator;
use utils::counter::Counter;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Default)]
struct Guard(usize);

/// Find the guard that slept the most. Then find the minute _that_ guard slept the most.
/// Return the product of the two as desired by the problem.
fn part1(minute_counters: &[Counter<Guard>]) -> usize {
    // Calculate the number of minutes slept per guard across all minute slots
    let guard_counter: Counter<Guard> =
        minute_counters
            .iter()
            .fold(Counter::new(), |mut acc, element| {
                acc += element;
                acc
            });
    let target_guard = guard_counter.find_max_count().unwrap().0;
    let target_minute = minute_counters
        .iter()
        .enumerate()
        .max_by_key(|(_, counter)| counter.get(target_guard))
        .unwrap()
        .0;
    target_guard.0 * target_minute
}

/// Find the guard + minute pairing that had the most sleep occurrences
/// Return the product of the two as desired by the problem.
fn part2(minute_counters: &[Counter<Guard>]) -> usize {
    let (minute, guard, _count) = minute_counters
        .iter()
        .enumerate()
        .filter_map(|(minute, counter)| {
            if let Some((guard, count)) = counter.find_max_count() {
                Some((minute, *guard, *count))
            } else {
                // Filter out minutes where no guards slept
                None
            }
        })
        .max_by_key(|(_minute, _guard, count)| *count)
        .unwrap();

    minute * guard.0
}

/// Store sleep data as a vector of Counters where each index of the counter corresponds to a
/// minute from 0-59 and each Counter tracks the number of times each guard slept during that
/// minute.
fn store_sleep_data(mut sorted_logs: BinaryHeap<Reverse<&str>>) -> Vec<Counter<Guard>> {
    let guard_num_re = Regex::new(r"#\d+").unwrap();
    let mut active_guard: Option<Guard> = None;
    let mut start_sleep_time: Option<usize> = None;

    let mut per_minute_counters: Vec<Counter<Guard>> = Vec::new();
    per_minute_counters.resize_with(60, Default::default);

    while sorted_logs.peek() != None {
        let line = sorted_logs.pop().unwrap().0.trim();
        let minute = line[15..=16].parse::<usize>().unwrap();

        if let Some(result) = guard_num_re.find(line) {
            // A new guard started
            // Cleanup any unlogged sleep minutes from the previous guard
            if let (Some(last_guard), Some(last_sleep_time)) = (active_guard, start_sleep_time) {
                for counter in &mut per_minute_counters[last_sleep_time..60] {
                    counter.add(last_guard)
                }
                start_sleep_time = None;
            }
            // Set new guard
            active_guard = Some(Guard(result.as_str()[1..].parse::<usize>().unwrap()));
        } else if line.contains("wakes") {
            // Guard woke up
            assert_ne!(start_sleep_time, None);

            for counter in &mut per_minute_counters[start_sleep_time.unwrap()..minute] {
                counter.add(active_guard.unwrap());
            }
            start_sleep_time = None;
        } else {
            // Guard fell asleep
            start_sleep_time = Some(minute);
        }
    }

    per_minute_counters
}

/// Parse input log file into sorted binary min-heap. Relies on lexographically sorting the lines of
/// the input to sort the logs chronologically.
fn parse_to_heap(input: &str) -> BinaryHeap<Reverse<&str>> {
    input.trim().split_terminator('\n').map(Reverse).collect()
}

fn main() {
    // A neat property of this macro and the parsing implementation is that we only work on
    // references to the string. No copies or allocations are needed until the data is extracted
    // out.
    let input = include_str!("../inputs/day4.txt");
    let data = parse_to_heap(input);
    let minute_counters = store_sleep_data(data);
    println!("Part 1: {:?}", part1(&minute_counters));
    println!("Part 2: {:?}", part2(&minute_counters));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
    "#;

    #[test]
    fn given_part1_input() {
        let data = parse_to_heap(TEST_INPUT);
        let minute_counters = store_sleep_data(data);
        assert_eq!(240, part1(&minute_counters));
    }

    #[test]
    fn given_part2_input() {
        let data = parse_to_heap(TEST_INPUT);
        let minute_counters = store_sleep_data(data);
        assert_eq!(4455, part2(&minute_counters));
    }
}
