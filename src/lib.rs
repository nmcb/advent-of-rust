macro_rules! library {
    ($year:tt $description:literal $($day:tt),*) => {
        #[doc = concat!("# ", $description)]
        pub mod $year {$(pub mod $day;)*}
    }
}

library!(util "Common utilities for the AOC"
    pos
);

library!(year2015 "Fix the weather machine's snow function."
    day01, day02, day03, day04, day05
);
