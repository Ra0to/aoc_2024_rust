// Problem: https://adventofcode.com/2024/day/19

use crate::extensions::*;
use crate::point::*;
use std::collections::HashSet;
use std::{collections::HashMap, fs::read_to_string};

#[allow(dead_code)]
pub fn read_input() -> (Vec<Vec<i8>>, P, P) {
    parse_input(&read_to_string("./inputs/d20").unwrap())
}

pub fn parse_input(input: &str) -> (Vec<Vec<i8>>, P, P) {
    let lines = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty());
    let mut start = P::zero();
    let mut end = P::zero();
    let mut map = Vec::new();

    for (y, line) in lines.enumerate() {
        let mut map_line = Vec::new();
        for (x, el) in line.chars().enumerate() {
            let p = P::pair(x as i32, y as i32);

            if el == '#' {
                map_line.push(-1);
                continue;
            }

            if el == 'S' {
                start = p;
            } else if el == 'E' {
                end = p;
            }

            map_line.push(0);
        }

        map.push(map_line);
    }

    (map, start, end)
}

#[allow(dead_code)]
pub fn solve(input: (Vec<Vec<i8>>, P, P)) -> usize {
    calc_cheats(input)
        .into_iter()
        .filter(|(k, _)| *k >= 100)
        .map(|(_, v)| v)
        .sum()
}

pub fn calc_cheats(input: (Vec<Vec<i8>>, P, P)) -> HashMap<usize, usize> {
    let map = input.0;
    let start = input.1;
    let end = input.2;

    let mut cheats = HashMap::new();
    let costs = calc_costs(&map, start);
    let path = retrieve_path(&costs, start, end);
    let path_costs = retrieve_path_costs(&costs, &path);

    for node in path {
        for dir in DIRECTIONS_4 {
            let start_node = node;
            let skip_node = node + dir;
            let end_node = node + 2 * dir;

            if map.get_by_p(end_node).is_none() {
                continue;
            }

            if *map.get_by_p(skip_node).unwrap() >= 0 {
                continue;
            }

            if !path_costs.contains_key(&end_node) {
                continue;
            }

            if path_costs[&end_node] < path_costs[&start_node] {
                continue;
            }

            let cheat_saves = path_costs[&end_node] - path_costs[&start_node] - 2;
            if !cheats.contains_key(&cheat_saves) {
                cheats.insert(cheat_saves, 1);
                continue;
            }

            *cheats.get_mut(&cheat_saves).unwrap() += 1;
        }
    }

    cheats
}

pub fn calc_costs(map: &[Vec<i8>], start: P) -> HashMap<P, (usize, P)> {
    let mut nodes = vec![start];
    let mut completed = HashSet::new();
    let mut costs = HashMap::new();
    costs.insert(start, (0, start));

    while !nodes.is_empty() {
        let node = nodes
            .iter()
            .min_by_key(|x| costs.get(x).map(|pair| pair.0).unwrap_or(usize::MAX))
            .unwrap()
            .clone();
        let index = nodes.iter().position(|x| *x == node).unwrap();
        nodes.remove(index);

        if completed.contains(&node) {
            continue;
        }

        completed.insert(node);
        let cost = costs[&node].0 + 1;

        for dir in DIRECTIONS_4 {
            let pos = node + dir;

            if map.get_by_p(pos).is_none() {
                continue;
            }

            if *map.get_by_p(pos).unwrap() < 0 {
                continue;
            }

            if completed.contains(&pos) {
                continue;
            }

            if !costs.contains_key(&pos) || cost < costs[&pos].0 {
                costs.insert(pos, (cost, node));
            }

            nodes.push(pos);
        }
    }

    costs
}

pub fn retrieve_path(costs: &HashMap<P, (usize, P)>, start: P, end: P) -> Vec<P> {
    let mut path = vec![end];

    let mut el = costs[&end].1;

    while el != start {
        path.push(el);
        el = costs[&el].1;
    }

    path.push(start);

    path.into_iter().rev().collect()
}

pub fn retrieve_path_costs(costs: &HashMap<P, (usize, P)>, path: &[P]) -> HashMap<P, usize> {
    let mut path_costs = HashMap::new();

    for node in path.into_iter() {
        path_costs.insert(*node, costs[&node].0);
    }

    path_costs
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
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
",
        );
        let mut answer = HashMap::new();
        answer.insert(2, 14);
        answer.insert(4, 14);
        answer.insert(6, 2);
        answer.insert(8, 4);
        answer.insert(10, 2);
        answer.insert(12, 3);
        answer.insert(20, 1);
        answer.insert(36, 1);
        answer.insert(38, 1);
        answer.insert(40, 1);
        answer.insert(64, 1);

        // When
        let result = calc_cheats(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_2() {
        // Given
        let input = parse_input(
            "
#####
#...#
#.#.#
#.#.#
#S#E#
#####
",
        );
        let mut answer = HashMap::new();
        answer.insert(2, 1);
        answer.insert(4, 1);
        answer.insert(6, 1);

        // When
        let result = calc_cheats(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d20p1_answer").unwrap();
        let answer = answer.trim().parse::<usize>().unwrap();

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }
}
