use aoc::year2015::day01::*;

use crate::input;

#[test]
fn answer1_test() {
    let input  = parse(input("year2015", "day01").as_str());
    let answer = solve1(&input);
    assert_eq!(answer, 138);
}

#[test]
fn answer2_test() {
    let input  = parse(input("year2015", "day01").as_str());
    let answer = solve2(&input);
    assert_eq!(answer, 1771);
}
