use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};

const SIZE: usize = 16;
const REPEAT: usize = 1_000_000_000;

pub enum Action {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

pub fn part1(file_path: &str) -> String {
    let mut line = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    ];
    let actions = parse_input(file_path);
    for action in actions {
        line = action.run(line);
    }

    line.iter().collect()
}

pub fn part2(file_path: &str) -> String {
    let mut line = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    ];
    let mut map: HashMap<String, usize> = HashMap::new();
    let actions = parse_input(file_path);
    for i in 1..=REPEAT {
        for action in actions.iter() {
            line = action.run(line);
        }
        let str: String = line.iter().collect();
        if map.contains_key(&str) {
            let cycle = i - map[&str];
            let target = REPEAT % cycle;
            for (k, v) in map.iter() {
                if *v == target {
                    return k.clone();
                }
            }
        }

        map.insert(str, i);
    }

    line.iter().collect()
}

fn parse_input(file_path: &str) -> Vec<Action> {
    let mut res = vec![];
    let file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).unwrap();
    let split = buffer.trim().split(",");
    for seq in split {
        if let Some(action) = Action::new(seq) {
            res.push(action);
        }
    }
    res
}

impl Action {
    pub fn new(str: &str) -> Option<Self> {
        let c = str.chars().nth(0)?;
        match c {
            's' => Some(Self::Spin(str[1..].parse::<usize>().ok()?)),
            'x' => {
                let split: Vec<&str> = str[1..].split('/').collect();
                Some(Self::Exchange(
                    split[0].parse().ok()?,
                    split[1].parse().ok()?,
                ))
            }
            'p' => Some(Self::Partner(str.chars().nth(1)?, str.chars().nth(3)?)),
            _ => None,
        }
    }

    pub fn run(&self, positions: [char; SIZE]) -> [char; SIZE] {
        let mut updated = positions.clone();
        match self {
            Action::Spin(n) => {
                // let shift = SIZE - n - 1;
                for i in 0..SIZE {
                    updated[(i + n) % SIZE] = positions[i];
                }
                updated
            }
            Action::Exchange(a, b) => {
                updated[*a] = positions[*b];
                updated[*b] = positions[*a];
                updated
            }
            Action::Partner(a, b) => {
                let index1 = positions.iter().position(|c| c == a).unwrap();
                let index2 = positions.iter().position(|c| c == b).unwrap();
                updated[index1] = positions[index2];
                updated[index2] = positions[index1];
                updated
            }
        }
    }
}
