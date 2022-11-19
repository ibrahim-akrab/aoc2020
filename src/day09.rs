use itertools::Itertools;

pub fn day9a() -> String {
    let numbers = read_data();
    let preamble: usize = 25;
    let wrong_number = find_invalid_number(&numbers, preamble);
    match wrong_number {
        Some(number) => number.to_string(),
        None => "No solution was found".to_string(),
    }
}

pub fn day9b() -> String {
    let numbers = read_data();
    let preamble: usize = 25;
    let invalid_number = find_invalid_number(&numbers, preamble).unwrap();
    find_xmas_weakness(&numbers, invalid_number).to_string()
}

fn find_invalid_number(numbers: &[u64], preamble: usize) -> Option<u64> {
    numbers
        .windows(preamble + 1)
        .find(|window| {
            let target = window[preamble];
            let win = &window[0..preamble];
            !win.iter()
                .combinations(2)
                .any(|v| v.iter().copied().sum::<u64>() == target)
        })
        .map(|w| w[preamble])
}

fn find_xmas_weakness(numbers: &[u64], invalid_number: u64) -> u64 {
    let (mut start_idx, mut end_idx) = (0, 0);
    let mut acc = numbers[start_idx];

    while acc != invalid_number {
        if acc < invalid_number {
            end_idx += 1;
            acc += numbers[end_idx];
        } else {
            acc -= numbers[start_idx];
            start_idx += 1;
        }
    }
    let smallest_number = numbers[start_idx..=end_idx].iter().min().unwrap();
    let largest_number = numbers[start_idx..=end_idx].iter().max().unwrap();

    smallest_number + largest_number
}

fn read_data() -> Vec<u64> {
    use std::fs;
    fs::read_to_string("inputs/day9.txt")
        .expect("Couldn't read file")
        .lines()
        .filter_map(|x| x.parse().ok())
        .collect()
}
