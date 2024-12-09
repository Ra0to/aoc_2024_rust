// Problem: https://adventofcode.com/2024/day/7

use rayon::prelude::*;

use crate::d7p1;

#[allow(dead_code)]
pub fn read_input() -> Vec<(i64, Vec<i64>)> {
    d7p1::read_input()
}

#[derive(Clone, Debug)]
pub enum Op {
    Add,
    Mul,
    Concat,
}

#[allow(dead_code)]
pub fn solve(input: Vec<(i64, Vec<i64>)>) -> i64 {
    // solve_generator(input)
    // solve_recursion(input)
    // solve_recursion_numeric_concatenation(input)
    // solve_generator_optimized(input)
    solve_recursion_parallel(input)
}

// ~1.9 s
#[allow(dead_code)]
pub fn solve_generator(input: Vec<(i64, Vec<i64>)>) -> i64 {
    input
        .iter()
        .filter(|(res, line)| try_find_ops(line, *res))
        .map(|(res, _)| res)
        .sum()
}

pub fn try_find_ops(nums: &[i64], res: i64) -> bool {
    let ops_cnt = nums.len() - 1;
    let max_ops = 3_i64.pow(ops_cnt as u32);
    for ops in 0..max_ops {
        let mut ops: Vec<Op> = get_ops_from_num(ops.try_into().unwrap());

        if ops.len() < ops_cnt {
            let missing_cnt = ops_cnt - ops.len();
            let mut new_ops = vec![Op::Add; missing_cnt];
            new_ops.append(&mut ops);
            ops = new_ops;
        }

        if calc_line(nums, &ops) == res {
            return true;
        }
    }

    false
}

pub fn calc_line(nums: &[i64], ops: &[Op]) -> i64 {
    let mut res = nums[0];
    for (op_index, num) in nums[1..].iter().enumerate() {
        let op = &ops[op_index];
        match op {
            Op::Add => res += num,
            Op::Mul => res *= num,
            Op::Concat => res = format!("{res}{num}").parse::<i64>().unwrap(),
        }
    }

    res
}

fn get_ops_from_num(mut x: u32) -> Vec<Op> {
    let mut result = vec![];
    let radix = 3_u32;

    loop {
        let m = x % radix;
        x /= radix;

        // will panic if you use a bad radix (< 2 or > 36).
        let op = match std::char::from_digit(m, radix).unwrap() {
            '0' => Op::Add,
            '1' => Op::Mul,
            _ => Op::Concat,
        };
        result.push(op);
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}

// ~238 ms
#[allow(dead_code)]
pub fn solve_recursion(input: Vec<(i64, Vec<i64>)>) -> i64 {
    input
        .iter()
        .filter(|(res, line)| try_find_ops_rec(line, *res, line[0], 1))
        .map(|(res, _)| res)
        .sum()
}

pub fn try_find_ops_rec(nums: &[i64], target: i64, current: i64, index: usize) -> bool {
    if index >= nums.len() {
        return current == target;
    }

    let val = nums[index];
    try_find_ops_rec(nums, target, current + val, index + 1)
        || try_find_ops_rec(nums, target, current * val, index + 1)
        || try_find_ops_rec(
            nums,
            target,
            format!("{current}{val}").parse::<i64>().unwrap(),
            index + 1,
        )
}

// ~16.9 ms
#[allow(dead_code)]
pub fn solve_recursion_numeric_concatenation(input: Vec<(i64, Vec<i64>)>) -> i64 {
    input
        .iter()
        .filter(|(res, line)| try_find_ops_rec_numeric_concat(line, *res, line[0], 1))
        .map(|(res, _)| res)
        .sum()
}

pub fn try_find_ops_rec_numeric_concat(
    nums: &[i64],
    target: i64,
    current: i64,
    index: usize,
) -> bool {
    if index >= nums.len() {
        return current == target;
    }

    let val = nums[index];
    try_find_ops_rec_numeric_concat(nums, target, current + val, index + 1)
        || try_find_ops_rec_numeric_concat(nums, target, current * val, index + 1)
        || try_find_ops_rec_numeric_concat(nums, target, concat_nums(current, val), index + 1)
}

pub fn concat_nums(lhs: i64, rhs: i64) -> i64 {
    lhs * 10_i64.pow(rhs.ilog10() + 1) + rhs
}

// ~3.3 ms
#[allow(dead_code)]
pub fn solve_recursion_parallel(input: Vec<(i64, Vec<i64>)>) -> i64 {
    input
        .par_iter()
        .filter(|(res, line)| try_find_ops_rec_numeric_concat(line, *res, line[0], 1))
        .map(|(res, _)| res)
        .sum()
}

// ~1.5 s
// Generator is slower than recursion because we should evaluate all line (execute all operations
// on any op change). Recursion insted stores previous calculations in the call stack.
//
// N - number of element in line
//
// Generator solution - O(N*3^N)
// Recursion solution - O(3^N)
#[allow(dead_code)]
pub fn solve_generator_optimized(input: Vec<(i64, Vec<i64>)>) -> i64 {
    input
        .iter()
        .filter(|(res, line)| try_find_ops_optimized(line, *res))
        .map(|(res, _)| res)
        .sum()
}

pub fn try_find_ops_optimized(nums: &[i64], res: i64) -> bool {
    let ops_cnt = nums.len() - 1;
    let max_ops = 3_i64.pow(ops_cnt as u32);
    (0..max_ops).any(|ops| calc_line_optimized(nums, ops) == res)
}

pub fn calc_line_optimized(nums: &[i64], mut seq: i64) -> i64 {
    let mut res = nums[0];
    for num in &nums[1..] {
        let op = seq % 3;
        seq /= 3;

        match op {
            0 => res += num,
            1 => res *= num,
            2 => res = format!("{res}{num}").parse::<i64>().unwrap(),
            _ => panic!("unexpected operation index: {op}"),
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_1() {
        // Given
        let input = vec![
            (190, vec![10, 19]),
            (3267, vec![81, 40, 27]),
            (83, vec![17, 5]),
            (156, vec![15, 6]),
            (7290, vec![6, 8, 6, 15]),
            (161011, vec![16, 10, 13]),
            (192, vec![17, 8, 14]),
            (21037, vec![9, 7, 18, 13]),
            (292, vec![11, 6, 16, 20]),
        ];
        let answer = 11387;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_2() {
        // Given
        let input = vec![
            (3, vec![5, 7, 0, 3]),
            //        *   *  +
        ];
        let answer = 3;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d7p2_answer")
            .unwrap()
            .trim()
            .parse::<i64>()
            .unwrap();

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }
}
