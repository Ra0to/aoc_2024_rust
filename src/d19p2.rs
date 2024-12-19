// Problem: https://adventofcode.com/2024/day/19

use crate::d19p1;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn read_input() -> (Vec<String>, Vec<String>) {
    d19p1::read_input()
}

#[allow(dead_code)]
pub fn solve(input: (Vec<String>, Vec<String>)) -> usize {
    let available = input.0;
    let puzzles = input.1;

    let mut visited = HashMap::new();
    puzzles
        .iter()
        .map(|puzzle| {
            visited.clear();
            d19p1::calc_variants(&available, puzzle, &mut visited)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;
    use crate::d19p1::parse_input;

    #[test]
    fn test_1() {
        // Given
        let input = parse_input(
            "
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
",
        );
        let answer = 16;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d19p2_answer").unwrap();
        let answer = answer.trim().parse::<usize>().unwrap();

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }
}
