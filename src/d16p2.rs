// Problem: https://adventofcode.com/2024/day/16

use crate::d16p1;
use crate::d16p1::{clockwise, counterclockwise};
use crate::extensions::*;
use crate::point::*;
use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
pub fn read_input() -> (Vec<Vec<i8>>, P, P) {
    d16p1::read_input()
}

#[allow(dead_code)]
pub fn solve(input: (Vec<Vec<i8>>, P, P)) -> u32 {
    let map = input.0;
    let start = input.1;
    let end = input.2;

    let mut nodes = vec![(start, P::right())];
    let mut comleted = HashSet::new();
    let mut costs = HashMap::new();
    let mut prevs = HashMap::new();
    costs.insert((start, P::right()), 0);

    while !nodes.is_empty() {
        let (node, dir) = nodes
            .iter()
            .min_by_key(|x| costs.get(&x).unwrap_or(&u32::MAX))
            .unwrap();
        let dir = dir.clone();
        let node = node.clone();
        let pair = (node, dir);
        let index = nodes.iter().position(|x| *x == pair).unwrap();
        nodes.swap_remove(index);
        if comleted.contains(&pair) {
            continue;
        }

        comleted.insert(pair);
        let cost = costs[&pair];

        let neighbors = [
            (node + dir, cost + 1, dir),
            (node, cost + 1000, clockwise(dir)),
            (node, cost + 1000, counterclockwise(dir)),
        ];
        let par_dir = dir;
        for (pos, cost, dir) in neighbors {
            let pair = (pos, dir);
            if map.get_by_p(pos).is_none() {
                continue;
            }

            if map.get_by_p(pos).is_some_and(|el| *el < 0) {
                continue;
            }

            if comleted.contains(&pair) {
                continue;
            }

            if !costs.contains_key(&pair) || cost < costs[&pair] {
                costs.insert(pair, cost);
                prevs.insert(pair, vec![(node, par_dir)]);
            } else if costs.contains_key(&pair) && cost == costs[&pair] {
                prevs.get_mut(&pair).unwrap().push((node, par_dir));
            }

            nodes.push(pair);
        }
    }

    let mut on_path = HashSet::new();

    let mut min = u32::MAX;
    for dir in DIRECTIONS_4.into_iter() {
        let pair = (end, dir);
        if !costs.contains_key(&pair) {
            continue;
        }

        if costs[&pair] < min {
            min = costs[&pair];
        }
    }

    for dir in DIRECTIONS_4.into_iter() {
        let pair = (end, dir);
        if !prevs.contains_key(&pair) {
            continue;
        }

        if costs[&pair] > min {
            continue;
        }

        get_visited(&prevs, pair, &mut on_path);
    }

    on_path.len() as u32
}

fn get_visited(prevs: &HashMap<(P, P), Vec<(P, P)>>, key: (P, P), on_path: &mut HashSet<P>) {
    if !prevs.contains_key(&key) {
        return;
    }

    on_path.insert(key.0);
    for k in prevs[&key].iter() {
        get_visited(prevs, *k, on_path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::d16p1::parse_input;
    use std::fs::read_to_string;

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
        let answer = 45;

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
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
",
        );
        let answer = 64;

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
#####
###E#
#...#
#.#.#
#...#
#S###
#####
",
        );
        let answer = 10;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d16p2_answer")
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
