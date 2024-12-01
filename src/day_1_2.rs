// Problem: https://adventofcode.com/2024/day/1

use std::{collections::HashMap, fs::read_to_string};

#[allow(dead_code)]
pub fn read_input() -> Vec<(i32, i32)> {
    read_to_string("./inputs/day_1_1/input")
        .unwrap()
        .lines()
        .map(|line| {
            let split = line
                .split_whitespace()
                .map(|pair| pair.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            (split[0], split[1])
        })
        .collect::<Vec<(i32, i32)>>()
}

#[allow(dead_code)]
pub fn solve(input: Vec<(i32, i32)>) -> i32 {
    solve_hash_map_optimized(input)
}

#[allow(dead_code)]
pub fn solve_hash_map(input: Vec<(i32, i32)>) -> i32 {
    let (left, right): (Vec<_>, Vec<_>) = input.into_iter().unzip();
    let mut count: HashMap<i32, i32> = HashMap::new();

    right.iter().for_each(|x| {
        count.insert(x.clone(), count.get(x).map(|v| v.clone()).unwrap_or(0) + 1);
    });

    left.iter()
        .map(|x| x * count.get(&x).map(|v| v.clone()).unwrap_or(0))
        .sum()
}

#[allow(dead_code)]
pub fn solve_hash_map_optimized(input: Vec<(i32, i32)>) -> i32 {
    let (left, right): (Vec<_>, Vec<_>) = input.into_iter().unzip();
    let mut count: HashMap<i32, i32> = HashMap::with_capacity(right.len());

    right.iter().for_each(|x| {
        count.entry(*x).and_modify(|e| *e += 1).or_insert(1);
    });

    left.iter()
        .map(|x| x * count.get(&x).map(|v| v.clone()).unwrap_or(0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Given
        let input = vec![
            (3, 4), //
            (4, 3),
            (2, 5),
            (1, 3),
            (3, 9),
            (3, 3),
        ];
        let answer = 31;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = 22962826;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }
}
