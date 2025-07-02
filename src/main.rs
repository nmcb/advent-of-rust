use std::fs::read_to_string;
use std::iter::empty;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};


pub mod year2015;

fn main() {
    let solutions: Vec<_> = empty()
        .chain(year2015())
        .collect();

    let mut duration = Duration::ZERO;

    for Solution { year, day, path, wrapper } in &solutions {
        if let Ok(data) = read_to_string(path) {
            let instant = Instant::now();
            let (part1, part2) = wrapper(data);
            duration += instant.elapsed();

            println!("{BOLD}{YELLOW}{year} Day {day:02}{RESET}");
            println!("    Part 1: {part1}");
            println!("    Part 2: {part2}");
        } else {
            eprintln!("{BOLD}{RED}{year} Day {day:02}{RESET}");
            eprintln!("    Missing input!");
            eprintln!("    Place input file in {BOLD}{WHITE}{}{RESET}", path.display());
        }
    }
}

macro_rules! run {
    ($year:tt $($day:tt),*) => {
        fn $year() -> Vec<Solution> {
            vec![$({
                let year = stringify!($year);
                let day  = stringify!($day);
                let path = Path::new("input").join(year).join(day).with_extension("txt");

                let wrapper = |data: String| {
                    use $year::$day::*;

                    let input = parse(&data);
                    let part1 = solve1(&input);
                    let part2 = solve2(&input);

                    (part1.to_string(), part2.to_string())
                };

                Solution { year: year.unsigned(), day: day.unsigned(), path, wrapper }
            },)*]
        }
    }
}

struct Solution {
    year: u32,
    day: u32,
    path: PathBuf,
    wrapper: fn(String) -> (String, String),
}

run!(year2015
    day01, day02
);