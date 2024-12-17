// Problem: https://adventofcode.com/2024/day/17

use std::fs::read_to_string;

const REG_A: usize = 0;
const REG_B: usize = 1;
const REG_C: usize = 2;

#[allow(dead_code)]
pub fn read_input() -> ([u32; 3], Vec<u8>) {
    parse_input(&read_to_string("./inputs/d17").unwrap())
}

pub fn parse_input(input: &str) -> ([u32; 3], Vec<u8>) {
    let mut regestries = [0_u32; 3];

    let mut lines = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty());

    regestries[REG_A] = lines.next().unwrap()[12..].parse::<u32>().unwrap();
    regestries[REG_B] = lines.next().unwrap()[12..].parse::<u32>().unwrap();
    regestries[REG_C] = lines.next().unwrap()[12..].parse::<u32>().unwrap();

    (
        regestries,
        lines.next().unwrap()[9..]
            .split(',')
            .map(|num| num.parse::<u8>().unwrap())
            .collect(),
    )
}

#[allow(dead_code)]
pub fn solve(input: ([u32; 3], Vec<u8>)) -> String {
    let mut regestries = input.0;
    let program: Vec<_> = input.1.chunks_exact(2).collect();
    let mut ip = 0;
    let mut output = String::new();
    let mut write = |x: u32| {
        if !output.is_empty() {
            output += ",";
        }
        output += &x.to_string();
    };

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
                let denonminator = 2_u32.pow(combo());
                regestries[REG_A] = numerator / denonminator;
            }

            // bxl
            1 => {
                regestries[REG_B] ^= operand as u32;
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
                write(combo() % 8);
            }

            // bdv
            6 => {
                let numerator = regestries[REG_A];
                let denonminator = 2_u32.pow(combo());
                regestries[REG_B] = numerator / denonminator;
            }

            // cdv
            7 => {
                let numerator = regestries[REG_A];
                let denonminator = 2_u32.pow(combo());
                regestries[REG_C] = numerator / denonminator;
            }

            inst => panic!("unsupported instruction {inst}"),
        }

        if ip == prev_ip {
            ip += 1;
        }
    }

    output
}

pub fn combo(regestries: &[u32; 3], operand: u8) -> u32 {
    match operand {
        op if op <= 3 => op as u32,
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
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
",
        );
        let answer = "4,6,3,5,6,3,5,2,1,0";

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d17p1_answer").unwrap();
        let answer = answer.trim();

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }
}
