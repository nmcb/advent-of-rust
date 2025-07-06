use std::fs::read_to_string;
use std::path::Path;

macro_rules! test {
    ($year:tt $($day:tt),*) => {
        pub mod $year {$(pub mod $day;)*}
    }
}

test!(year2015
    day01, day02, day03
);


fn input(year: &str, day: &str) -> String {
    let path = Path::new("input").join(year).join(day).with_extension("txt");

    if let Ok(data) = read_to_string(&path) {
        data
    } else {
        panic!("Missing input! Place input in {}", path.display());
    }
}
