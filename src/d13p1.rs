// Problem: https://adventofcode.com/2024/day/13

use crate::point::*;
use std::fs::read_to_string;

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
            let a_p = P::pair(
                a_x[1..].parse::<i32>().unwrap(),
                a_y[1..].parse::<i32>().unwrap(),
            );
            // Button B: X+22, Y+67
            let (b_x, b_y) = &lines[1][10..].split_once(", ").unwrap();
            let b_p = P::pair(
                b_x[1..].parse::<i32>().unwrap(),
                b_y[1..].parse::<i32>().unwrap(),
            );
            // Prize: X=8400, Y=5400
            let (prize_x, prize_y) = &lines[2][7..].split_once(", ").unwrap();
            let prize_p = P::pair(
                prize_x[2..].parse::<i32>().unwrap(),
                prize_y[2..].parse::<i32>().unwrap(),
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
pub fn solve(input: Vec<ClawMachineDefinition>) -> u32 {
    input
        .into_iter()
        .map(|def| try_find_solution(&def))
        .filter(|ans| ans.is_some())
        .map(|ans| ans.unwrap())
        .sum()
}

pub fn try_find_solution(def: &ClawMachineDefinition) -> Option<u32> {
    let mut ans = None;
    for a_cnt in 0..=MOVE_LIMIT {
        for b_cnt in 0..=MOVE_LIMIT {
            let pos = (a_cnt as i32) * def.a_move + (b_cnt as i32) * def.b_move;
            if pos != def.prize {
                continue;
            }

            let price = (a_cnt as u32) * def.a_price + (b_cnt as u32) * def.b_price;

            if ans.is_none() {
                ans = Some(price);
                continue;
            }

            let ans_val = ans.unwrap();
            ans = Some(ans_val.min(price));
        }
    }

    ans
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
            .parse::<u32>()
            .unwrap();

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }
}
