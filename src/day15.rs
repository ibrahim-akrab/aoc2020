use std::collections::HashMap;

pub fn day15a() -> String {
    let starting_numbers = read_data();
    let mut elves = Elves::default();
    elves.play(starting_numbers, 2020).to_string()
}

pub fn day15b() -> String {
    let starting_numbers = read_data();
    let mut elves = Elves::default();
    elves.play(starting_numbers, 30000000).to_string()
}

fn read_data() -> Vec<usize> {
    std::fs::read_to_string("inputs/day15.txt")
        // std::fs::read_to_string("test.txt")
        .expect("Couldn't read file")
        .trim()
        .split(',')
        .filter_map(|n| n.parse().ok())
        .collect()
}

struct Elves {
    mem: HashMap<usize, (usize, usize)>,
    last_number: usize,
    turn: usize,
}

impl Default for Elves {
    fn default() -> Self {
        Self {
            mem: Default::default(),
            last_number: Default::default(),
            turn: 1,
        }
    }
}

impl Elves {
    fn play(&mut self, numbers: Vec<usize>, nth: usize) -> usize {
        // println!("numbers: {numbers:#?}");
        numbers.iter().for_each(|&number| {
            self.mem.insert(number, (self.turn, 0));
            self.last_number = number;
            self.turn += 1;
        });
        // while self.turn < 12 {
        while self.turn < nth {
            // println!("turn: {}", self.turn);
            // println!("{:#?}", self.mem);
            self.take_turns();
            // println!("number spoken:{}", self.last_number);
        }
        self.take_turns();
        self.last_number
    }

    fn take_turns(&mut self) {
        match self.mem.get(&self.last_number) {
            // number said only once
            Some((_, 0)) => {
                self.last_number = 0;
            }
            // number said at least twice before
            Some((l1, l0)) => {
                self.last_number = l1 - l0;
            }
            // number never spoken before
            None => {
                self.last_number = 0;
            }
        }
        self.mem
            .entry(self.last_number)
            .and_modify(|v| *v = (self.turn, v.0))
            .or_insert((self.turn, 0));
        self.turn += 1;
    }
}
