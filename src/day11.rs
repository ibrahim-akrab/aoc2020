use std::fmt::{Display, Write};

use itertools::Itertools;

pub fn day11a() -> String {
    let layout = read_data();
    let final_state = layout.simulate(Layout::count_adjacent, 4);
    final_state
        .seats
        .iter()
        .map(|row| {
            row.iter()
                .filter(|seat| matches!(seat, Seat::Occupied))
                .count()
        })
        .sum::<usize>()
        .to_string()
}

pub fn day11b() -> String {
    let layout = read_data();
    let final_state = layout.simulate(Layout::count_visible, 5);
    final_state
        .seats
        .iter()
        .map(|row| {
            row.iter()
                .filter(|seat| matches!(seat, Seat::Occupied))
                .count()
        })
        .sum::<usize>()
        .to_string()
}

#[derive(Clone, Eq, PartialEq)]
enum Seat {
    Empty,
    Occupied,
    Floor,
}

impl Display for Seat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Floor => f.write_str("."),
            Self::Empty => f.write_str("L"),
            Self::Occupied => f.write_str("#"),
        }
    }
}

impl Seat {
    fn new(s: char) -> Self {
        match s {
            'L' => Self::Empty,
            '.' => Self::Floor,
            '#' => Self::Occupied,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Layout {
    seats: Vec<Vec<Seat>>,
    rows: usize,
    cols: usize,
}
impl Display for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.seats.iter().for_each(|row| {
            row.iter().for_each(|seat| {
                f.write_fmt(format_args!("{}", seat));
            });
            f.write_char('\n');
        });
        Ok(())
    }
}
type CountFn = fn(&Layout, usize, usize) -> usize;

impl Layout {
    fn new(seats: Vec<Vec<Seat>>) -> Self {
        let rows = seats.len();
        let cols = seats[0].len();
        Self { seats, rows, cols }
    }

    fn simulate(&self, count_fn: CountFn, max_occupied: usize) -> Layout {
        let mut last = self.clone();
        loop {
            let next = last.simulate_once(count_fn, max_occupied);
            // exit if no change
            if next == last {
                return next;
            }
            last = next;
        }
    }

    fn simulate_once(&mut self, count_fn: CountFn, max_occupied: usize) -> Self {
        let mut result = self.clone();
        self.seats.iter().enumerate().for_each(|(row, seats)| {
            seats.iter().enumerate().for_each(|(col, seat)| {
                let occupied_neighbors = count_fn(self, row, col);
                match *seat {
                    Seat::Occupied => {
                        if occupied_neighbors >= max_occupied {
                            result.seats[row][col] = Seat::Empty;
                        }
                    }
                    Seat::Empty => {
                        if occupied_neighbors == 0 {
                            result.seats[row][col] = Seat::Occupied;
                        }
                    }
                    _ => (),
                }
            })
        });
        result
    }

    fn count_visible(&self, row: usize, col: usize) -> usize {
        (-1..=1)
            .cartesian_product(-1..=1)
            .filter_map(|(r, c)| self.check_visible(row, col, r, c))
            .filter(|seat| matches!(seat, Seat::Occupied))
            .count()
    }

    fn check_visible(
        &self,
        row: usize,
        col: usize,
        row_dir: isize,
        col_dir: isize,
    ) -> Option<Seat> {
        if row_dir == 0 && col_dir == 0 {
            return None;
        }
        let mut r = row as isize + row_dir;
        let mut c = col as isize + col_dir;
        while (0..self.rows as isize).contains(&r) && (0..self.cols as isize).contains(&c) {
            match self.get(r as usize, c as usize) {
                Some(Seat::Occupied) => return Some(Seat::Occupied),
                Some(Seat::Empty) => return Some(Seat::Empty),
                _ => {
                    r += row_dir;
                    c += col_dir;
                }
            }
        }
        None
    }

    fn count_adjacent(&self, row: usize, col: usize) -> usize {
        let mut count = 0usize;
        let (min_row, max_row): (usize, usize) = if row == 0 {
            (row, row + 1)
        } else if row == self.rows - 1 {
            (row - 1, row)
        } else {
            (row - 1, row + 1)
        };
        let (min_col, max_col): (usize, usize) = if col == 0 {
            (col, col + 1)
        } else if col == self.cols - 1 {
            (col - 1, col)
        } else {
            (col - 1, col + 1)
        };

        for (r, c) in (min_row..=max_row).cartesian_product(min_col..=max_col) {
            match self.get(r, c) {
                Some(Seat::Occupied) => {
                    count += 1;
                }
                _ => continue,
            }
        }
        match self.get(row, col) {
            Some(Seat::Occupied) => count - 1,
            _ => count,
        }
    }

    fn get(&self, row: usize, col: usize) -> Option<&Seat> {
        self.seats.get(row)?.get(col)
    }
}

fn read_data() -> Layout {
    Layout::new(
        std::fs::read_to_string("inputs/day11.txt")
            .expect("Couldn't read file")
            .lines()
            .map(|s| s.chars().map(Seat::new).collect())
            .collect::<Vec<Vec<Seat>>>(),
    )
}
