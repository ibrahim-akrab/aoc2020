use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

pub fn day5a() -> String {
    let passes = read_data();
    passes
        .iter()
        .map(|s| calculate_id(s))
        .max()
        .unwrap()
        .to_string()
}

pub fn day5b() -> String {
    let passes = read_data();
    passes
        .iter()
        .map(|s| calculate_id(s))
        .sorted()
        .fold_while(0, |previous_id, current_id| {
            if current_id - previous_id == 2 {
                Done(current_id - 1)
            } else {
                Continue(current_id)
            }
        })
        .into_inner()
        .to_string()
}

fn read_data() -> Vec<String> {
    use std::fs;
    fs::read_to_string("inputs/day5.txt")
        .expect("Couldn't read file")
        .split('\n')
        .filter(|&s| s.len() == 10)
        .map(String::from)
        .collect()
}

fn calculate_id(s: &str) -> usize {
    let row = calculate_row(&s[0..7]);
    let col = calculate_col(&s[7..]);

    // println!("{row}, {col}");
    row * 8 + col
}

fn calculate_row(s: &str) -> usize {
    find_index(128, 'F', 'B', s)
}

fn calculate_col(s: &str) -> usize {
    find_index(8, 'L', 'R', s)
}

fn find_index(
    space: usize,
    lower_split_mark: char,
    higher_split_mark: char,
    location: &str,
) -> usize {
    let (start, _) = location
        .chars()
        .fold((0usize, space), |(start, end), mark| match mark {
            mark if mark == lower_split_mark => (start, (start + end) / 2),
            mark if mark == higher_split_mark => ((start + end) / 2, end),
            _ => unreachable!(),
        });
    start
}
