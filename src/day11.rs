use std::{
    fs::File,
    io::{BufReader, Read},
};

pub enum Direction {
    North,
    South,
    NorthEast,
    SouthEast,
    NorthWest,
    SouthWest,
}

pub fn part1(file_path: &str) -> f64 {
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let directions = parse_input(file_path);

    for direction in directions {
        match direction {
            Direction::North => y -= 1.0,
            Direction::South => y += 1.0,
            Direction::NorthEast => {
                y -= 0.5;
                x += 1.0;
            }
            Direction::SouthEast => {
                y += 0.5;
                x += 1.0;
            }
            Direction::NorthWest => {
                y -= 0.5;
                x -= 1.0;
            }
            Direction::SouthWest => {
                y += 0.5;
                x -= 1.0;
            }
        }
    }

    get_number_of_steps(x, y)
}

pub fn part2(file_path: &str) -> f64 {
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let mut positions = vec![];
    let directions = parse_input(file_path);

    for direction in directions {
        match direction {
            Direction::North => y -= 1.0,
            Direction::South => y += 1.0,
            Direction::NorthEast => {
                y -= 0.5;
                x += 1.0;
            }
            Direction::SouthEast => {
                y += 0.5;
                x += 1.0;
            }
            Direction::NorthWest => {
                y -= 0.5;
                x -= 1.0;
            }
            Direction::SouthWest => {
                y += 0.5;
                x -= 1.0;
            }
        }

        positions.push(get_number_of_steps(x, y));
    }

    let mut max = 0.0;
    for position in positions {
        if position > max {
            max = position;
        }
    }

    max
}

fn get_number_of_steps(x: f64, y: f64) -> f64 {
    let x = x.abs();
    let y = y.abs();

    let up = y - x * 0.5;

    x + up
}

fn parse_input(file_path: &str) -> Vec<Direction> {
    let file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(file);
    let mut res = vec![];
    let mut str = String::new();
    reader.read_to_string(&mut str).unwrap();
    let split = str.trim().split(",");
    for s in split {
        if let Some(dir) = Direction::new(s) {
            res.push(dir);
        }
    }
    res
}

impl Direction {
    pub fn new(str: &str) -> Option<Self> {
        match str {
            "n" => Some(Self::North),
            "ne" => Some(Self::NorthEast),
            "nw" => Some(Self::NorthWest),
            "s" => Some(Self::South),
            "se" => Some(Self::SouthEast),
            "sw" => Some(Self::SouthWest),
            _ => None,
        }
    }
}
