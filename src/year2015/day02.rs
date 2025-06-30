use std::num::ParseIntError;

struct Present {
    l: u32,
    w: u32,
    h: u32,
}

impl Present {
    fn paper_required(&self) -> u32 {
        3 * self.l * self.w + 2 * self.w * self.h + 2 * self.h * self.l
    }
    fn ribbon_required(&self) -> u32 {
        2 * (self.l + self.w) + (self.l * self.w) * self.h
    }
}

fn parse(input: &str) -> Vec<Present> {
    let presents: Result<Vec<Present>, ParseIntError> = input.lines().map(|l| {
            l.split('x')
                .map(|f| f.parse::<u32>())
                .collect::<Result<Vec<u32>, std::num::ParseIntError>>()
                .map(|mut p| {
                    p.sort_unstable();
                    Present { l: p[0], w: p[1], h: p[2] }
                })
        })
        .collect();
    match presents {
        Ok(result) => result,
        Err(msg) => panic!("error: {}", msg)
    }

}

pub fn answer1(input: &str) -> u32 {
    parse(input).iter().map(|p| p.paper_required()).sum()
}

pub fn answer2(input: &str) -> u32 {
    parse(input).iter().map(|p| p.ribbon_required()).sum()
}