use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part1(file_path: &str) -> i32 {
    let mut instructions = parse_input(file_path);
    let mut cursor: i32 = 0;
    let mut steps = 0;

    while cursor >= 0 && cursor < instructions.len() as i32 {
        let jump = instructions[cursor as usize];
        instructions[cursor as usize] += 1;
        cursor = cursor + jump;
        steps += 1
    }

    steps
}

pub fn part2(file_path: &str) -> i32 {
    let mut instructions = parse_input(file_path);
    let mut cursor: i32 = 0;
    let mut steps = 0;

    while cursor >= 0 && cursor < instructions.len() as i32 {
        let jump = instructions[cursor as usize];
        instructions[cursor as usize] += if jump >= 3 { -1 } else { 1 };
        cursor = cursor + jump;
        steps += 1
    }

    steps
}

fn parse_input(file_path: &str) -> Vec<i32> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut result = vec![];

    for line in reader.lines() {
        if let Ok(line) = line {
            let line = line.trim();
            if let Ok(value) = line.parse::<i32>() {
                result.push(value);
            }
        }
    }

    result
}
