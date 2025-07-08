use fancy_regex::*;

pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn solve1(input: &Vec<&str>) -> usize {
    let mut count = 0;
    for line in input.iter() {
        if count_vowels(line) >= 3 && count_adjacent(line) >= 1 && is_legal(line) {
            count += 1;
        }
    }
    count
}

pub fn solve2(input: &Vec<&str>) -> usize {
    let two_pair = Regex::new("(..).*\\1").unwrap();
    let triple   = Regex::new("(.).\\1").unwrap();
    input.iter().filter(|line| {
        two_pair.is_match(line).unwrap() && triple.is_match(line).unwrap()
    }).count()
}

fn count_vowels(s: &str) -> usize {
    let mut count = 0;
    for c in s.chars() {
        if "aeoui".contains(c) {
            count += 1;
        }
    }
    count
}

fn count_adjacent(s: &str) -> usize {
    let mut count = 0;
    for window in sliding(s, 2) {
        let mut chars = window.chars();
        if chars.next() == chars.next() {
            count += 1;
        }
    }
    count
}

fn is_legal(s: &str) -> bool {
    for cc in sliding(s, 2) {
        if ["ab", "cd", "pq", "xy"].contains(&cc) {
            return false;
        }
    }
    true
}

fn sliding(src: &str, size: usize) -> impl Iterator<Item=&str> {
    src.char_indices()
        .flat_map(move |(from, _)| {
            src[from..].char_indices()
                .skip(size - 1)
                .next()
                .map(|(to, c)| {
                    &src[from..from + to + c.len_utf8()]
                })
        })
}