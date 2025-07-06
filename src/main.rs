use std::fs::read_to_string;
use std::iter::empty;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use aoc::*;


fn main() {
    let runners: Vec<_> = empty()
        .chain(year2015())
        .collect();

    let mut duration = Duration::ZERO;

    for Runner { year, day, path, execute } in &runners {
        if let Ok(data) = read_to_string(path) {
            let instant = Instant::now();
            let (answer1, answer2) = execute(data);
            duration += instant.elapsed();

            println!("{year} Day {day:02} [{}ms]", duration.as_millis());
            println!("    Answer 1: {answer1}");
            println!("    Answer 2: {answer2}");
        } else {
            eprintln!("{year} Day {day:02}");
            eprintln!("    Missing input!");
            eprintln!("    Place input file in {}", path.display());
        }
    }
}

macro_rules! run {
    ($year:tt $($day:tt),*) => {
        fn $year() -> Vec<Runner> {
            vec![$({
                let year = stringify!($year);
                let day  = stringify!($day);
                let path = Path::new("input").join(year).join(day).with_extension("txt");

                let execute = |data: String| {
                    use $year::$day::*;

                    let input = parse(&data);
                    let part1 = solve1(&input);
                    let part2 = solve2(&input);

                    (part1.to_string(), part2.to_string())
                };

                Runner { year: year.to_string(), day: day.to_string(), path, execute }
            },)*]
        }
    }
}

struct Runner {
    year: String,
    day: String,
    path: PathBuf,
    execute: fn(String) -> (String, String),
}

run!(year2015
    day01, day02, day03
);
