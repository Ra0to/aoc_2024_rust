// Problem: https://adventofcode.com/2024/day/8

use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

use crate::{extensions::TableExtensions, point::P};

#[allow(dead_code)]
pub fn read_input() -> Vec<Vec<char>> {
    pasre_input(&read_to_string("./inputs/d8").unwrap())
}

pub fn pasre_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[allow(dead_code)]
pub fn solve(input: Vec<Vec<char>>) -> usize {
    let mut antennas: HashMap<char, Vec<P>> = HashMap::new();

    for y in 0..input.len() {
        let width = input[y].len();
        for x in 0..width {
            let p = P::pair(x as i32, y as i32);
            match input
                .get_by_p(p)
                .expect("map point should be in the valid range")
            {
                '.' => (),
                freq => {
                    antennas
                        .entry(*freq)
                        .and_modify(|vec| vec.push(p))
                        .or_insert(vec![p]);
                }
            }
        }
    }

    let mut antinodes: HashSet<P> = HashSet::new();

    antennas.into_iter().for_each(|(_, vec)| {
        antinodes.extend(get_all_antinodes_for_freq(&vec, &input));
    });

    antinodes.len()
}

pub fn get_all_antinodes_for_freq(antennas: &[P], map: &[Vec<char>]) -> Vec<P> {
    let mut antinodes = Vec::new();
    for l_index in 0..antennas.len() {
        for r_index in (l_index + 1)..antennas.len() {
            let left = antennas[l_index];
            let right = antennas[r_index];

            antinodes.extend(
                get_raw_antinodes(left, right)
                    .into_iter()
                    .filter(|node| map.get_by_p(*node).is_some()),
            );
        }
    }

    antinodes
}

pub fn get_raw_antinodes(left: P, right: P) -> [P; 2] {
    // Prevent float arihmethics
    let left = left * 10;
    let right = right * 10;

    let middle = (left + right) / 2;
    let radius = right - middle;

    let triple_radius = radius * 3;

    let first_antinode = (middle + triple_radius) / 10;
    let second_antinode = (middle - triple_radius) / 10;

    [first_antinode, second_antinode]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn format_input(raw_input: &str) -> Vec<Vec<char>> {
        pasre_input(
            &raw_input
                .lines()
                .filter(|line| !line.trim().is_empty())
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }

    #[test]
    fn test_1() {
        // Given
        let input = format_input(
            "
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
",
        );
        let answer = 14;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_2() {
        // Given
        let input = format_input(
            "
..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........

",
        );
        let answer = 2;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_3() {
        // Given
        let input = format_input(
            "
..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
..........
",
        );
        let answer = 4;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_4() {
        // Given
        let input = format_input(
            "
..........
..........
..........
....a.....
........a.
.....a....
..........
......A...
..........
..........
",
        );
        let answer = 4;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d8p1_answer")
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }
}
