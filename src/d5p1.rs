// Problem: https://adventofcode.com/2024/day/5

use std::{collections::HashMap, fs::read_to_string};

#[allow(dead_code)]
pub fn read_input() -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    parse_input_string(read_to_string("./inputs/d5").unwrap())
}

pub fn parse_input_string(input: String) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let mut rules: Vec<(usize, usize)> = vec![];
    let mut updates: Vec<Vec<usize>> = vec![];
    let mut is_updates_lines_started = false;

    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() {
            is_updates_lines_started = true;
            continue;
        }

        if is_updates_lines_started {
            updates.push(
                line.split(",")
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect(),
            );
        } else {
            rules.push(
                line.split_once("|")
                    .map(|(left, right)| {
                        (
                            left.parse::<usize>().unwrap(),
                            right.parse::<usize>().unwrap(),
                        )
                    })
                    .unwrap(),
            );
        }
    }

    (rules, updates)
}

#[allow(dead_code)]
pub fn solve(input: (Vec<(usize, usize)>, Vec<Vec<usize>>)) -> usize {
    let mut sum = 0;
    let rules = input.0;
    let updates = input.1;

    for manual in updates {
        let len = manual.len();
        if len % 2 == 0 {
            panic!("manual has even len! manual: `{manual:?}`");
        }
        let middle = manual[len / 2];

        let mut pages_hash = HashMap::<usize, usize>::new();
        for (index, page) in manual.into_iter().enumerate() {
            if pages_hash.contains_key(&page) {
                panic!("Page duplicate! page: `{page}`");
            }

            pages_hash.insert(page, index);
        }

        let is_satisfy = rules.iter().all(|(first, next)| {
            if !pages_hash.contains_key(&first) {
                return true;
            }
            if !pages_hash.contains_key(&next) {
                return true;
            }

            pages_hash[first] < pages_hash[next]
        });

        if !is_satisfy {
            continue;
        }

        sum += middle;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let answer = 143;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d5p1_answer")
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
