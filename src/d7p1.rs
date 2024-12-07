// Problem: https://adventofcode.com/2024/day/7

use std::fs::read_to_string;

#[allow(dead_code)]
pub fn read_input() -> Vec<(i64, Vec<i64>)> {
    read_to_string("./inputs/d7")
        .unwrap()
        .lines()
        .map(|line| {
            let (raw_sum, raw_nums) = line.split_once(":").unwrap();
            (
                raw_sum.parse::<i64>().unwrap(),
                raw_nums
                    .split_whitespace()
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect(),
            )
        })
        .collect()
}

#[derive(Clone, Debug)]
pub enum Op {
    Add,
    Mul,
}

#[allow(dead_code)]
pub fn solve(input: Vec<(i64, Vec<i64>)>) -> i64 {
    input
        .iter()
        .filter(|(res, line)| try_find_ops(line, *res))
        .map(|(res, _)| res)
        .sum()
}

pub fn try_find_ops(nums: &[i64], res: i64) -> bool {
    let ops_cnt = nums.len() - 1;
    let max_ops = 2_i64.pow(ops_cnt as u32);
    for ops in 0..max_ops {
        let mut ops: Vec<Op> = format!("{:b}", ops)
            .to_owned()
            .chars()
            .map(|ch| if ch == '0' { Op::Add } else { Op::Mul })
            .collect();

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
        }
    }

    res
}

#[cfg(test)]
mod tests {
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
        let answer = 3749;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d7p1_answer")
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
