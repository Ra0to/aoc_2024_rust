// Problem: https://adventofcode.com/2024/day/5

use crate::d5p1;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

#[allow(dead_code)]
pub fn read_input() -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    d5p1::read_input()
}

#[allow(dead_code)]
pub fn solve(input: (Vec<(usize, usize)>, Vec<Vec<usize>>)) -> usize {
    let mut sum = 0;
    let rules: HashSet<(usize, usize)> = input.0.into_iter().collect();
    let updates = input.1;

    for manual in updates {
        let len = manual.len();
        if len % 2 == 0 {
            panic!("manual has even len! manual: `{manual:?}`");
        }

        let mut pages_hash = HashMap::<usize, usize>::new();
        for (index, page) in manual.iter().enumerate() {
            if pages_hash.contains_key(page) {
                panic!("Page duplicate! page: `{page}`");
            }

            pages_hash.insert(*page, index);
        }

        let is_satisfy = rules.iter().all(|(first, next)| {
            if !pages_hash.contains_key(first) {
                return true;
            }
            if !pages_hash.contains_key(next) {
                return true;
            }

            pages_hash[first] < pages_hash[next]
        });

        if is_satisfy {
            continue;
        }
        let mut new_manual = manual.clone();
        new_manual.sort_unstable_by(|left, right| {
            if rules.contains(&(*left, *right)) {
                return Ordering::Less;
            }
            if rules.contains(&(*right, *left)) {
                return Ordering::Greater;
            }

            Ordering::Equal
        });

        let middle = new_manual[len / 2];
        sum += middle;
    }

    sum
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;
    use crate::d5p1::parse_input_string;

    #[test]
    fn test_1() {
        // Given
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
            .to_string();
        let input = parse_input_string(input);
        let answer = 123;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d5p2_answer")
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }
}
