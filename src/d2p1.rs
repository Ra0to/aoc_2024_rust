// Problem: https://adventofcode.com/2024/day/2

use std::fs::read_to_string;

const MAX_DIFF: i32 = 3;

#[allow(dead_code)]
pub fn read_input() -> Vec<Vec<i32>> {
    read_to_string("./inputs/d2")
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|pair| pair.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

#[allow(dead_code)]
pub fn solve(input: Vec<Vec<i32>>) -> usize {
    let mut safe_lines = 0;
    for line in input {
        if is_valid_line(&line) {
            safe_lines += 1;
        }
    }

    safe_lines
}

pub fn is_valid_line(line: &Vec<i32>) -> bool {
    let mut last_diff: Option<i32> = None;
    for slice in line.windows(2) {
        let cur_diff = slice[1] - slice[0];

        if cur_diff == 0 {
            return false;
        }
        if cur_diff.abs() > MAX_DIFF {
            return false;
        }

        match last_diff {
            None => {
                last_diff = Some(cur_diff);
                continue;
            }

            Some(prev_diff) => {
                if cur_diff.signum() != prev_diff.signum() {
                    return false;
                }
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Given
        let input = vec![
            vec![7, 6, 4, 2, 1], //
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        let answer = 2;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_2_invalid_diff_at_the_beginning() {
        // Given
        let input = vec![
            vec![0, 0], //
        ];
        let answer = 0;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d2p1_answer")
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
