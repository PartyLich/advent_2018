//! Solutions to 2018 day 7 problems part two
//! --- Day 7: The Sum of Its Parts ---
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::read_file;

use super::parse_line;

/// get the duration for a step with the supplied label
fn get_time(ch: char) -> usize {
    ch as usize - 64 + 60
}

#[derive(Debug, Default)]
struct State {
    /// Steps that have already been utilized (as an unordered set)
    used: HashSet<char>,
    /// Remaining steps
    remaining: HashMap<char, HashSet<char>>,
    /// Steps whose preconditions have already been met
    available: BinaryHeap<Reverse<char>>,
    /// Steps currently being worked on paired with their completion time
    in_progress: HashSet<(char, usize)>,
    /// Maximum available workers to complete steps
    worker_max: usize,
    /// Time
    time: usize,
}

impl Iterator for State {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let State {
            remaining,
            available,
            used,
            worker_max,
            in_progress,
            time,
        } = self;

        // move completed steps
        in_progress.retain(|(ch, completion)| {
            if *completion <= *time {
                used.insert(*ch);
                false
            } else {
                true
            }
        });

        // find available
        remaining.retain(|ch, ancestors| {
            if ancestors.is_subset(used) {
                // add removed items to available set
                available.push(Reverse(*ch));
                false
            } else {
                true
            }
        });

        // start work on available steps
        while (in_progress.len() < *worker_max) && !available.is_empty() {
            let ch = available.pop().unwrap().0;
            let completion = *time + get_time(ch);
            in_progress.insert((ch, completion));
        }

        if in_progress.is_empty() {
            // nothing left to do
            return None;
        }

        *time = in_progress
            .iter()
            .min_by(|a, b| a.1.cmp(&b.1))
            .map(|(_, time)| *time)
            .unwrap();

        Some(*time)
    }
}

/// returns the completion time (in seconds) of the assembly steps
pub fn two(file_path: &str) -> usize {
    const WORKERS: usize = 5;
    let remaining: HashMap<char, HashSet<char>> = read_file(file_path)
        .lines()
        .map(|line| parse_line(line).unwrap())
        .fold(
            HashMap::new(),
            // build ancestor sets
            |mut acc, next| {
                let parent = acc.entry(next.0).or_default().clone();
                let ancestors = acc.entry(next.1).or_default();
                ancestors.extend([next.0]);
                ancestors.extend(parent);

                acc
            },
        );
    let iter = State {
        remaining,
        worker_max: WORKERS,
        ..Default::default()
    };

    iter.last().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn finds_time() {
        let msg = "should return the duration for a given step label";
        let expected = 61;
        let actual = get_time('A');
        assert_eq!(actual, expected, "{}", msg);

        let expected = 86;
        let actual = get_time('Z');
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_two() {
        let msg = "should return the total assembly time in seconds";

        let remaining: HashMap<char, HashSet<char>> = read_file("input/7-t.txt")
            .lines()
            .map(|line| parse_line(line).unwrap())
            .fold(HashMap::new(), |mut acc, next| {
                let parent = acc.entry(next.0).or_default().clone();
                let ancestors = acc.entry(next.1).or_default();
                ancestors.extend([next.0]);
                ancestors.extend(parent);

                acc
            });
        let iter = State {
            remaining,
            worker_max: 2,
            ..Default::default()
        };

        let expected = 258;
        let actual = iter.last().unwrap();
        assert_eq!(actual, expected, "{}", msg);
    }
}
