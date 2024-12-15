// Problem: https://adventofcode.com/2024/day/15

use crate::extensions::*;
use crate::point::P;
use std::fs::read_to_string;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Up,
    Right,
    Down,
    Left,
}

impl Move {
    pub fn parse(ch: char) -> Move {
        match ch {
            '^' => Move::Up,
            '<' => Move::Left,
            '>' => Move::Right,
            'v' => Move::Down,
            _ => panic!("unknown move {}", ch),
        }
    }

    pub fn to_p(self) -> P {
        match self {
            Move::Up => P::up(),
            Move::Right => P::right(),
            Move::Down => P::down(),
            Move::Left => P::left(),
        }
    }

    pub fn invert_by_y(self) -> Self {
        match self {
            Move::Up => Move::Down,
            Move::Right => Move::Right,
            Move::Down => Move::Up,
            Move::Left => Move::Left,
        }
    }

    pub fn opposite(self) -> Self {
        match self {
            Move::Up => Move::Down,
            Move::Right => Move::Left,
            Move::Down => Move::Up,
            Move::Left => Move::Right,
        }
    }
}

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
                    .enumerate()
                    .map(|(x, ch)| {
                        if ch == '@' {
                            pos = P::pair(x as i32, y as i32);
                        }

                        match ch {
                            '#' => -1,
                            'O' => 1,
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
    let free_spot = find_free_spot(map, pos, mv);
    if free_spot.is_none() {
        return pos;
    }

    let free_spot = free_spot.unwrap();
    move_blocks(map, free_spot, pos, mv.opposite().to_p());

    pos + mv.to_p()
}

pub fn find_free_spot(map: &[Vec<i8>], pos: P, mv: Move) -> Option<P> {
    let new_pos = pos + mv.to_p();
    match map.get_by_p(new_pos) {
        // Free spot
        Some(cell) if *cell == 0 => Some(new_pos),
        // Wall
        Some(cell) if *cell == -1 => None,
        Some(_) => find_free_spot(map, new_pos, mv),
        None => None,
    }
}

pub fn move_blocks(map: &mut [Vec<i8>], start: P, cur_pos: P, dir: P) {
    if start == cur_pos {
        return;
    }

    let cur_value = map.get_by_p(start).unwrap().clone();
    let other_pos = start + dir;
    let other_value = map.get_by_p(other_pos).unwrap().clone();

    *map.get_mut_by_p(start).unwrap() = other_value;
    *map.get_mut_by_p(other_pos).unwrap() = cur_value;
    move_blocks(map, other_pos, cur_pos, dir);
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
            "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<",
        );
        let answer = 2028;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_2() {
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
        let answer = 10092;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d15p1_answer")
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
