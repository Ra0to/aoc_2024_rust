mod day_1_1;
use day_1_1 as day;

pub fn main() {
    let input = day::read_input();
    let answer = day::solve(input);
    dbg!(answer);
}
