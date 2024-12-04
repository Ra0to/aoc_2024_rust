// Problem: https://adventofcode.com/2024/day/4

use std::fs::read_to_string;

#[derive(Debug, Clone)]
pub struct P(i32, i32);

impl P {
    pub fn add(self, other: &Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }

    pub fn mul(self, mul: i32) -> Self {
        Self(self.0 * mul, self.1 * mul)
    }

    pub fn pair(x: i32, y: i32) -> Self {
        Self(x, y)
    }

    pub fn zero() -> Self {
        Self::pair(0, 0)
    }

    pub fn one() -> Self {
        Self::pair(1, 1)
    }

    pub fn up() -> Self {
        Self::pair(0, 1)
    }

    pub fn down() -> Self {
        Self::pair(0, -1)
    }

    pub fn left() -> Self {
        Self::pair(-1, 0)
    }

    pub fn right() -> Self {
        Self::pair(1, 0)
    }

    pub fn down_left() -> Self {
        Self::down().add(&Self::left())
    }

    pub fn down_right() -> Self {
        Self::down().add(&Self::right())
    }

    pub fn up_left() -> Self {
        Self::up().add(&Self::left())
    }

    pub fn up_right() -> Self {
        Self::up().add(&Self::right())
    }
}

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
            let p = P::pair(x as i32, y as i32);
            if !test_symbol(&input, p.clone(), 'X') {
                continue;
            }

            count += directions
                .iter()
                .filter(|d| test_word(&input, p.clone(), (*d).clone()))
                .count();
        }
    }

    count
}

pub fn test_word(input: &Vec<Vec<char>>, start: P, direction: P) -> bool {
    test_symbol(input, start.clone(), 'X')
        && test_symbol(input, start.clone().add(&direction.clone().mul(1)), 'M')
        && test_symbol(input, start.clone().add(&direction.clone().mul(2)), 'A')
        && test_symbol(input, start.clone().add(&direction.clone().mul(3)), 'S')
}

pub fn test_symbol(input: &Vec<Vec<char>>, pos: P, ch: char) -> bool {
    let y = pos.1;
    if !(0_i32..(input.len() as i32)).contains(&y) {
        return false;
    }
    let y = y as usize;

    input
        .get(y)
        .and_then(|line| {
            let x = pos.0;
            if !(0_i32..(line.len() as i32)).contains(&x) {
                return None;
            }

            let x = x as usize;
            line.get(x)
        })
        .is_some_and(|target| target == &ch)
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
    #[ignore = "not solved yet"]
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
