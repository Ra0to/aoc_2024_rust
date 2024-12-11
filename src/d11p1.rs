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

#[allow(dead_code)]
pub fn solve(input: Vec<u64>) -> usize {
    solve_for_blinks(input, 25)
}

#[allow(dead_code)]
pub fn solve_for_blinks(mut input: Vec<u64>, blinks_count: usize) -> usize {
    for _blink in 1..=blinks_count {
        let mut new_input = vec![];
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
    fn test_2() {
        // Given
        let input = vec![125, 17];

        // When

        // Then
        assert_eq!(solve_for_blinks(input.clone(), 1), 3);
        assert_eq!(solve_for_blinks(input.clone(), 2), 4);
        assert_eq!(solve_for_blinks(input.clone(), 3), 5);
        assert_eq!(solve_for_blinks(input.clone(), 4), 9);
        assert_eq!(solve_for_blinks(input.clone(), 5), 13);
        assert_eq!(solve_for_blinks(input.clone(), 6), 22);
        assert_eq!(solve_for_blinks(input.clone(), 25), 55312);
    }

    #[test]
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
