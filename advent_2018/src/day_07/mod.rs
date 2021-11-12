//! Solutions to 2018 day 7 problems
//! --- Day 7: The Sum of Its Parts ---
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet};

use lazy_static::lazy_static;
use regex::Regex;

use crate::read_file;

/// Parse a step and its precondition from str
fn parse_line(input: &str) -> Result<(char, char), &'static str> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r#"Step (\w) must be finished before step (\w) can begin."#).unwrap();
    }

    RE.captures(input)
        .map(|captures| {
            let parent = captures.get(1).unwrap().as_str().chars().next().unwrap();
            let child = captures.get(2).unwrap().as_str().chars().next().unwrap();
            (parent, child)
        })
        .ok_or("Failed to parse rule")
}

/// An instruction step (order, label) pair
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct OrderedStep(usize, char);

#[derive(Debug, Default)]
struct State {
    /// Ordered set of steps
    order: BTreeSet<OrderedStep>,
    /// Steps that have already been utilized (as an unordered set)
    used: HashSet<char>,
    /// Remaining steps
    remaining: HashMap<char, HashSet<char>>,
    /// Steps whose preconditions have already been met
    available: BinaryHeap<Reverse<char>>,
}

/// An iterator over the ordered assembly step labels
struct StateIter(State);

impl Iterator for StateIter {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let State {
            remaining,
            available,
            used,
            order,
        } = &mut self.0;

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

        // select next used
        if let Some(next) = available.pop() {
            used.insert(next.0);
            order.insert(OrderedStep(used.len(), next.0));

            Some(next.0)
        } else {
            None
        }
    }
}

/// returns the order of the assembly steps
pub fn one(file_path: &str) -> String {
    let input = read_file(file_path);
    let remaining: HashMap<char, HashSet<char>> =
        input.lines().map(|line| parse_line(line).unwrap()).fold(
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

    StateIter(State {
        remaining,
        ..Default::default()
    })
    .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the order of the assembly steps";
        let expected = "CABDFE".to_string();
        let actual = one("input/7-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
