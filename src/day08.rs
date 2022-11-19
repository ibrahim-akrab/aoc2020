use itertools::Itertools;

pub fn day8a() -> String {
    let instructions = read_data();
    run(&instructions).to_string()
}

pub fn day8b() -> String {
    let instructions = read_data();
    let wrong_instruction = instructions
        .iter()
        .enumerate()
        .filter(|(_, inst)| matches!(inst, Instruction::Jmp(_) | Instruction::Nop(_)))
        .filter_map(|(i, _)| repair_boot(&instructions, i))
        .exactly_one();
    match wrong_instruction {
        Ok(i) => i.to_string(),
        _ => "No solution was found".to_string(),
    }
}

#[derive(Debug)]
enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

impl Instruction {
    fn create(s: &str) -> Option<Self> {
        let operation = &s[0..3];
        let digit = s[4..].parse::<isize>().ok()?;
        match operation {
            "acc" => Some(Instruction::Acc(digit)),
            "jmp" => Some(Instruction::Jmp(digit)),
            "nop" => Some(Instruction::Nop(digit)),
            _ => None,
        }
    }
}

fn run(instructions: &Vec<Instruction>) -> isize {
    let mut executed = vec![false; instructions.len()];
    let mut pointer: usize = 0;
    let mut acc: isize = 0;
    while !executed[pointer] {
        let instruction = &instructions[pointer];
        executed[pointer] = true;
        match instruction {
            Instruction::Acc(val) => {
                acc += val;
            }
            Instruction::Jmp(val) => {
                pointer = ((pointer as isize) + val - 1) as usize;
            }
            Instruction::Nop(_) => (),
        }
        pointer += 1;
    }
    acc
}

fn repair_boot(instructions: &Vec<Instruction>, flipped_instruction: usize) -> Option<isize> {
    let mut executed = vec![false; instructions.len()];
    let mut pointer: usize = 0;
    let mut acc: isize = 0;
    while !executed[pointer] {
        let instruction = &instructions[pointer];
        executed[pointer] = true;
        match (instruction, pointer) {
            (Instruction::Acc(val), _) => {
                acc += val;
            }
            (Instruction::Nop(val), n) if n == flipped_instruction => {
                pointer = ((pointer as isize) + val - 1) as usize;
            }
            (Instruction::Jmp(_), n) if n == flipped_instruction => (),

            (Instruction::Jmp(val), _) => {
                pointer = ((pointer as isize) + val - 1) as usize;
            }
            (Instruction::Nop(_), _) => (),
        }
        pointer += 1;
        if pointer == instructions.len() {
            return Some(acc);
        }
    }
    None
}

fn read_data() -> Vec<Instruction> {
    use std::fs;
    fs::read_to_string("inputs/day8.txt")
        .expect("Couldn't read file")
        .lines()
        .filter_map(Instruction::create)
        .collect()
}
