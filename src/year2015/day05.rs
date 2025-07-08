use fancy_regex::*;

pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn solve1(input: &Vec<&str>) -> usize {
    let vowels  = Regex::new("[aeiou].*[aeiou].*[aeiou]").unwrap();
    let pair    = Regex::new("(.)\\1").unwrap();
    let naughty = Regex::new("ab|cd|pq|xy").unwrap();
    input.iter().filter(|line| {
        vowels.is_match(line).unwrap()
            && pair.is_match(line).unwrap()
            && !naughty.is_match(line).unwrap()
    }).count()
}

pub fn solve2(input: &Vec<&str>) -> usize {
    let two_pair = Regex::new("(..).*\\1").unwrap();
    let triple   = Regex::new("(.).\\1").unwrap();
    input.iter().filter(|line| {
        two_pair.is_match(line).unwrap()
            && triple.is_match(line).unwrap()
    }).count()
}