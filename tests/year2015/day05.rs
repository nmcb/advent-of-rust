use aoc::year2015::day05::*;
use crate::input;

#[test]
fn answer1_test() {
    let data   = input("year2015", "day05");
    let input  = parse(data.as_str());
    let answer = solve1(&input);
    assert_eq!(answer, 258);
}

#[test]
fn answer2_test() {
    let data   = input("year2015", "day05");
    let input  = parse(data.as_str());
    let answer = solve2(&input);
    assert_eq!(answer, 53);
}
