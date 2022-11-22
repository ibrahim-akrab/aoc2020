use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

pub fn day14a() -> String {
    let instructions = read_data();
    let mut vm = Vm::default();
    instructions.iter().for_each(|inst| vm.exec_v1(*inst));
    vm.sum().to_string()
}

pub fn day14b() -> String {
    let instructions = read_data();
    let mut vm = Vm::default();
    instructions.iter().for_each(|inst| vm.exec_v2(*inst));
    vm.sum().to_string()
}

fn read_data() -> Vec<Instruction> {
    let re: Regex = Regex::new(r".*\[(\d+)\].* (\d+)$").unwrap();
    std::fs::read_to_string("inputs/day14.txt")
        .expect("Couldn't read file")
        .lines()
        .map(|s| Instruction::new(s, &re))
        .collect()
}

#[derive(Clone, Copy)]
enum Instruction {
    Mask(u64, u64, u64),
    Mem(u64, u64),
}

impl Instruction {
    fn new(s: &str, re: &Regex) -> Self {
        if s.starts_with("mask") {
            let (mask_ones, mask_zeros, mask_floating) = Self::parse_mask(s);
            Self::Mask(mask_ones, mask_zeros, mask_floating)
        } else {
            let (address, value) = Self::parse_mem(s, re);
            Self::Mem(address, value)
        }
    }

    fn parse_mask(s: &str) -> (u64, u64, u64) {
        let mut mask_ones: u64 = 0;
        let mut mask_zeros: u64 = 0;
        let mut mask_floating: u64 = 0;
        for bit in s[7..].chars() {
            mask_ones <<= 1;
            mask_zeros <<= 1;
            mask_floating <<= 1;
            match bit {
                '1' => mask_ones |= 1,
                '0' => mask_zeros |= 1,
                'X' => mask_floating |= 1,
                _ => (),
            }
        }
        (mask_ones, mask_zeros, mask_floating)
    }

    fn parse_mem(s: &str, re: &Regex) -> (u64, u64) {
        let caps = re.captures(s).unwrap();
        let address = caps.get(1).unwrap().as_str().parse().unwrap();
        let value = caps.get(2).unwrap().as_str().parse().unwrap();
        (address, value)
    }
}

#[derive(Default)]
struct Vm {
    mem: HashMap<u64, u64>,
    mask_ones: u64,
    mask_zeros: u64,
    mask_floating: u64,
}

impl Vm {
    fn exec_v1(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Mask(mask_ones, mask_zeros, mask_floating) => {
                (self.mask_ones, self.mask_zeros, self.mask_floating) =
                    (mask_ones, mask_zeros, mask_floating);
            }
            Instruction::Mem(add, val) => {
                let value = (val | self.mask_ones) & !self.mask_zeros;
                self.mem
                    .entry(add)
                    .and_modify(|v| *v = value)
                    .or_insert(value);
            }
        }
    }

    fn exec_v2(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Mask(mask_ones, mask_zeros, mask_floating) => {
                (self.mask_ones, self.mask_zeros, self.mask_floating) =
                    (mask_ones, mask_zeros, mask_floating);
            }
            Instruction::Mem(add, val) => {
                let floating_pos = (0..self.mask_floating.count_ones() as usize + 1)
                    .flat_map(|k| BitIndexIter::new(self.mask_floating).combinations(k))
                    .collect::<Vec<Vec<usize>>>();

                let base_address = (add | self.mask_ones) & !self.mask_floating;
                floating_pos.iter().for_each(|v| {
                    let floating_address = v
                        .iter()
                        .fold(base_address, |new_add, pos| new_add | (1 << pos));
                    self.mem
                        .entry(floating_address)
                        .and_modify(|v| *v = val)
                        .or_insert(val);
                });
            }
        }
    }

    fn sum(&self) -> u64 {
        self.mem.values().sum()
    }
}

struct BitIndexIter(u64);

impl BitIndexIter {
    fn new(value: u64) -> Self {
        Self(value)
    }
}

impl Iterator for BitIndexIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            0 => None,
            val => {
                let index = val.trailing_zeros();
                self.0 ^= 1 << index;
                Some(index as usize)
            }
        }
    }
}
