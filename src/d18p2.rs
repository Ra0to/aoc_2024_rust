// Problem: https://adventofcode.com/2024/day/18

use crate::d18p1;
use crate::extensions::*;
use crate::point::*;

#[allow(dead_code)]
pub fn read_input() -> Vec<P> {
    d18p1::read_input()
}

#[allow(dead_code)]
pub fn solve(input: Vec<P>) -> String {
    let p = solve_for_map(input, 71, 1024);
    format!("{},{}", p.x, p.y)
}

#[allow(dead_code)]
pub fn solve_for_map(input: Vec<P>, map_size: usize, time: usize) -> P {
    let n = map_size;
    let start = P::zero();
    let end = P::pair((n - 1) as i32, (n - 1) as i32);

    let timeframes = (time..input.len()).into_iter().collect::<Vec<usize>>();
    let ind = timeframes.partition_point(|t| {
        d18p1::find_shortest_path(&get_map_at_time(&input, map_size, *t), start, end).is_some()
    });
    let no_path_time = timeframes[ind - 1];
    input[no_path_time]
}

pub fn get_map_at_time(input: &[P], map_size: usize, time: usize) -> Vec<Vec<i8>> {
    let n = map_size;
    let mut map: Vec<Vec<i8>> = vec![vec![0; n]; n];

    for i in 0..time {
        let byte = input[i];
        *map.get_mut_by_p(byte).unwrap() = -1;
    }

    map
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;
    use crate::d18p1::parse_input;

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
        let answer = P::pair(6, 1);

        // When
        let result = solve_for_map(input, 7, 12);

        // Then
        assert_eq!(result, answer);
    }

    #[test]
    fn problem() {
        // Given
        let input = read_input();
        let answer = read_to_string("./inputs/d18p2_answer").unwrap();
        let answer = answer.trim();

        // When
        let result = solve(input);

        // Then
        assert_eq!(result, answer);
    }
}
