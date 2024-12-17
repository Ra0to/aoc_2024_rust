// Problem: https://adventofcode.com/2024/day/13

use std::fs::read_to_string;

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct P(i128, i128);

#[allow(dead_code)]
pub fn read_input() -> Vec<ClawMachineDefinition> {
    parse_input(&read_to_string("./inputs/d13").unwrap())
}

pub fn parse_input(input: &str) -> Vec<ClawMachineDefinition> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>()
        .chunks_exact(3)
        .map(|lines| {
            // Button A: X+94, Y+34
            let (a_x, a_y) = &lines[0][10..].split_once(", ").unwrap();
            let a_p = P(
                a_x[1..].parse::<i128>().unwrap(),
                a_y[1..].parse::<i128>().unwrap(),
            );
            // Button B: X+22, Y+67
            let (b_x, b_y) = &lines[1][10..].split_once(", ").unwrap();
            let b_p = P(
                b_x[1..].parse::<i128>().unwrap(),
                b_y[1..].parse::<i128>().unwrap(),
            );
            // Prize: X=8400, Y=5400
            let (prize_x, prize_y) = &lines[2][7..].split_once(", ").unwrap();
            let prize_p = P(
                prize_x[2..].parse::<i128>().unwrap(),
                prize_y[2..].parse::<i128>().unwrap(),
            );

            ClawMachineDefinition::with_default_price(a_p, b_p, prize_p)
        })
        .collect()
}

pub const MOVE_LIMIT: usize = 100;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClawMachineDefinition {
    pub a_move: P,
    pub a_price: u32,
    pub b_move: P,
    pub b_price: u32,
    pub prize: P,
}

impl ClawMachineDefinition {
    pub const fn with_default_price(a_move: P, b_move: P, prize: P) -> Self {
        Self {
            a_move,
            a_price: 3,
            b_move,
            b_price: 1,
            prize,
        }
    }
}

#[allow(dead_code)]
pub fn solve(input: Vec<ClawMachineDefinition>) -> u128 {
    input
        .into_iter()
        .filter_map(|def| try_find_solution(&def, Some(MOVE_LIMIT), None))
        .sum()
}

pub fn try_find_solution(
    def: &ClawMachineDefinition,
    limit: Option<usize>,
    addition: Option<i128>,
) -> Option<u128> {
    let addition = addition.unwrap_or(0);
    let p = def.prize;
    let p_a = def.a_move;
    let p_b = def.b_move;
    let p_x = p.0 + addition;
    let p_y = p.1 + addition;
    let x_a = p_a.0;
    let x_b = p_b.0;
    let y_a = p_a.1;
    let y_b = p_b.1;

    let a_nominator = p_y * x_b - y_b * p_x;
    let a_denominator = y_a * x_b - x_a * y_b;

    if a_nominator % a_denominator != 0 {
        return None;
    }

    let a = a_nominator / a_denominator;

    if a < 0 || limit.is_some_and(|l| a as usize > l) {
        return None;
    }

    let b_nominator = p_x - a * x_a;
    let b_denmominator = x_b;

    if b_nominator % b_denmominator != 0 {
        return None;
    }

    let b = b_nominator / b_denmominator;

    if b < 0 || limit.is_some_and(|l| b as usize > l) {
        return None;
    }

    let a = a as u128;
    let b = b as u128;

    Some(a * (def.a_price as u128) + b * (def.b_price as u128))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Given
        let input = parse_input(
            "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
        );
        let answer = 480;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d13p1_answer")
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
