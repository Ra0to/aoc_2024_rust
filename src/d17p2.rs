// Problem: https://adventofcode.com/2024/day/17

use std::fs::read_to_string;

const REG_A: usize = 0;
const REG_B: usize = 1;
const REG_C: usize = 2;

#[allow(dead_code)]
pub fn read_input() -> ([u128; 3], Vec<u8>) {
    parse_input(&read_to_string("./inputs/d17").unwrap())
}

pub fn parse_input(input: &str) -> ([u128; 3], Vec<u8>) {
    let mut regestries = [0_u128; 3];

    let mut lines = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty());

    regestries[REG_A] = lines.next().unwrap()[12..].parse::<u128>().unwrap();
    regestries[REG_B] = lines.next().unwrap()[12..].parse::<u128>().unwrap();
    regestries[REG_C] = lines.next().unwrap()[12..].parse::<u128>().unwrap();

    (
        regestries,
        lines.next().unwrap()[9..]
            .split(',')
            .map(|num| num.parse::<u8>().unwrap())
            .collect(),
    )
}

#[allow(dead_code)]
pub fn solve(input: ([u128; 3], Vec<u8>)) -> u128 {
    let registries = input.0;
    let program = input.1;
    let n = program.len();

    let min = 8_u128.pow((n - 1) as u32);
    find_rec(&registries, &program, 0, min)
}

pub fn find_rec(regestries: &[u128; 3], program: &Vec<u8>, num: usize, min: u128) -> u128 {
    let n = program.len();
    if num >= n {
        let mut reg = regestries.clone();
        reg[REG_A] = min;
        let ans = chech(reg, &program);
        if &ans == program {
            return min;
        }
        return 0;
    }

    let step = 8_u128.pow((n - num - 1) as u32);
    let max = min + 8_u128.pow((n - num) as u32);
    let mut i = min;

    while i <= max {
        let mut reg = regestries.clone();
        reg[REG_A] = i;
        let ans = chech(reg, &program);
        if ans.len() == n && ans[n - num - 1] == program[n - num - 1] {
            let dd = find_rec(regestries, program, num + 1, i);
            if dd > 0 {
                return dd;
            }
        }
        i += step;
    }

    0
}

pub fn chech(mut regestries: [u128; 3], raw_program: &Vec<u8>) -> Vec<u8> {
    let program: Vec<_> = raw_program.chunks_exact(2).collect();
    let mut ip = 0;
    let mut ans = Vec::new();

    while ip < program.len() {
        let data = program[ip];
        let cmd = data[0];
        let operand = data[1];
        let combo = || combo(&regestries, operand);

        let prev_ip = ip;

        match cmd {
            // adv
            0 => {
                let numerator = regestries[REG_A];
                let denonminator = 2_u128.pow(combo() as u32);
                regestries[REG_A] = numerator / denonminator;
            }

            // bxl
            1 => {
                regestries[REG_B] ^= operand as u128;
            }

            // bst
            2 => {
                regestries[REG_B] = combo() % 8;
            }

            // jnz
            3 => {
                if regestries[REG_A] != 0 {
                    ip = operand as usize;
                }
            }

            // bxc
            4 => {
                regestries[REG_B] ^= regestries[REG_C];
            }

            // out
            5 => {
                ans.push((combo() % 8) as u8);
            }

            // bdv
            6 => {
                let numerator = regestries[REG_A];
                let denonminator = 2_u128.pow(combo() as u32);
                regestries[REG_B] = numerator / denonminator;
            }

            // cdv
            7 => {
                let numerator = regestries[REG_A];
                let denonminator = 2_u128.pow(combo() as u32);
                regestries[REG_C] = numerator / denonminator;
            }

            inst => panic!("unsupported instruction {inst}"),
        }

        if ip == prev_ip {
            ip += 1;
        }
    }

    ans
}

pub fn combo(regestries: &[u128; 3], operand: u8) -> u128 {
    match operand {
        op if op <= 3 => op as u128,
        4 => regestries[REG_A],
        5 => regestries[REG_B],
        6 => regestries[REG_C],
        op => panic!("invalid combo operand {op}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Given
        let input = parse_input(
            "
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
",
        );
        let answer = 117440;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d17p2_answer")
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
