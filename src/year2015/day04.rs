pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn solve1(input: &str) -> usize {
    solve(input, "00000")
}

pub fn solve2(input: &str) -> usize {
    solve(input, "000000")
}

const HEX_CHAR_LOOKUP: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7',
    '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'
];

fn as_hex(array: [u8; 16]) -> String {
    let mut result = String::new();
    for byte in array {
        result.push(HEX_CHAR_LOOKUP[(byte >> 4) as usize]);
        result.push(HEX_CHAR_LOOKUP[(byte & 0xF) as usize]);
    }
    result
}

fn hashing(input: &str, count: usize) -> String {
    let mut data = input.to_owned();
    data.push_str(count.to_string().as_str());
    let digest = md5::compute(&data);
    as_hex(*digest)
}

fn solve(input: &str, prefix: &str) -> usize {
    let mut count = 0;
    let mut hash  = hashing(input, count);
    while !hash.starts_with(prefix) {
        count += 1;
        hash = hashing(input, count);
    }
    count
}


