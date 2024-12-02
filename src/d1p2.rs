// Problem: https://adventofcode.com/2024/day/1

use crate::d1p1;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn read_input() -> Vec<(i32, i32)> {
    d1p1::read_input().unwrap()
}

#[allow(dead_code)]
pub fn solve(input: Vec<(i32, i32)>) -> i32 {
    solve_hash_map_optimized_without_unzip(input)
}

#[allow(dead_code)]
pub fn solve_hash_map(input: Vec<(i32, i32)>) -> i32 {
    let (left, right): (Vec<_>, Vec<_>) = input.into_iter().unzip();
    let mut count: HashMap<i32, i32> = HashMap::new();

    right.iter().for_each(|x| {
        count.insert(*x, count.get(x).copied().unwrap_or(0) + 1);
    });

    left.iter()
        .map(|x| x * count.get(x).copied().unwrap_or(0))
        .sum()
}

#[allow(dead_code)]
pub fn solve_hash_map_optimized(input: Vec<(i32, i32)>) -> i32 {
    let (left, right): (Vec<_>, Vec<_>) = input.into_iter().unzip();
    let mut count: HashMap<i32, i32> = HashMap::with_capacity(right.len());

    right.iter().for_each(|x| {
        count.entry(*x).and_modify(|e| *e += 1).or_insert(1);
    });

    left.iter().map(|x| x * *count.get(x).unwrap_or(&0)).sum()
}

#[allow(dead_code)]
pub fn solve_count_each_time(input: Vec<(i32, i32)>) -> i32 {
    let (left, right): (Vec<_>, Vec<_>) = input.into_iter().unzip();

    left.iter()
        .map(|x| x * right.iter().filter(|e| *e == x).count() as i32)
        .sum()
}

#[allow(dead_code)]
pub fn solve_count_with_hash(input: Vec<(i32, i32)>) -> i32 {
    let (left, right): (Vec<_>, Vec<_>) = input.into_iter().unzip();
    let mut count: HashMap<i32, i32> = HashMap::with_capacity(right.len());

    left.iter()
        .map(|x| {
            *x * *count
                .entry(*x)
                .or_insert(right.iter().filter(|e| *e == x).count() as i32)
        })
        .sum()
}

#[allow(dead_code)]
pub fn solve_hash_map_optimized_without_unzip(input: Vec<(i32, i32)>) -> i32 {
    let mut count: HashMap<i32, i32> = HashMap::with_capacity(input.len());

    input.iter().for_each(|(_, x)| {
        count.entry(*x).and_modify(|e| *e += 1).or_insert(1);
    });

    input
        .iter()
        .map(|(x, _)| x * *count.get(x).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

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
        let answer = read_to_string("./inputs/d1p2_answer")
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }
}
