// Problem: https://adventofcode.com/2024/day/15

use crate::d15p1::{find_free_spot, move_blocks, Move};
use crate::extensions::*;
use crate::point::P;
use std::fs::read_to_string;

#[allow(dead_code)]
pub fn read_input() -> (Vec<Vec<i8>>, P, Vec<Move>) {
    parse_input(&read_to_string("./inputs/d15").unwrap())
}

pub fn parse_input(input: &str) -> (Vec<Vec<i8>>, P, Vec<Move>) {
    let mut map = Vec::new();
    let mut moves = Vec::new();
    let mut pos = P::zero();
    let lines = input.lines();
    let mut is_map = true;
    for (y, line) in lines.enumerate() {
        let line = line.trim();
        if line.is_empty() {
            is_map = false;
            continue;
        }

        if is_map {
            map.push(
                line.chars()
                    .map(|ch| {
                        match ch {
                            '@' => "@.",
                            '#' => "##",
                            'O' => "[]",
                            '.' => "..",
                            ch => panic!("unexpected map char {}", ch),
                        }
                        .chars()
                    })
                    .flatten()
                    .enumerate()
                    .map(|(x, ch)| {
                        if ch == '@' {
                            pos = P::pair(x as i32, y as i32);
                        }

                        match ch {
                            '#' => -1,
                            '[' => 1,
                            ']' => 2,
                            _ => 0,
                        }
                    })
                    .collect::<Vec<i8>>(),
            );
        } else {
            moves.append(
                &mut line
                    .chars()
                    .map(|ch| Move::parse(ch))
                    .collect::<Vec<Move>>(),
            );
        }
    }

    (map, pos, moves)
}

#[allow(dead_code)]
pub fn solve(input: (Vec<Vec<i8>>, P, Vec<Move>)) -> u32 {
    let mut map = input.0;
    let start = input.1;
    let moves = input.2;

    let mut pos = start;
    for mv in moves {
        pos = process_move(&mut map, pos, mv.invert_by_y());
    }

    calc_coords(&map)
}

pub fn process_move(map: &mut [Vec<i8>], pos: P, mv: Move) -> P {
    if mv == Move::Left || mv == Move::Right {
        let free_spot = find_free_spot(map, pos, mv);
        if free_spot.is_none() {
            return pos;
        }

        let free_spot = free_spot.unwrap();
        move_blocks(map, free_spot, pos, mv.opposite().to_p());

        return pos + mv.to_p();
    } else {
        if can_move_blocks(map, pos, mv) {
            move_blocks_vert(map, pos, mv);
            pos + mv.to_p()
        } else {
            pos
        }
    }
}

pub fn can_move_blocks(map: &mut [Vec<i8>], pos: P, mv: Move) -> bool {
    let next_pos = pos + mv.to_p();
    match map.get_by_p(next_pos).unwrap() {
        -1 => false,
        0 => true,
        1 => can_move_blocks(map, next_pos, mv) && can_move_blocks(map, next_pos + P::right(), mv),
        2 => can_move_blocks(map, next_pos, mv) && can_move_blocks(map, next_pos + P::left(), mv),
        block => panic!("unexpected map block {}", block),
    }
}

pub fn move_blocks_vert(map: &mut [Vec<i8>], pos: P, mv: Move) {
    let next_pos = pos + mv.to_p();
    match map.get_by_p(next_pos).unwrap() {
        1 => {
            move_blocks_vert(map, next_pos, mv);
            move_blocks_vert(map, next_pos + P::right(), mv);
        },
        2 => {
            move_blocks_vert(map, next_pos, mv);
            move_blocks_vert(map, next_pos + P::left(), mv);
        },
        _ => (),
    };

    map.swap_by_p(pos, next_pos);
}

pub fn calc_coords(map: &[Vec<i8>]) -> u32 {
    let mut sum = 0;
    for (y, line) in map.iter().enumerate() {
        for (x, el) in line.iter().enumerate() {
            if *el != 1 {
                continue;
            }

            sum += x as u32 + (y as u32) * 100;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Given
        let input = parse_input(
            "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
        );
        let answer = 9021;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d15p2_answer")
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
