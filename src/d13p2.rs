// Problem: https://adventofcode.com/2024/day/13

use crate::d13p1;
use crate::d13p1::{try_find_solution, ClawMachineDefinition};

#[allow(dead_code)]
pub fn read_input() -> Vec<ClawMachineDefinition> {
    d13p1::read_input()
}

#[allow(dead_code)]
pub fn solve(input: Vec<ClawMachineDefinition>) -> u128 {
    input
        .into_iter()
        .filter_map(|def| try_find_solution(&def, None, Some(10000000000000)))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d13p2_answer")
            .unwrap()
            .trim()
            .parse::<u128>()
            .unwrap();

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }
}
