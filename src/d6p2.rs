// Problem: https://adventofcode.com/2024/day/6

use crate::d6p1;
use crate::d6p1::get_next_dir;
use crate::extensions::*;
use crate::point::P;
use rayon::prelude::*;

#[allow(dead_code)]
pub fn read_input() -> (Vec<Vec<i32>>, P) {
    d6p1::read_input()
}

#[allow(dead_code)]
pub fn solve(input: (Vec<Vec<i32>>, P)) -> usize {
    let (map, start_pos) = input;

    let mut visited_map = map.clone();
    d6p1::move_guard(&mut visited_map, start_pos, P::down());

    map.par_iter()
        .enumerate()
        .map(|(y, line)| {
            line.par_iter()
                .enumerate()
                .map(|(x, el)| {
                    let p = P::pair(x as i32, y as i32);

                    if p == start_pos {
                        return 0;
                    }

                    if *el == -1 {
                        return 0;
                    }

                    // New block outside of guard path can't change anything
                    if visited_map[y][x] <= 0 {
                        return 0;
                    }

                    let mut new_map = map.clone();
                    new_map[y][x] = -1;
                    if is_loop(&mut new_map, start_pos) {
                        1
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

pub fn is_loop(map: &mut [Vec<i32>], mut pos: P) -> bool {
    *map.get_mut_by_p(pos)
        .expect("guard initial pos should be inside map") += 1;
    let mut dir = P::down();
    while let Some(cell) = map.get_mut_by_p(pos + dir) {
        if *cell == -1 {
            dir = get_next_dir(dir);
            continue;
        }

        *cell += 1;

        if *cell >= 5 {
            return true;
        }

        pos = pos + dir;
    }

    false
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use d6p1::parse_input_string;

    use super::*;

    #[test]
    fn test_1() {
        // Given
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            .to_string();
        let input = parse_input_string(input);
        let answer = 6;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d6p2_answer")
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
