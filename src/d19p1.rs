// Problem: https://adventofcode.com/2024/day/19

use std::{collections::HashSet, fs::read_to_string};

#[allow(dead_code)]
pub fn read_input() -> (Vec<String>, Vec<String>) {
    parse_input(&read_to_string("./inputs/d19").unwrap())
}

pub fn parse_input(input: &str) -> (Vec<String>, Vec<String>) {
    let mut available = Vec::new();
    let mut puzzles = Vec::new();
    let lines = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty());

    for line in lines {
        if line.contains(", ") {
            available = line.split(", ").map(|pat| pat.to_string()).collect();
        } else {
            puzzles.push(line.to_string());
        }
    }

    (available, puzzles)
}

#[allow(dead_code)]
pub fn solve(input: (Vec<String>, Vec<String>)) -> usize {
    let available = input.0;
    let puzzles = input.1;

    let mut visited = HashSet::new();
    puzzles
        .iter()
        .filter(|puzzle| {
            visited.clear();
            can_be_created(&available, puzzle, &mut visited)
        })
        .count()
}

pub fn can_be_created(available: &[String], puzzle: &str, visited: &mut HashSet<usize>) -> bool {
    if visited.contains(&puzzle.len()) {
        return false;
    }

    if puzzle.is_empty() {
        return true;
    }

    for towel in available {
        let len = towel.len();
        if puzzle.starts_with(towel) && can_be_created(available, &puzzle[len..], visited) {
            return true;
        }
    }

    visited.insert(puzzle.len());

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Given
        let input = parse_input(
            "
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
",
        );
        let answer = 6;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d19p1_answer").unwrap();
        let answer = answer.trim().parse::<usize>().unwrap();

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }
}
