// Problem: https://adventofcode.com/2024/day/2

use crate::d2p1;

const MAX_DIFF: i32 = 3;

#[allow(dead_code)]
pub fn read_input() -> Vec<Vec<i32>> {
    d2p1::read_input()
}

#[allow(dead_code)]
pub fn solve(input: Vec<Vec<i32>>) -> usize {
    let mut safe_lines = 0;
    for line in input {
        if test_all_line_variants(&line) {
            safe_lines += 1;
        }
    }

    safe_lines
}

fn test_all_line_variants(line: &Vec<i32>) -> bool {
    if is_valid_line(line) {
        return true;
    }

    for i in 0..line.len() {
        let mut new_line = line.clone();
        new_line.remove(i);
        if is_valid_line(&new_line) {
            return true;
        }
    }

    false
}
fn is_valid_line(line: &Vec<i32>) -> bool {
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
    use std::fs::read_to_string;

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
        let answer = 4;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_2_invalid_diff_at_the_beginning_can_be_skipped() {
        // Given
        let input = vec![
            vec![0, 0], //
        ];
        let answer = 1;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_3_error_at_the_end_can_be_skipped() {
        // Given
        let input = vec![
            vec![0, 1, 9], //
        ];
        let answer = 1;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_4_error_at_the_beginning_can_be_skipped() {
        // Given
        let input = vec![
            vec![9, 0, 1], //
        ];
        let answer = 1;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_5_error_in_the_middle_can_be_skipped() {
        // Given
        let input = vec![
            vec![0, 9, 1], //
        ];
        let answer = 1;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_6_error_in_the_middle_can_not_be_skipped() {
        // Given
        let input = vec![
            vec![0, 9, 5], //
        ];
        let answer = 0;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_7_error_can_not_be_skipped() {
        // Given
        let input = vec![
            vec![1, 1, 1], //
        ];
        let answer = 0;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_8() {
        // Given
        let input = vec![
            vec![1, 1, 2], //
        ];
        let answer = 1;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_9() {
        // Given
        let input = vec![
            vec![3, 1, 2, 1, 0], //
        ];
        let answer = 1;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d2p2_answer")
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
