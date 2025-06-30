use std::fs;

use crate::year2015::*;

pub mod year2015;

fn main() {
    let input = fs::read_to_string("input/2015/input01.txt").expect("File not found");
    let answer1 = day01::answer1(input.trim());
    println!("Answer 1: {}", answer1);
    let answer2 = day01::answer2(input.trim());
    println!("Answer 2: {}", answer2);

    let input = fs::read_to_string("input/2015/input02.txt").expect("File not found");
    let answer1 = day02::answer1(input.trim());
    println!("Answer 1: {}", answer1);
    let answer2 = day02::answer2(input.trim());
    println!("Answer 2: {}", answer2);

}