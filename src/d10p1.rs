// Problem: https://adventofcode.com/2024/day/10

use std::collections::HashSet;
use std::fs::read_to_string;

use crate::extensions::*;
use crate::point::P;

#[allow(dead_code)]
pub fn read_input() -> Vec<Vec<u8>> {
    parse_input(&read_to_string("./inputs/d10").unwrap())
}

pub fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.trim()
                .chars()
                .map(|ch| ch.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

#[allow(dead_code)]
pub fn solve(map: Vec<Vec<u8>>) -> u32 {
    map.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, el)| **el == 0)
                .map(|(x, _)| {
                    let mut reached_peaks = HashSet::new();
                    find_reachable_peaks(&map, P::pair(x as i32, y as i32), &mut reached_peaks);
                    reached_peaks.len() as u32
                })
                .sum::<u32>()
        })
        .sum()
}

fn find_reachable_peaks(map: &[Vec<u8>], pos: P, reached_peaks: &mut HashSet<P>) {
    let cur = map.get_by_p(pos);
    match cur {
        Some(height) if *height == 9 => {
            reached_peaks.insert(pos);
        },
        Some(height) => {
            [P::down(), P::right(), P::up(), P::left()]
                    .into_iter()
                    .map(|dir| pos + dir)
                    .filter(|new_pos| map.is_at_p(*new_pos, &(*height + 1)))
                    .for_each(|new_pos| find_reachable_peaks(map, new_pos, reached_peaks));
        },
        None => (),
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
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
",
        );
        let answer = 36;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d10p1_answer")
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
