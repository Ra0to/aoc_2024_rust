// Problem: https://adventofcode.com/2024/day/3

use std::{fs::read_to_string, iter::Peekable};

use regex::Regex;

#[allow(dead_code)]
pub fn read_input() -> String {
    read_to_string("./inputs/d3").unwrap()
}

#[allow(dead_code)]
pub fn solve(input: String) -> i32 {
    solve_regex(input)
}

#[allow(dead_code)]
pub fn solve_iter(input: String) -> i32 {
    let mut sum = 0;
    let mut iter = input.chars().peekable();

    while iter.peek().is_some() {
        let res = try_get_mul_res(&mut iter);
        match res {
            Some(val) => sum += val,
            _ => {
                iter.next();
            }
        }
    }

    sum
}

pub fn try_get_mul_res<I: Iterator<Item = char>>(iter: &mut Peekable<I>) -> Option<i32> {
    try_get_mul(iter).map(|(left, right)| left * right)
}

pub fn try_get_mul<I: Iterator<Item = char>>(iter: &mut Peekable<I>) -> Option<(i32, i32)> {
    iter.next_if_eq(&'m')?;
    iter.next_if_eq(&'u')?;
    iter.next_if_eq(&'l')?;
    iter.next_if_eq(&'(')?;
    let left = try_get_num(iter)?;
    iter.next_if_eq(&',')?;
    let right = try_get_num(iter)?;
    iter.next_if_eq(&')')?;

    Some((left, right))
}

pub fn try_get_num<I: Iterator<Item = char>>(iter: &mut Peekable<I>) -> Option<i32> {
    let d = iter
        .next_if(|ch| ch.is_numeric())
        .and_then(|x| x.to_digit(10))? as i32;
    let mut num = d;
    match iter
        .next_if(|ch| ch.is_numeric())
        .and_then(|x| x.to_digit(10))
    {
        None => {
            return Some(num);
        }
        Some(d) => {
            num = num * 10 + d as i32;
        }
    }
    match iter
        .next_if(|ch| ch.is_numeric())
        .and_then(|x| x.to_digit(10))
    {
        None => {
            return Some(num);
        }
        Some(d) => {
            num = num * 10 + d as i32;
        }
    }
    Some(num)
}

#[allow(dead_code)]
pub fn solve_regex(input: String) -> i32 {
    Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
        .expect("regex pattern should be valid")
        .captures_iter(&input)
        .map(|c| c.extract())
        .map(|(_, [left, right])| {
            left.parse::<i32>()
                .expect("matched number should be valid int")
                * right
                    .parse::<i32>()
                    .expect("matched number should be valid int")
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Given
        let input =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string();
        //            ^^^^^^^^                    ^^^^^^^^                ^^^^^^^^|^^^^^^^^
        //                8                          25                       88       40
        let answer = 161;

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
    fn test_5_invalid_mul_missing_left_param() {
        // Given
        let input = "mul(,2)".to_string();
        let answer = 0;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_6_invalid_mul_missing_right_param() {
        // Given
        let input = "mul(1,)".to_string();
        let answer = 0;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_7_invalid_mul_invalid_keywoard() {
        // Given
        let input = "mu(1,2)".to_string();
        let answer = 0;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_8_invalid_mul_missing_left_bracket() {
        // Given
        let input = "mul1,2)".to_string();
        let answer = 0;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_9_invalid_mul_missing_right_bracket() {
        // Given
        let input = "mul(1,2".to_string();
        let answer = 0;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_10_invalid_mul_missing_brackets() {
        // Given
        let input = "mul1,2".to_string();
        let answer = 0;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_11_invalid_mul_double_brackets() {
        // Given
        let input = "mul((1,2))".to_string();
        let answer = 0;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_12_two_valid_muls() {
        // Given
        let input = "mul(1,2)mul(1,2)".to_string();
        let answer = 4;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_13_two_valid_muls_separated_by_invalid_character() {
        // Given
        let input = "mul(1,2) mul(1,2)".to_string();
        let answer = 4;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_14_invalid_mul_wrong_brackets() {
        // Given
        let input = "mul[1,2]".to_string();
        let answer = 0;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_15_two_valid_muls_separated_by_new_line() {
        // Given
        let input = "mul(1,2)\nmul(1,2)".to_string();
        let answer = 4;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_16_two_valid_muls_separated_by_backslash() {
        // Given
        let input = r"mul(1,2)\mul(1,2)".to_string();
        let answer = 4;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_17_invalid_mul_negative_number() {
        // Given
        let input = "mul(1,-2)".to_string();
        let answer = 0;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_18_valid_mul_with_zero() {
        // Given
        let input = "mul(1,0)".to_string();
        let answer = 0;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_19_invalid_mul_too_many_digits() {
        // Given
        let input = "mul(1111,2)".to_string();
        let answer = 0;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_20_valid_mul_full_zeroes() {
        // Given
        let input = "mul(000,000)".to_string();
        let answer = 0;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_21_valid_mul_with_prefix_zeroes() {
        // Given
        let input = "mul(001,002)".to_string();
        let answer = 2;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_22_invalid_empty_input() {
        // Given
        let input = "".to_string();
        let answer = 0;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_23_invalid_params() {
        // Given
        let input = "mul(a,b)".to_string();
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
        let answer = read_to_string("./inputs/d3p1_answer")
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
