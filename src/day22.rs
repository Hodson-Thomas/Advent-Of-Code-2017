use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

const BURSTS1: usize = 10_000;
const BURSTS2: usize = 10_000_000;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum CellState {
    Weakened,
    Infected,
    Flagged,
}

pub fn part1(file_path: &str) -> i32 {
    let input = parse_input1(file_path);
    let mut infected = input.0;
    let mut infections = 0;
    let mut position = input.1;
    let mut direction = Direction::Up;

    for _ in 0..BURSTS1 {
        if infected.contains(&position) {
            let index = infected.iter().position(|p| *p == position).unwrap();
            infected.remove(index);
            direction = direction.turn_right();
        } else {
            infected.push(position);
            infections += 1;
            direction = direction.turn_left();
        }
        position = direction.walk(position);
    }

    infections
}

pub fn part2(file_path: &str) -> i32 {
    let input = parse_input2(file_path);
    let mut infections = 0;
    let mut cells = input.0;
    let mut position = input.1;
    let mut direction = Direction::Up;
    for _ in 0..BURSTS2 {
        if cells.contains_key(&position) {
            match cells[&position] {
                CellState::Weakened => {
                    cells.insert(position, CellState::Infected);
                    infections += 1;
                }
                CellState::Infected => {
                    cells.insert(position, CellState::Flagged);
                    direction = direction.turn_right();
                }
                CellState::Flagged => {
                    cells.remove(&position);
                    direction = direction.turn_back();
                }
            }
        } else {
            cells.insert(position, CellState::Weakened);
            direction = direction.turn_left();
        }

        position = direction.walk(position);
    }

    infections
}

fn parse_input1(file_path: &str) -> (Vec<(i32, i32)>, (i32, i32)) {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut res = vec![];
    let mut position: (i32, i32) = (0, 0);

    let lines: Vec<Result<String, std::io::Error>> = reader.lines().collect();

    for (y, line) in lines.iter().enumerate() {
        if let Ok(line) = line {
            if position == (0, 0) {
                position = ((line.len() / 2) as i32, (lines.len() / 2) as i32);
            }
            for (x, c) in line.trim().chars().enumerate() {
                if c == '#' {
                    res.push((x as i32, y as i32));
                }
            }
        }
    }

    (res, position)
}

fn parse_input2(file_path: &str) -> (HashMap<(i32, i32), CellState>, (i32, i32)) {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut res = HashMap::new();
    let mut position: (i32, i32) = (0, 0);

    let lines: Vec<Result<String, std::io::Error>> = reader.lines().collect();

    for (y, line) in lines.iter().enumerate() {
        if let Ok(line) = line {
            if position == (0, 0) {
                position = ((line.len() / 2) as i32, (lines.len() / 2) as i32);
            }
            for (x, c) in line.trim().chars().enumerate() {
                if c == '#' {
                    res.insert((x as i32, y as i32), CellState::Infected);
                }
            }
        }
    }

    (res, position)
}

impl Direction {
    pub fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Self::Left,
            Direction::Down => Self::Right,
            Direction::Left => Self::Down,
            Direction::Right => Self::Up,
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Self::Right,
            Direction::Down => Self::Left,
            Direction::Left => Self::Up,
            Direction::Right => Self::Down,
        }
    }

    pub fn turn_back(&self) -> Self {
        match self {
            Direction::Up => Self::Down,
            Direction::Down => Self::Up,
            Direction::Left => Self::Right,
            Direction::Right => Self::Left,
        }
    }

    pub fn walk(&self, current_position: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => (current_position.0, current_position.1 - 1),
            Direction::Down => (current_position.0, current_position.1 + 1),
            Direction::Left => (current_position.0 - 1, current_position.1),
            Direction::Right => (current_position.0 + 1, current_position.1),
        }
    }
}
