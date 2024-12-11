// Problem: https://adventofcode.com/2024/day/11

use crate::d11p1;

#[allow(dead_code)]
pub fn read_input() -> Vec<u64> {
    d11p1::read_input()
}

#[allow(dead_code)]
pub fn solve(input: Vec<u64>) -> usize {
    d11p1::solve_for_blinks(input, 75)
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d11p2_answer")
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
