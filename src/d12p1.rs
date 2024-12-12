// Problem: https://adventofcode.com/2024/day/12

use crate::extensions::*;
use crate::point::*;
use std::fs::read_to_string;

#[allow(dead_code)]
pub fn read_input() -> Vec<Vec<char>> {
    parse_input(&read_to_string("./inputs/d12").unwrap())
}

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().chars().collect())
        .collect()
}

#[allow(dead_code)]
pub fn solve(input: Vec<Vec<char>>) -> u32 {
    let mut total_price = 0;
    let mut visited = vec![vec![false; input.width()]; input.height()];

    for (y, line) in input.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            let p = P::pair(x as i32, y as i32);
            if *visited.get_by_p(p).unwrap() {
                continue;
            }

            let (perimeter, area) = process_field(&input, &mut visited, p);
            total_price += perimeter * area;
        }
    }

    total_price
}

fn process_field(input: &[Vec<char>], visited: &mut [Vec<bool>], p: P) -> (u32, u32) {
    *visited.get_mut_by_p(p).unwrap() = true;
    let current = *input.get_by_p(p).unwrap();
    let mut perimeter = DIRECTIONS_4
        .iter()
        .map(|dir| input.get_by_p(*dir + p))
        .filter(|neighbor| neighbor.is_none() || neighbor.is_some_and(|val| *val != current))
        .count() as u32;
    let mut area = 1;

    for dir in DIRECTIONS_4.iter() {
        let pos = p + *dir;
        if !input.get_by_p(pos).is_some_and(|val| *val == current) {
            continue;
        }
        if *visited.get_by_p(pos).unwrap() {
            continue;
        }
        let (neighbor_per, neighbor_area) = process_field(input, visited, pos);
        perimeter += neighbor_per;
        area += neighbor_area;
    }

    (perimeter, area)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Given
        let input = parse_input(
            "
AAAA
BBCD
BBCC
EEEC
",
        );
        let answer = 140;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_2() {
        // Given
        let input = parse_input(
            "
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
",
        );
        let answer = 772;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_3() {
        // Given
        let input = parse_input(
            "
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
",
        );
        let answer = 1930;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d12p1_answer")
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
