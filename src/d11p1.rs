// Problem: https://adventofcode.com/2024/day/11

use std::{collections::HashMap, fs::read_to_string};

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
pub fn solve_for_blinks(input: Vec<u64>, blinks_count: usize) -> usize {
    input
        .iter()
        .map(|stone| ans(*stone, blinks_count, &mut HashMap::new()))
        .sum()
}

fn ans(val: u64, blinks_count: usize, memo: &mut HashMap<(u64, usize), usize>) -> usize {
    if blinks_count == 0 {
        return 1;
    }
    if let Some(ans) = memo.get(&(val, blinks_count)) {
        return *ans;
    }

    let res = match val {
        _ if val == 0 => ans(1, blinks_count - 1, memo),
        _ if val.to_string().len() % 2 == 0 => {
            let rep = val.to_string();
            let len = rep.len();
            let half_len = len / 2;
            let left = rep[..half_len].parse::<u64>().unwrap();
            let right = rep[half_len..].parse::<u64>().unwrap();

            ans(left, blinks_count - 1, memo) + ans(right, blinks_count - 1, memo)
        }
        _ => ans(val * 2024, blinks_count - 1, memo),
    };
    memo.insert((val, blinks_count), res);

    res
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
