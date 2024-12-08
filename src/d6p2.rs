// Problem: https://adventofcode.com/2024/day/6

use crate::d6p1;
use crate::extensions::*;
use crate::point::P;

#[allow(dead_code)]
pub fn read_input() -> (Vec<Vec<i32>>, P) {
    d6p1::read_input()
}

#[allow(dead_code)]
pub fn solve(input: (Vec<Vec<i32>>, P)) -> usize {
    let mut new_obstacles_count = 0;
    let map = input.0;
    let pos = input.1;

    for y in 0..map.len() {
        let line_len = map[y].len();
        for x in 0..line_len {
            let p = P::pair(x as i32, y as i32);
            if p == pos {
                continue;
            }

            match map.get_by_p(p) {
                Some(e) => match *e {
                    -1 => continue,
                    _ => {
                        let mut new_map = map.clone();
                        new_map[y][x] = -1;
                        if is_loop(&mut new_map, pos) {
                            new_obstacles_count += 1;
                        }
                    }
                },
                None => panic!("We are out of map bounds"),
            }
        }
    }

    new_obstacles_count
}

pub fn is_loop(map: &mut [Vec<i32>], pos: P) -> bool {
    if let Some(e) = map.get_mut_by_p(pos) {
        *e += 1;
    }
    // Our map has inverted numbers by Y
    move_guard(map, pos, P::down())
}

pub fn move_guard(map: &mut [Vec<i32>], pos: P, dir: P) -> bool {
    let new_pos = pos + dir;
    match map.get_by_p(new_pos) {
        None => false,
        Some(&-1) => {
            let dir = get_next_dir(dir);
            move_guard(map, pos, dir)
        }
        Some(_) => {
            if let Some(e) = map.get_mut_by_p(new_pos) {
                *e += 1;
            }
            match map.get_mut_by_p(new_pos) {
                // I have no idea why 5 but this works
                Some(e) if *e >= 5 => return true,
                _ => (),
            }
            move_guard(map, new_pos, dir)
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
