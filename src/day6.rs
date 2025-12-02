use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
struct Bank {
    content: (
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
    ),
}

pub fn part1(file_path: &str) -> u32 {
    let mut memory: Vec<Bank> = Vec::new();
    let mut state = parse_input(file_path);
    let mut cycles = 0;

    loop {
        let bank = Bank::array_to_bank(&state);
        if memory.contains(&bank) {
            break;
        }
        memory.push(bank);
        state = cycle(state);
        cycles += 1;
    }

    cycles
}

pub fn part2(file_path: &str) -> u32 {
    let mut memory: Vec<Bank> = Vec::new();
    let mut cycles_count: Vec<u32> = Vec::new();
    let mut state = parse_input(file_path);
    let mut bank: Bank;

    loop {
        bank = Bank::array_to_bank(&state);
        if memory.contains(&bank) {
            break;
        }
        memory.push(bank);
        cycles_count.push(0);
        for i in 0..cycles_count.len() {
            cycles_count[i] += 1;
        }
        state = cycle(state);
    }

    let mut i = 0;
    while memory[i] != bank {
        i += 1;
    }

    cycles_count[i]
}

fn cycle(state: [u32; 16]) -> [u32; 16] {
    let mut result = state.clone();
    let mut max_index = 0;
    let mut max = 0;
    for i in 0..result.len() {
        if max < result[i] {
            max_index = i;
            max = result[i];
        }
    }

    result[max_index] = 0;

    for _ in 0..max {
        max_index = (max_index + 1) % result.len();
        result[max_index] += 1;
    }

    result
}

fn parse_input(file_path: &str) -> [u32; 16] {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut result: [u32; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut done = false;

    'outer: for line in reader.lines() {
        if let Ok(line) = line {
            let line = line.trim();
            let split: Vec<&str> = line.split("\t").collect();
            if split.len() != 16 {
                continue;
            }
            for i in 0..16 {
                if let Ok(val) = split[i].parse::<u32>() {
                    result[i] = val;
                } else {
                    continue 'outer;
                }
            }
            done = true;
        }
    }

    if !done {
        panic!("Could not parse input.");
    }

    result
}

impl Bank {
    pub fn array_to_bank(array: &[u32; 16]) -> Self {
        Self {
            content: (
                array[0], array[1], array[2], array[3], array[4], array[5], array[6], array[7],
                array[8], array[9], array[10], array[11], array[12], array[13], array[14],
                array[15],
            ),
        }
    }
}

impl PartialEq for Bank {
    fn eq(&self, other: &Self) -> bool {
        self.content.0 == other.content.0
            && self.content.1 == other.content.1
            && self.content.2 == other.content.2
            && self.content.3 == other.content.3
            && self.content.4 == other.content.4
            && self.content.5 == other.content.5
            && self.content.6 == other.content.6
            && self.content.7 == other.content.7
            && self.content.8 == other.content.8
            && self.content.9 == other.content.9
            && self.content.10 == other.content.10
            && self.content.11 == other.content.11
            && self.content.12 == other.content.12
            && self.content.13 == other.content.13
            && self.content.14 == other.content.14
            && self.content.15 == other.content.15
    }
}
