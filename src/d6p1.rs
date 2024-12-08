// Problem: https://adventofcode.com/2024/day/6

use crate::extensions::*;
use crate::point::P;
use std::fs::read_to_string;

#[allow(dead_code)]
pub fn read_input() -> (Vec<Vec<i32>>, P) {
    parse_input_string(read_to_string("./inputs/d6").unwrap())
}

pub fn parse_input_string(input: String) -> (Vec<Vec<i32>>, P) {
    let mut map = vec![];
    let mut pos = P::zero();
    for (y, line) in input.lines().enumerate() {
        let line = line.trim();
        let mut map_line = vec![];

        for (x, ch) in line.chars().enumerate() {
            match ch {
                '^' => {
                    pos = P::pair(x as i32, y as i32);
                    map_line.push(0);
                }
                '.' => map_line.push(0),
                _ => map_line.push(-1),
            }
        }

        map.push(map_line);
    }

    (map, pos)
}

#[allow(dead_code)]
pub fn solve(input: (Vec<Vec<i32>>, P)) -> usize {
    let mut map = input.0;
    let pos = input.1;

    if let Some(e) = map.get_mut_by_p(pos) {
        *e += 1;
    }
    // Our map has inverted numbers by Y
    move_guard(&mut map, pos, P::down());

    map.iter()
        .map(|line| line.iter().filter(|el| **el > 0).count())
        .sum()
}

pub fn move_guard(map: &mut [Vec<i32>], pos: P, dir: P) {
    let new_pos = pos + dir;
    match map.get_by_p(new_pos) {
        None => (),
        Some(&-1) => {
            let dir = get_next_dir(dir);
            move_guard(map, pos, dir);
        }
        Some(_) => {
            if let Some(e) = map.get_mut_by_p(new_pos) {
                *e += 1;
            }
            move_guard(map, new_pos, dir);
        }
    }
}

pub fn get_next_dir(dir: P) -> P {
    if dir == P::down() {
        P::right()
    } else if dir == P::right() {
        P::up()
    } else if dir == P::up() {
        P::left()
    } else {
        P::down()
    }
}

#[cfg(test)]
mod tests {
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
        let answer = 41;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d6p1_answer")
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
