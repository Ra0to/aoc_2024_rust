// Problem: https://adventofcode.com/2024/day/9

use crate::d9p1;

#[allow(dead_code)]
pub fn read_input() -> Vec<u32> {
    d9p1::read_input()
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

    let mut r = memo.len() - 1;
    while r > 0 {
        if let None = memo[r] {
            r -= 1;
            continue;
        }

        let block_len = calc_block_len(&memo, r);
        let max_pos = r + 1 - block_len;
        if let Some(new_pos) = try_find_free_space(&memo, max_pos, block_len) {
            for i in 0..block_len {
                memo[new_pos + i] = memo[r - i].take();
            }
        }

        r = max_pos;
        if r == 0 {
            break;
        }
        r -= 1;
    }

    memo.iter()
        .enumerate()
        .map(|(pos, val)| val.unwrap_or(0) as u64 * pos as u64)
        .sum()
}

fn try_find_free_space(memo: &[Option<usize>], max_pos: usize, req: usize) -> Option<usize> {
    let mut pos = 0;
    while pos < max_pos {
        let block_len = calc_block_len(memo, pos);
        if memo[pos].is_none() && block_len >= req {
            return Some(pos);
        }

        pos += block_len;
    }

    None
}

fn calc_block_len(memo: &[Option<usize>], start: usize) -> usize {
    let to_right = if start == 0 {
        true
    } else if start == memo.len() - 1 {
        false
    } else if memo[start] == memo[start - 1] {
        false
    } else {
        true
    };

    let mut ind = start;
    let mut cnt = 0;

    while (0..memo.len()).contains(&ind) && memo[ind] == memo[start] {
        cnt += 1;
        if ind == 0 && !to_right {
            break;
        }
        if to_right {
            ind += 1;
        } else {
            ind -= 1;
        }
    }

    cnt
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;
    use crate::d9p1::parse_input;

    #[test]
    fn test_1() {
        // Given
        let input = parse_input("2333133121414131402");
        let answer = 2858;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d9p2_answer")
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
