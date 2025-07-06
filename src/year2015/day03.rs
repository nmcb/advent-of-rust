use std::collections::HashSet;

use crate::util::pos::*;

pub fn parse(input: &str) -> Vec<Pos> {
    input.trim().bytes().map(Pos::from).collect()
}

pub fn solve1(input: &[Pos]) -> usize {
    deliver(input, |_| true)
}

pub fn solve2(input: &[Pos]) -> usize {
    deliver(input, |i| i.is_multiple_of(2))
}

fn deliver(input: &[Pos], predicate: fn(usize) -> bool) -> usize {
    let mut santa = ORIGIN;
    let mut robot = ORIGIN;
    let mut set = HashSet::new();
    set.insert(ORIGIN);

    for (index, pos) in input.iter().enumerate() {
        if predicate(index) {
            santa += *pos;
            set.insert(santa);
        } else {
            robot += *pos;
            set.insert(robot);
        }
    }

    set.len()
}
