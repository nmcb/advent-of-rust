pub fn answer1(input: &str) -> i32 {
    let mut result = 0; 
    for c in input.chars() {
        if c == '(' { result += 1 } else { result -= 1 }
    }
    result
}

pub fn answer2(input: &str) -> i32 {
    let mut counter = 0;
    let mut result = 0;
    for c in input.chars() {
        counter += 1;
        if c == '(' { result += 1 } else { result -= 1 }
        if result == -1 { break }
    }
    counter
}