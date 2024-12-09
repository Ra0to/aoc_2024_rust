// Problem: https://adventofcode.com/2024/day/9

use std::{cmp::Reverse, collections::BinaryHeap};

use crate::d9p1;

#[allow(dead_code)]
pub fn read_input() -> Vec<usize> {
    d9p1::read_input()
}

#[allow(dead_code)]
pub fn solve(input: Vec<usize>) -> u64 {
    let mut memo = Vec::with_capacity(input.len() * 9);
    let mut free_spaces = [const { BinaryHeap::<Reverse<usize>>::new() }; 10];

    for (index, x) in input.iter().enumerate() {
        let is_block = index % 2 == 0;
        let id = if is_block { Some(index / 2) } else { None };

        if !is_block {
            free_spaces[*x].push(Reverse(memo.len()));
        }

        for _ in 0..*x {
            memo.push(id);
        }
    }

    let mut r = memo.len() - 1;
    while r > 0 {
        if memo[r].is_none() {
            r -= 1;
            continue;
        }

        let block_len = calc_block_len(&memo, r);
        let max_pos = r + 1 - block_len;
        if let Some(new_pos) = try_find_free_space(&mut free_spaces, max_pos, block_len) {
            swap(&mut memo, new_pos, r, block_len);
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

fn swap(memo: &mut [Option<usize>], left: usize, right: usize, len: usize) {
    for i in 0..len {
        memo.swap(left + i, right - i);
    }
}

fn try_find_free_space(
    free_space: &mut [BinaryHeap<Reverse<usize>>],
    max_pos: usize,
    req: usize,
) -> Option<usize> {
    let mut candidate_len = None;
    let mut candidate_ind = None;

    for len in req..free_space.len() {
        let heap = &mut free_space[len];
        match heap.peek() {
            None => continue,
            Some(ind) if ind.0 > max_pos => continue,
            Some(ind) => {
                if candidate_len.is_none() {
                    candidate_len = Some(len);
                    candidate_ind = Some(ind.0);
                    continue;
                }

                if candidate_ind.unwrap() > ind.0 {
                    candidate_len = Some(len);
                    candidate_ind = Some(ind.0);
                }
            }
        }
    }

    if let Some(len) = candidate_len {
        let heap = &mut free_space[len];
        let index = heap.pop().unwrap().0;
        let new_len = len - req;
        if new_len > 0 {
            free_space[new_len].push(Reverse(index + req));
        }
        return Some(index);
    }

    None
}

fn calc_block_len(memo: &[Option<usize>], start: usize) -> usize {
    let to_right = if start == 0 {
        true
    } else if start == memo.len() - 1 {
        false
    } else {
        memo[start] != memo[start - 1]
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
    fn test_2() {
        // Given
        let input = parse_input("23122");
        let answer = 14;

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
