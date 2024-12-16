// Problem: https://adventofcode.com/2024/day/16

use crate::extensions::*;
use crate::point::P;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

#[allow(dead_code)]
pub fn read_input() -> (Vec<Vec<i8>>, P, P) {
    parse_input(&read_to_string("./inputs/d16").unwrap())
}

pub fn parse_input(input: &str) -> (Vec<Vec<i8>>, P, P) {
    let mut start = P::zero();
    let mut end = P::zero();

    let map = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, el)| {
                    let p = P::pair(x as i32, y as i32);
                    if el == 'S' {
                        start = p;
                    }
                    if el == 'E' {
                        end = p;
                    }

                    match el {
                        '#' => -1,
                        _ => 0,
                    }
                })
                .collect::<Vec<i8>>()
        })
        .collect();

    (map, start, end)
}

#[allow(dead_code)]
pub fn solve(input: (Vec<Vec<i8>>, P, P)) -> u32 {
    let map = input.0;
    let start = input.1;
    let end = input.2;

    let mut nodes = vec![(start, P::right())];
    let mut comleted = HashSet::new();
    let mut costs = HashMap::new();
    costs.insert(start, 0);

    while !nodes.is_empty() {
        let (node, dir) = nodes
            .iter()
            .min_by_key(|x| costs.get(&x.0).unwrap_or(&u32::MAX))
            .unwrap();
        let dir = dir.clone();
        let node = node.clone();
        let index = nodes.iter().position(|x| x.0 == node).unwrap();
        nodes.swap_remove(index);
        if comleted.contains(&node) {
            continue;
        }

        comleted.insert(node);
        let cost = costs[&node];

        let neighbors = [
            (node + dir, cost + 1, dir),
            (node + clockwise(dir), cost + 1001, clockwise(dir)),
            (
                node + counterclockwise(dir),
                cost + 1001,
                counterclockwise(dir),
            ),
        ];
        for (pos, cost, dir) in neighbors {
            if map.get_by_p(pos).is_none() {
                continue;
            }

            if map.get_by_p(pos).is_some_and(|el| *el < 0) {
                continue;
            }

            if comleted.contains(&pos) {
                continue;
            }

            if !costs.contains_key(&pos) || cost < costs[&pos] {
                costs.insert(pos, cost);
            }

            nodes.push((pos, dir));
        }
    }

    costs[&end]
}

fn clockwise(dir: P) -> P {
    if dir == P::up() {
        P::right()
    } else if dir == P::right() {
        P::down()
    } else if dir == P::down() {
        P::left()
    } else {
        P::up()
    }
}

fn counterclockwise(dir: P) -> P {
    if dir == P::up() {
        P::left()
    } else if dir == P::left() {
        P::down()
    } else if dir == P::down() {
        P::right()
    } else {
        P::up()
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
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
",
        );
        let answer = 7036;

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
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#^#
#.#.#.#...#...#^#
#.#.#.#.###.#.#^#
#>>v#.#.#.....#^#
#^#v#.#.#.#####^#
#^#v..#.#.#>>>>^#
#^#v#####.#^###.#
#^#v#..>>>>^#...#
#^#v###^#####.###
#^#v#>>^#.....#.#
#^#v#^#####.###.#
#^#v#^........#.#
#^#v#^#########.#
#S#>>^..........#
#################
",
        );
        let answer = 11048;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d16p1_answer")
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
