// Problem: https://adventofcode.com/2024/day/11

use std::fs::read_to_string;

#[allow(dead_code)]
pub fn read_input() -> Vec<u64> {
    read_to_string("./inputs/d11")
        .unwrap()
        .trim()
        .split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}

const TOTAL_BLINKS: usize = 25;

#[allow(dead_code)]
pub fn solve(mut input: Vec<u64>) -> usize {
    let mut new_input = vec![];
    for _blink in 1..=TOTAL_BLINKS {
        for stone in &input {
            if *stone == 0 {
                new_input.push(1);
                continue;
            } else if stone.to_string().len() % 2 == 0 {
                let rep = stone.to_string();
                let len = rep.len();
                let half_len = len / 2;
                new_input.push(rep[..half_len].parse::<u64>().unwrap());
                new_input.push(rep[half_len..].parse::<u64>().unwrap());
            } else {
                new_input.push(stone * 2024);
            }
        }

        input = new_input;
        new_input = vec![];
    }

    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Given
        let input = vec![125, 17];
        let answer = 55312;

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
        let answer = read_to_string("./inputs/d11p1_answer")
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
