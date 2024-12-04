// Problem: https://adventofcode.com/2024/day/4

use crate::d4p1;
use crate::d4p1::{test_symbol, P};

#[allow(dead_code)]
pub fn read_input() -> Vec<Vec<char>> {
    d4p1::read_input()
}

pub fn parse_input_string(input: String) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect()
}

#[allow(dead_code)]
pub fn solve(input: Vec<Vec<char>>) -> usize {
    let mut count = 0;
    let height = input.len();

    for y in 0..height {
        let width = input[y].len();
        for x in 0..width {
            let p = P::pair(x as i32, y as i32);
            if !test_symbol(&input, p.clone(), 'A') {
                continue;
            }

            if test_x_shape(&input, p.clone()) {
                count += 1;
            }
        }
    }

    count
}

pub fn test_x_shape(input: &Vec<Vec<char>>, middle: P) -> bool {
    (test_mas_word(input, middle.clone().add(&P::up_left()), P::down_right())
        || test_mas_word(input, middle.clone().add(&P::down_right()), P::up_left()))
        && (test_mas_word(input, middle.clone().add(&P::up_right()), P::down_left())
            || test_mas_word(input, middle.clone().add(&P::down_left()), P::up_right()))
}

pub fn test_mas_word(input: &Vec<Vec<char>>, start: P, direction: P) -> bool {
    test_symbol(input, start.clone(), 'M')
        && test_symbol(input, start.clone().add(&direction.clone().mul(1)), 'A')
        && test_symbol(input, start.clone().add(&direction.clone().mul(2)), 'S')
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_1() {
        // Given
        let input = "M.S
.A.
M.S"
        .to_string();
        let input = parse_input_string(input);
        let answer = 1;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_2() {
        // Given
        let input = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
.........."
            .to_string();
        let input = parse_input_string(input);
        let answer = 9;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d4p2_answer")
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
