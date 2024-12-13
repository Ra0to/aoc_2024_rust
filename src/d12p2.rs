// Problem: https://adventofcode.com/2024/day/12

use crate::d12p1;
use crate::extensions::*;
use crate::point::*;

#[allow(dead_code)]
pub fn read_input() -> Vec<Vec<char>> {
    d12p1::read_input()
}

#[allow(dead_code)]
pub fn solve(input: Vec<Vec<char>>) -> u32 {
    let mut total_price = 0;
    let mut visited = vec![vec![false; input.width()]; input.height()];
    let mut used_perimeters = vec![vec![UsedPerimeter::unmarked(); input.width()]; input.height()];

    for (y, line) in input.iter().enumerate() {
        for (x, _el) in line.iter().enumerate() {
            let p = P::pair(x as i32, y as i32);
            if *visited.get_by_p(p).unwrap() {
                continue;
            }

            let (perimeter, area) = process_field(&input, &mut visited, &mut used_perimeters, p);
            total_price += perimeter * area;
        }
    }

    total_price
}

fn process_field(
    input: &[Vec<char>],
    visited: &mut [Vec<bool>],
    used_perimeters: &mut [Vec<UsedPerimeter>],
    p: P,
) -> (u32, u32) {
    *visited.get_mut_by_p(p).unwrap() = true;
    let current = *input.get_by_p(p).unwrap();
    let mut perimeter = 0_u32;
    for dir in DIRECTIONS_4.iter() {
        let pos = p + *dir;
        let neighbor = input.get_by_p(pos);

        if neighbor.is_some_and(|val| *val == current) {
            continue;
        }

        if used_perimeters.get_by_p(p).unwrap().is_marked(*dir) {
            continue;
        }

        perimeter += 1;
        mark_perimeter_as_used(input, used_perimeters, p, *dir);
    }

    let mut area = 1;

    for dir in DIRECTIONS_4.iter() {
        let pos = p + *dir;
        if !input.get_by_p(pos).is_some_and(|val| *val == current) {
            continue;
        }
        if *visited.get_by_p(pos).unwrap() {
            continue;
        }
        let (neighbor_per, neighbor_area) = process_field(input, visited, used_perimeters, pos);
        perimeter += neighbor_per;
        area += neighbor_area;
    }

    (perimeter, area)
}

fn mark_perimeter_as_used(
    input: &[Vec<char>],
    used_perimeters: &mut [Vec<UsedPerimeter>],
    start: P,
    dir: P,
) {
    let mut pos = start;
    let current = *input.get_by_p(start).unwrap();
    used_perimeters.get_mut_by_p(start).unwrap().mark(dir);
    let move_dir = if dir == P::left() || dir == P::right() {
        P::up()
    } else {
        P::right()
    };
    loop {
        pos = pos + move_dir;
        let neighbor = input.get_by_p(pos + dir);
        match input.get_by_p(pos) {
            None => break,
            Some(new) if *new != current => break,
            Some(_) if !neighbor.is_some_and(|val| *val == current) => {
                used_perimeters.get_mut_by_p(pos).unwrap().mark(dir)
            }
            Some(_) => break,
        }
    }
    let move_dir = if dir == P::left() || dir == P::right() {
        P::down()
    } else {
        P::left()
    };
    pos = start;
    loop {
        pos = pos + move_dir;
        let neighbor = input.get_by_p(pos + dir);
        match input.get_by_p(pos) {
            None => break,
            Some(new) if *new != current => break,
            Some(_) if !neighbor.is_some_and(|val| *val == current) => {
                used_perimeters.get_mut_by_p(pos).unwrap().mark(dir)
            }
            Some(_) => break,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct UsedPerimeter {
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
}

impl UsedPerimeter {
    const fn unmarked() -> Self {
        Self {
            top: false,
            bottom: false,
            left: false,
            right: false,
        }
    }

    fn mark(&mut self, p: P) {
        if p == P::up() {
            self.top = true;
        } else if p == P::down() {
            self.bottom = true;
        } else if p == P::left() {
            self.left = true;
        } else if p == P::right() {
            self.right = true;
        } else {
            panic!("unknown direction");
        }
    }

    fn is_marked(&self, p: P) -> bool {
        if p == P::up() {
            self.top
        } else if p == P::down() {
            self.bottom
        } else if p == P::left() {
            self.left
        } else if p == P::right() {
            self.right
        } else {
            panic!("unknown direction");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::d12p1::parse_input;
    use std::fs::read_to_string;

    #[test]
    fn test_1() {
        // Given
        let input = parse_input(
            "
AAAA
BBCD
BBCC
EEEC
",
        );
        let answer = 80;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_2() {
        // Given
        let input = parse_input(
            "
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
",
        );
        let answer = 436;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_3() {
        // Given
        let input = parse_input(
            "
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
",
        );
        let answer = 236;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_4() {
        // Given
        let input = parse_input(
            "
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
",
        );
        let answer = 368;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn test_5() {
        // Given
        let input = parse_input(
            "
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
",
        );
        let answer = 1206;

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d12p2_answer")
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
