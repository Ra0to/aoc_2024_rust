// Problem: https://adventofcode.com/2024/day/9

use std::fs::read_to_string;

#[allow(dead_code)]
pub fn read_input() -> Vec<u32> {
    parse_input(&read_to_string("./inputs/d9").unwrap())
}

pub fn parse_input(input: &str) -> Vec<u32> {
    input
        .trim()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .collect()
}

#[allow(dead_code)]
pub fn solve(input: Vec<u32>) -> u64 {
    let mut memo = Vec::with_capacity(input.len() * 9);

    for (index, x) in input.iter().enumerate() {
        let is_block = index % 2 == 0;
        let id = if is_block { Some(index / 2) } else { None };

        for _ in 0..*x {
            memo.push(id);
        }
    }
    let mut hash: u64 = 0;
    let mut l = 0;
    let mut r = memo.len() - 1;
    while r >= l {
        if let Some(left) = memo[l] {
            hash += left as u64 * l as u64;
            l += 1;
            continue;
        }

        if let Some(right) = memo[r] {
            memo[l] = Some(right);
        }

        r -= 1;
    }

    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Given
        let input = parse_input("2333133121414131402");
        let answer = 1928;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d9p1_answer")
            .unwrap()
            .trim()
            .parse::<u64>()
            .unwrap();

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }
}
