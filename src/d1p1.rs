// Problem: https://adventofcode.com/2024/day/1

use std::fs::read_to_string;

#[allow(dead_code)]
pub fn read_input() -> std::io::Result<Vec<(i32, i32)>> {
    Ok(read_to_string("./inputs/d1")?
        .lines()
        .map(|line| {
            line.split_once("   ")
                .map(|(left, right)| (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap()))
                .unwrap()
        })
        .collect::<Vec<(i32, i32)>>())
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
    left.iter().zip(right).map(|(a, b)| (b - a).abs()).sum()
}

#[allow(dead_code)]
pub fn solve_sort_unstable(input: Vec<(i32, i32)>) -> i32 {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input.into_iter().unzip();
    left.sort_unstable();
    right.sort_unstable();
    left.iter().zip(right).map(|(a, b)| (b - a).abs()).sum()
}

#[allow(dead_code)]
pub fn solve_sort_unstable_wo_zip(input: Vec<(i32, i32)>) -> i32 {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input.into_iter().unzip();
    left.sort_unstable();
    right.sort_unstable();
    left.iter()
        .enumerate()
        .map(|(i, a)| (right[i] - a).abs())
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
        let input = read_input().expect("can't read problem input");
        let answer = read_to_string("./inputs/d1p1_answer")
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
