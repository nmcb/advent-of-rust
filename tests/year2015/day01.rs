use aoc::year2015::day01::*;

const EXAMPLE: &str = "()())";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(solve1(&input), -1);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(solve2(&input), 5);
}