// Problem: https://adventofcode.com/2024/day/3

use crate::d3p1;
use std::iter::Peekable;

#[allow(dead_code)]
pub fn read_input() -> String {
    d3p1::read_input()
}

#[allow(dead_code)]
pub fn solve(input: String) -> i32 {
    let mut sum = 0;
    let mut iter = input.chars().peekable();
    let mut is_enabled = true;

    while iter.peek().is_some() {
        let res = d3p1::try_get_mul_res(&mut iter);
        if let Some(val) = res {
            if is_enabled {
                sum += val
            }
            continue;
        }
        let res = try_get_operation_state(&mut iter);
        if let Some(state) = res {
            is_enabled = state;
            continue;
        }
        iter.next();
    }

    sum
}

fn try_get_operation_state<I: Iterator<Item = char>>(iter: &mut Peekable<I>) -> Option<bool> {
    let mut is_enable = true;
    iter.next_if_eq(&'d')?;
    iter.next_if_eq(&'o')?;
    match iter.peek() {
        Some('(') => (),
        Some('n') => {
            if try_get_not(iter).is_none() {
                return None;
            }

            is_enable = false;
        }
        _ => return None,
    }
    iter.next_if_eq(&'(')?;
    iter.next_if_eq(&')')?;

    Some(is_enable)
}

fn try_get_not<I: Iterator<Item = char>>(iter: &mut Peekable<I>) -> Option<()> {
    iter.next_if_eq(&'n')?;
    iter.next_if_eq(&'\'')?;
    iter.next_if_eq(&'t')?;

    Some(())
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_1() {
        // Given
        let input =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string();
        let answer = 48;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_2_valid_mul() {
        // Given
        let input = "mul(1,2)".to_string();
        let answer = 2;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_3_valid_mul_with_max_digits() {
        // Given
        let input = "mul(111,222)".to_string();
        let answer = 24642;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_3_invalid_mul_space_in_params() {
        // Given
        let input = "mul(1, 2)".to_string();
        let answer = 0;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_4_invalid_mul_no_params() {
        // Given
        let input = "mul()".to_string();
        let answer = 0;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_5_do_not_disables_operations() {
        // Given
        let input = "don't()mul(1,2)".to_string();
        let answer = 0;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_6_do_enable_operations_back() {
        // Given
        let input = "don't()do()mul(1,2)".to_string();
        let answer = 2;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_7_mul_is_not_working_when_operations_disabled() {
        // Given
        let input = "don't()mul(1,2)do()mul(1,2)".to_string();
        let answer = 2;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    #[ignore = "not solved yet"]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d3p2_answer")
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
