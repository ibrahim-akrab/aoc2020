use aoc2020::{
    day1::{day1a, day1b},
    day2::{day2a, day2b},
    day3::{day3a, day3b},
    day4::{day4a, day4b},
    day5::{day5a, day5b},
    day6::{day6a, day6b},
    day7::{day7a, day7b},
};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let problem = args.get(1).map(|s| s.as_str()).unwrap_or("None");

    let result = match problem {
        "day1a" => day1a(),
        "day1b" => day1b(),
        "day2a" => day2a(),
        "day2b" => day2b(),
        "day3a" => day3a(),
        "day3b" => day3b(),
        "day4a" => day4a(),
        "day4b" => day4b(),
        "day5a" => day5a(),
        "day5b" => day5b(),
        "day6a" => day6a(),
        "day6b" => day6b(),
        "day7a" => day7a(),
        "day7b" => day7b(),
        _ => day7b(),
        // _ => "Not yet solved".to_string(),
    };
    println!("{result}");
}
