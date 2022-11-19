use std::collections::VecDeque;

use itertools::Itertools;

pub fn day10a() -> String {
    let mut adapters = read_data();
    // add the seat charging outlet joltage
    adapters.push(0);
    adapters.sort_unstable();
    //add the joltage of the device input
    adapters.push(adapters[adapters.len() - 1] + 3);
    let differences = adapters.windows(2).map(|w| w[1] - w[0]).counts();
    println!("{differences:#?}");
    (differences[&1] * differences[&3]).to_string()
}

pub fn day10b() -> String {
    let mut adapters = read_data();
    // add the seat charging outlet joltage
    adapters.push(0);
    adapters.sort_unstable();
    //add the joltage of the device input
    adapters.push(adapters[adapters.len() - 1] + 3);

    let valid_connections = adapters
        .windows(4)
        .map(|window| {
            let input = window[0];
            let closest_adapters = &window[1..];
            closest_adapters
                .iter()
                .fold(0, |valid_arrangements, adapter| {
                    if adapter - input <= 3 {
                        valid_arrangements + 1
                    } else {
                        valid_arrangements
                    }
                })
        })
        .collect::<Vec<usize>>();

    valid_connections
        .iter()
        .rev()
        .fold(
            &mut VecDeque::from([1usize; 3]),
            |last_arrangements, &connection| {
                last_arrangements.push_front(last_arrangements.iter().take(connection).sum());
                last_arrangements.pop_back();
                last_arrangements
            },
        )
        .pop_front()
        .unwrap()
        .to_string()
}

fn read_data() -> Vec<usize> {
    std::fs::read_to_string("inputs/day10.txt")
        .expect("Couldn't read file")
        .lines()
        .filter_map(|x| x.parse().ok())
        .collect()
}
