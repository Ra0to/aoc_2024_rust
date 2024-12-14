// Problem: https://adventofcode.com/2024/day/14

use crate::d14p1;
use crate::extensions::*;
use crate::point::P;

use image::{Rgb, RgbImage};

#[allow(dead_code)]
pub fn read_input() -> Vec<(P, P)> {
    d14p1::read_input()
}

#[allow(dead_code)]
pub fn solve(input: Vec<(P, P)>) -> u32 {
    let map_size = P::pair(101, 103);
    let loop_time = 10403; // find_loop_time(&input, map_size);
    let first_vertical = 97;
    let mut time = first_vertical;
    while time <= loop_time {
        let map = get_final_map(&input, time, map_size);
        draw_map(&map, time);
        time += map_size.x;
    }
    0
}

pub fn get_final_map(input: &[(P, P)], time: i32, map_size: P) -> Vec<Vec<usize>> {
    let mut map = vec![vec![0; map_size.x as usize]; map_size.y as usize];

    input.into_iter().for_each(|info| {
        *map.get_mut_by_p(d14p1::get_final_pos(*info, time, map_size))
            .unwrap() += 1;
    });

    map
}

pub fn is_maps_equal(input: &[(P, P)], map_size: P, time_lhs: i32, time_rhs: i32) -> bool {
    let left = get_final_map(input, time_lhs, map_size);
    let right = get_final_map(input, time_rhs, map_size);

    for y in 0..left.height() {
        for x in 0..left.width() {
            let p = P::pair(x as i32, y as i32);
            if *left.get_by_p(p).unwrap() != *right.get_by_p(p).unwrap() {
                return false;
            }
        }
    }

    true
}

// Find such time as get_final_map(0) == get_final_map(result)
pub fn find_loop_time(input: &[(P, P)], map_size: P) -> i32 {
    let mut time = 1;
    loop {
        if is_maps_equal(&input, map_size, 0, time) {
            assert!(is_maps_equal(&input, map_size, 1, time + 1));
            return time;
        }

        time += 1;
    }
}

pub fn print_map(map: &[Vec<usize>], time: i32) {
    println!("Time: {}", time);
    println!();
    println!("-------------------------");
    println!();
    for line in map {
        for el in line {
            let ch = match el {
                0 => " ".to_string(),
                _ => el.to_string(),
            };
            print!("{}", ch);
        }
        println!();
    }
    println!();
    println!("-------------------------");
    println!();
}

pub fn draw_map(map: &[Vec<usize>], time: i32) {
    let mut img = RgbImage::new(map.width() as u32, map.height() as u32);

    for (y, line) in map.iter().enumerate() {
        for (x, el) in line.iter().enumerate() {
            let color = match el {
                0 => Rgb([255, 255, 255]),
                _ => Rgb([0, 255, 0]),
            };
            img.put_pixel(x as u32, y as u32, color);
        }
    }

    img.save(format!("output/d14p2_vert/{}.png", time)).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    #[ignore = "not solved yet"]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d14p2_answer")
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
