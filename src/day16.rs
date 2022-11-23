use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

pub fn day16a() -> String {
    let (ranges, _, tickets) = read_data();
    tickets
        .iter()
        .flatten()
        .filter(|&field| {
            !ranges
                .iter()
                .flatten()
                .any(|(low, high)| low <= field && field <= high)
        })
        .sum::<usize>()
        .to_string()
}

pub fn day16b() -> String {
    let (ranges, my_ticket, tickets) = read_data();

    let valid_tickets = tickets
        .iter()
        .filter(|v| {
            v.iter().all(|field| {
                ranges
                    .iter()
                    .flatten()
                    .any(|(low, high)| low <= field && field <= high)
            })
        })
        .collect::<Vec<&Ticket>>();

    let mut valids: Vec<usize> = Vec::new();
    ranges.iter().for_each(|field_range| {
        let mut bitmap = 0usize;
        for i in 0..my_ticket.len() {
            if valid_tickets.iter().rev().all(|ticket| {
                field_range
                    .iter()
                    .any(|(low, high)| (*low..=*high).contains(ticket.get(i).unwrap()))
            }) {
                bitmap |= 1 << i;
            }
        }
        valids.push(bitmap);
    });
    let mut fields_map: HashMap<usize, usize> = HashMap::new();
    // initialize fields_map
    valids
        .iter()
        .enumerate()
        .sorted_by_key(|(_, x)| x.count_ones())
        .fold(0, |mask, (idx, x)| {
            let ticket_field = (x ^ mask).trailing_zeros();
            fields_map.insert(idx, ticket_field as usize);
            mask | (1 << ticket_field)
        });
    (0..=5usize)
        .map(|n| my_ticket[fields_map[&n]])
        .product::<usize>()
        .to_string()
}

type Ticket = Vec<usize>;
type Range = (usize, usize);

fn read_data() -> (Vec<Vec<Range>>, Ticket, Vec<Ticket>) {
    let text = std::fs::read_to_string("inputs/day16.txt")
        .expect("Couldn't read file")
        .split("\n\n")
        .map(String::from)
        .collect::<Vec<String>>();
    let ranges_re: Regex = Regex::new(r"^.*: (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    let valid_ranges = text[0]
        .lines()
        .map(|line| {
            let caps = ranges_re.captures(line).unwrap();
            let mut v = Vec::new();
            for i in 0..2 {
                v.push((
                    (caps[2 * i + 1]).parse::<usize>().unwrap(),
                    (caps[2 * i + 2]).parse::<usize>().unwrap(),
                ));
            }
            v
        })
        .collect();
    let my_ticket = text[1]
        .lines()
        .skip(1)
        .flat_map(|line| line.split(',').map(|s| s.parse::<usize>().unwrap()))
        .collect();
    let nearby_tickets = text[2]
        .lines()
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();
    (valid_ranges, my_ticket, nearby_tickets)
}
