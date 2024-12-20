// Problem: https://adventofcode.com/2024/day/20

use crate::d20p1;
use crate::point::*;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn read_input() -> (Vec<Vec<i8>>, P, P) {
    d20p1::read_input()
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
    d20p1::calc_cheats_with_skips(input, get_skips_for)
}

pub fn get_skips_for(pos: P) -> Vec<P> {
    let mut skips = vec![];
    let from = pos + 20 * P::down_left();
    let to = pos + 20 * P::up_right();

    for x in from.x..=to.x {
        for y in from.y..=to.y {
            let len = (pos.x - x).abs() + (pos.y - y).abs();
            if len > 20 {
                continue;
            }

            let end_node = P::pair(x, y);
            skips.push(end_node);
        }
    }

    skips
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;
    use crate::d20p1::parse_input;

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
        answer.insert(50, 32);
        answer.insert(52, 31);
        answer.insert(54, 29);
        answer.insert(56, 39);
        answer.insert(58, 25);
        answer.insert(60, 23);
        answer.insert(62, 20);
        answer.insert(64, 19);
        answer.insert(66, 12);
        answer.insert(68, 14);
        answer.insert(70, 12);
        answer.insert(72, 22);
        answer.insert(74, 4);
        answer.insert(76, 3);

        // When
        let mut result = calc_cheats(input);

        let res_clone = result.clone();
        let keys_to_remove = res_clone
            .iter()
            .filter(|kvp| *kvp.0 < 50)
            .map(|kvp| kvp.0)
            .collect::<Vec<_>>();

        for k in keys_to_remove {
            result.remove(&k);
        }
        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d20p2_answer").unwrap();
        let answer = answer.trim().parse::<usize>().unwrap();

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }
}
