// Problem: https://adventofcode.com/2024/day/4

use crate::{extensions::*, point::P};
use std::fs::read_to_string;

#[allow(dead_code)]
pub fn read_input() -> Vec<Vec<char>> {
    parse_input_string(read_to_string("./inputs/d4").unwrap())
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
    let directions = [
        P::up(),
        P::down(),
        P::left(),
        P::right(),
        P::up_left(),
        P::up_right(),
        P::down_left(),
        P::down_right(),
    ];

    for y in 0..height {
        let width = input[y].len();
        for x in 0..width {
            // Problem is 1000x1000 so we can safely cast indexes to i32
            let p = P::pair(x as i32, y as i32);
            if !test_symbol(&input, p, 'X') {
                continue;
            }

            count += directions
                .iter()
                .filter(|d| test_word(&input, p, **d))
                .count();
        }
    }

    count
}

pub fn test_word(input: &[Vec<char>], start: P, direction: P) -> bool {
    test_symbol(input, start, 'X')
        && test_symbol(input, start.add(direction), 'M')
        && test_symbol(input, start.add(direction.mul(2)), 'A')
        && test_symbol(input, start.add(direction.mul(3)), 'S')
}

pub fn test_symbol(input: &[Vec<char>], pos: P, ch: char) -> bool {
    input.get_by_p(pos).is_some_and(|target| target == &ch)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Given
        let input = "..X...
.SAMX.
.A..A.
XMAS.S
.X...."
            .to_string();
        let input = parse_input_string(input);
        let answer = 4;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_2() {
        // Given
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            .to_string();
        let input = parse_input_string(input);
        let answer = 18;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_3_horizontal_fwd() {
        // Given
        let input = "..XMAS..
........
........
........
........
........
........
........"
            .to_string();
        let input = parse_input_string(input);
        let answer = 1;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_4_horizontal_bwd() {
        // Given
        let input = "..SAMX..
........
........
........
........
........
........
........"
            .to_string();
        let input = parse_input_string(input);
        let answer = 1;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_5_vertical_fwd() {
        // Given
        let input = "........
....X...
....M...
....A...
....S...
........
........
........"
            .to_string();
        let input = parse_input_string(input);
        let answer = 1;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_5_vertical_bwd() {
        // Given
        let input = "........
....S...
....A...
....M...
....X...
........
........
........"
            .to_string();
        let input = parse_input_string(input);
        let answer = 1;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_6_diagonal_lr_fwd() {
        // Given
        let input = "........
........
..X.....
...M....
....A...
.....S..
........
........"
            .to_string();
        let input = parse_input_string(input);
        let answer = 1;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_7_diagonal_lr_bwd() {
        // Given
        let input = "........
........
..S.....
...A....
....M...
.....X..
........
........"
            .to_string();
        let input = parse_input_string(input);
        let answer = 1;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_8_diagonal_rl_fwd() {
        // Given
        let input = "........
........
.....X..
....M...
...A....
..S.....
........
........"
            .to_string();
        let input = parse_input_string(input);
        let answer = 1;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_9_diagonal_rl_bwd() {
        // Given
        let input = "........
........
.....S..
....A...
...M....
..X.....
........
........"
            .to_string();
        let input = parse_input_string(input);
        let answer = 1;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_10_overlapping() {
        // Given
        let input = "........
........
........
..XMAS..
..M.....
..A.....
..S.....
........"
            .to_string();
        let input = parse_input_string(input);
        let answer = 2;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_11_overlapping_at_the_beggining() {
        // Given
        let input = "XMAS....
.M......
..A.....
...S....
........
........
........
........"
            .to_string();
        let input = parse_input_string(input);
        let answer = 2;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_11_overlapping_at_the_end() {
        // Given
        let input = "........
........
........
........
.......S
.......A
.......M
....SAMX"
            .to_string();
        let input = parse_input_string(input);
        let answer = 2;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_12_three_words_overlap() {
        // Given
        let input = "........
........
........
........
....S..S
.....A.A
......MM
....SAMX"
            .to_string();
        let input = parse_input_string(input);
        let answer = 3;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_13_overlap_in_the_middle() {
        // Given
        let input = "........
...S....
...A....
..XMAS..
...X....
........
........
........"
            .to_string();
        let input = parse_input_string(input);
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
        let answer = read_to_string("./inputs/d4p1_answer")
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
