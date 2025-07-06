use aoc::year2015::day03::*;
use crate::input;

#[test]
fn answer1_test() {
    let input  = parse(input("year2015", "day03").as_str());
    let answer = solve1(&input);
    assert_eq!(answer, 2572);
}

#[test]
fn answer2_test() {
    let input  = parse(input("year2015", "day03").as_str());
    let answer = solve2(&input);
    assert_eq!(answer, 2631);
}
