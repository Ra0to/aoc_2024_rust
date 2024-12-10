// Problem: https://adventofcode.com/2024/day/10

use crate::d10p1;
use crate::extensions::*;
use crate::point::P;

#[allow(dead_code)]
pub fn read_input() -> Vec<Vec<u8>> {
    d10p1::read_input()
}

#[allow(dead_code)]
pub fn solve(map: Vec<Vec<u8>>) -> u32 {
    map.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, el)| **el == 0)
                .map(|(x, _)| find_reachable_peaks(&map, P::pair(x as i32, y as i32), None))
                .sum::<u32>()
        })
        .sum()
}

fn find_reachable_peaks(map: &[Vec<u8>], pos: P, prev_pos: Option<P>) -> u32 {
    let cur = map.get_by_p(pos);
    match cur {
        Some(height) if *height == 9 => 1,
        Some(height) => [P::down(), P::right(), P::up(), P::left()]
            .into_iter()
            .map(|dir| pos + dir)
            .filter(|new_pos| prev_pos.is_none() || prev_pos.is_some_and(|x| x != *new_pos))
            .filter(|new_pos| map.is_at_p(*new_pos, &(*height + 1)))
            .map(|new_pos| find_reachable_peaks(map, new_pos, Some(pos)))
            .sum(),
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::d10p1::parse_input;
    use std::fs::read_to_string;

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
        let answer = 81;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d10p2_answer")
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
