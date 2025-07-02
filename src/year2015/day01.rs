pub fn parse(input: &str) -> Vec<i32> {
    fn parse(b: u8) -> i32 {
        match b {
            b'(' => 1,
            b')' => -1,
            _ => 0,
        }
    }
    input.bytes().map(parse).collect()
}

pub fn solve1(input: &[i32]) -> i32 {
    input.iter().sum()
}

pub fn solve2(input: &[i32]) -> usize {
    let mut floor = 0;

    for (i, x) in input.iter().enumerate() {
        floor += x;
        if floor < 0 {
            return i + 1;
        }
    }

    unreachable!()
}
