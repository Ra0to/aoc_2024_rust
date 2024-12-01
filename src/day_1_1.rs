// Problem: https://adventofcode.com/2024/day/1

use std::fs::read_to_string;

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
    solve_sort_unstable(input)
}

#[allow(dead_code)]
pub fn solve_sort(input: Vec<(i32, i32)>) -> i32 {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input.into_iter().unzip();
    left.sort();
    right.sort();
    left.iter()
        .zip(right)
        .into_iter()
        .map(|(a, b)| (b - a).abs())
        .sum()
}

#[allow(dead_code)]
pub fn solve_sort_unstable(input: Vec<(i32, i32)>) -> i32 {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input.into_iter().unzip();
    left.sort_unstable();
    right.sort_unstable();
    left.iter()
        .zip(right)
        .into_iter()
        .map(|(a, b)| (b - a).abs())
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
        let answer = 11;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_2() {
        // Given
        let input = vec![
            (1, 2), //
            (3, 1),
        ];
        let answer = 1;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_3() {
        // Given
        let input = vec![
            (1, 2), //
            (3, 2),
        ];
        let answer = 2;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = 2192892;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }
}
