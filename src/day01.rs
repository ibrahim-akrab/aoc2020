use itertools::Itertools;

pub fn day1a() -> String {
    find_expense(2)
}

pub fn day1b() -> String {
    find_expense(3)
}

fn find_expense(n: usize) -> String {
    let values = read_data();
    let result: Option<_> = values
        .iter()
        .combinations(n)
        .find(|v| v.iter().copied().sum::<usize>() == 2020)
        .map(|v| v.into_iter().product::<usize>());
    match result {
        Some(v) => v.to_string(),
        None => "No solution".to_string(),
    }
}

fn read_data() -> Vec<usize> {
    use std::fs;
    let values = fs::read_to_string("inputs/day01.txt").expect("Couldn't read file");
    values
        .split('\n')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect()
}
