use aoc::year2015::day04::*;
use crate::input;

#[test]
fn answer1_test() {
    let input  = parse(input("year2015", "day04").as_str());
    let answer = solve1(&input);
    assert_eq!(answer, 282749);
}

#[test]
fn answer2_test() {
    let input  = parse(input("year2015", "day04").as_str());
    let answer = solve2(&input);
    assert_eq!(answer, 9962624);
}
