// Problem: https://adventofcode.com/2024/day/14

use crate::point::P;
use std::fs::read_to_string;

#[allow(dead_code)]
pub fn read_input() -> Vec<(P, P)> {
    parse_input(&read_to_string("./inputs/d14").unwrap())
}

pub fn parse_input(input: &str) -> Vec<(P, P)> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            // p=0,4 v=3,-3
            let (position, velocity) = &line.split_once(" ").unwrap();
            let (p_x, p_y) = &position[2..].split_once(",").unwrap();
            let p = P::pair(p_x.parse::<i32>().unwrap(), p_y.parse::<i32>().unwrap());

            let (v_x, v_y) = &velocity[2..].split_once(",").unwrap();
            let v = P::pair(v_x.parse::<i32>().unwrap(), v_y.parse::<i32>().unwrap());

            (p, v)
        })
        .collect()
}

#[allow(dead_code)]
pub fn solve(input: Vec<(P, P)>) -> u32 {
    solve_for_map(input, 100, P::pair(101, 103))
}

#[allow(dead_code)]
pub fn solve_for_map(input: Vec<(P, P)>, time: i32, map_size: P) -> u32 {
    let mut quadrants = [0_u32; 5];
    input
        .into_iter()
        .for_each(|info| quadrants[find_quadrant(info, time, map_size)] += 1);
    quadrants[1] * quadrants[2] * quadrants[3] * quadrants[4]
}

/// Return robot quadrant after N seconds.
/// 0 - in the middle (no quadrant)
/// 1 - top left
/// 2 - top right
/// 3 - bottom left
/// 4 - bottom right
///
/// -------
/// |1 | 2|
/// |  |  |
/// |-----|
/// |  |  |
/// |4 | 3|
/// -------
#[allow(dead_code)]
pub fn find_quadrant(info: (P, P), time: i32, map_size: P) -> usize {
    let start = info.0;
    let v = info.1;
    let raw_pos = start + v * time;
    let pos = get_pos_in_map(raw_pos, map_size);
    let x = pos.x;
    let y = pos.y;

    let half_width = map_size.x / 2;
    let half_height = map_size.y / 2;

    if x == half_width || y == half_height {
        return 0;
    }

    if x < half_width {
        if y < half_height {
            1
        } else {
            4
        }
    } else {
        if y < half_height {
            2
        } else {
            3
        }
    }
}

pub fn get_pos_in_map(pos: P, map_size: P) -> P {
    P::pair(
        get_num_in_borders(pos.x, map_size.x),
        get_num_in_borders(pos.y, map_size.y),
    )
}

pub fn get_num_in_borders(num: i32, limit: i32) -> i32 {
    ((num.abs() % limit) * num.signum() + limit) % limit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Given
        let input = parse_input(
            "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
        );
        let answer = 12;

        // When
        let result = solve_for_map(input, 100, P::pair(11, 7));

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d14p1_answer")
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
