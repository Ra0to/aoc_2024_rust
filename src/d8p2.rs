// Problem: https://adventofcode.com/2024/day/8

use std::collections::{HashMap, HashSet};

use crate::d8p1;
use crate::{extensions::TableExtensions, point::P};

#[allow(dead_code)]
pub fn read_input() -> Vec<Vec<char>> {
    d8p1::read_input()
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

            antinodes.append(&mut get_antinodes_for(left, right, map));
        }
    }

    antinodes
}

pub fn get_antinodes_for(left: P, right: P, map: &[Vec<char>]) -> Vec<P> {
    // Prevent float arihmethics
    let left = left * 10;
    let right = right * 10;

    let middle = (left + right) / 2;
    let radius = right - middle;
    let diameter = radius * 2;

    let mut res = Vec::new();

    let mut start = middle + radius;

    for i in 0.. {
        let point = (start + (diameter * i)) / 10;
        match map.get_by_p(point) {
            Some(_) => res.push(point),
            None => break,
        }
    }

    start = middle - radius;

    for i in 0.. {
        let point = (start - (diameter * i)) / 10;
        match map.get_by_p(point) {
            Some(_) => res.push(point),
            None => break,
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    fn format_input(raw_input: &str) -> Vec<Vec<char>> {
        d8p1::pasre_input(
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
        let answer = 34;

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
T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........
",
        );
        let answer = 9;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d8p2_answer")
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
