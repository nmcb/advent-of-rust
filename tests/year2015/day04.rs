use aoc::year2015::day04::*;
use crate::input;

#[test]
fn answer1_test() {
    let data   = input("year2015", "day04");
    let input  = parse(data.as_str());
    let answer = solve1(input);
    assert_eq!(answer, 282749);
}

#[test]
fn answer2_test() {
    let data   = input("year2015", "day04");
    let input  = parse(data.as_str());
    let answer = solve2(input);
    assert_eq!(answer, 9962624);
}
