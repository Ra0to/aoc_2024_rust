// Problem: https://adventofcode.com/2024/day/18

use crate::extensions::*;
use crate::point::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

#[allow(dead_code)]
pub fn read_input() -> Vec<P> {
    parse_input(&read_to_string("./inputs/d18").unwrap())
}

pub fn parse_input(input: &str) -> Vec<P> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            P::pair(x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        })
        .collect()
}

#[allow(dead_code)]
pub fn solve(input: Vec<P>) -> usize {
    solve_for_map(input, 71, 1024)
}

#[allow(dead_code)]
pub fn solve_for_map(input: Vec<P>, map_size: usize, time: usize) -> usize {
    let n = map_size;
    let start = P::zero();
    let end = P::pair((n - 1) as i32, (n - 1) as i32);
    let mut map: Vec<Vec<i8>> = vec![vec![0; n]; n];

    for i in 0..time {
        let byte = input[i];
        *map.get_mut_by_p(byte).unwrap() = -1;
    }

    find_shortest_path(&map, start, end).expect("we should have shortest path")
}

pub fn find_shortest_path(map: &[Vec<i8>], start: P, end: P) -> Option<usize> {
    let mut costs = HashMap::new();
    let mut completed = HashSet::new();
    costs.insert(start, 0);
    let mut nodes = vec![start];

    while !nodes.is_empty() {
        let node = *nodes
            .iter()
            .min_by_key(|p| costs.get(p).unwrap_or(&usize::MAX))
            .unwrap();
        let ind = nodes.iter().position(|x| *x == node).unwrap();
        nodes.remove(ind);
        if completed.contains(&node) {
            continue;
        }

        completed.insert(node);
        let cost = costs[&node] + 1;

        for dir in DIRECTIONS_4 {
            let new_p = node + dir;
            if map.get_by_p(new_p).is_none() {
                continue;
            }

            if *map.get_by_p(new_p).unwrap() < 0 {
                continue;
            }

            if completed.contains(&new_p) {
                continue;
            }

            if !costs.contains_key(&new_p) || cost < costs[&new_p] {
                costs.insert(new_p, cost);
            }

            nodes.push(new_p);
        } 
    }

    costs.get(&end).map(|x| *x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Given
        let input = parse_input(
            "
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
",
        );
        let answer = 22;

        // When
        let result = solve_for_map(input, 7, 12);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d18p1_answer").unwrap();
        let answer = answer.trim().parse::<usize>().unwrap();

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }
}
