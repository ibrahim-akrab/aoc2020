use itertools::Itertools;

pub fn day13a() -> String {
    let lines = read_data();
    let arrival_timestamp = lines[0].parse().expect("timestamp must be integer");
    let ids: Vec<usize> = lines[1]
        .split(',')
        .filter_map(|id| id.parse().ok())
        .collect();
    let departure_timestamp = (arrival_timestamp..)
        .find_or_first(|time| ids.iter().any(|id| time % id == 0))
        .expect("no solution was found");
    let bus_id = ids
        .iter()
        .find(|&id| departure_timestamp % id == 0)
        .unwrap();
    ((departure_timestamp - arrival_timestamp) * bus_id).to_string()
}

pub fn day13b() -> String {
    let lines = read_data();
    let ids: Vec<Option<u64>> = lines[1].split(',').map(|id| id.parse().ok()).collect();
    // let timestamp = find_timestamp_bruteforce(ids);
    let timestamp = find_timestamp_ring_algorithm(ids);
    timestamp.to_string()
}

fn find_timestamp_bruteforce(ids: Vec<Option<u64>>) -> u64 {
    #![allow(dead_code)]
    let first_id = ids[0].unwrap();
    let lower_bound: u64 = 100000000000000;
    let search_start = (lower_bound..).find(|v| v % first_id == 0).unwrap();
    let timestamp: u64 = (search_start..)
        .step_by(first_id.try_into().unwrap())
        .into_iter()
        .find_or_first(|&timestamp| {
            if timestamp % (1000 * first_id) == 0 {
                println!("timestamp: {timestamp}");
            }
            ids.iter().enumerate().all(|(idx, &id)| {
                if let Some(bus) = id {
                    return (timestamp + idx as u64) % bus == 0;
                }
                true
            })
        })
        .unwrap();
    timestamp
}

fn read_data() -> Vec<String> {
    std::fs::read_to_string("inputs/day13.txt")
        .expect("Couldn't read file")
        .lines()
        .map(String::from)
        .collect()
}

fn find_timestamp_ring_algorithm(ids: Vec<Option<u64>>) -> u64 {
    let residues: Vec<i64> = ids
        .iter()
        .enumerate()
        .filter(|(_, o)| o.is_some())
        .map(|(idx, o)| match o {
            Some(id) => (*id as i64 - idx as i64) % *id as i64,
            None => unreachable!(),
        })
        .collect();
    let modulii: Vec<i64> = ids
        .iter()
        .filter(|o| o.is_some())
        .map(|&o| match o {
            Some(id) => id as i64,
            None => unreachable!(),
        })
        .collect();
    chinese_remainder(&residues, &modulii).expect("bus ids are not pairwise coprime") as u64
}

// https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}
