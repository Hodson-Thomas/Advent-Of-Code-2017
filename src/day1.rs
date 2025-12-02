use std::fs::File;
use std::io::{BufReader, Read};

pub fn part1(file_path: String) -> i32 {
    let content = parse_input(file_path);
    let mut sum = 0;
    for i in 0..(content.len() - 1) {
        if content[i] == content[i + 1] {
            sum += char_to_int(content[i]).unwrap();
        }
    }

    if content[0] == content[content.len() - 1] {
        sum += char_to_int(content[0]).unwrap();
    }

    sum
}

pub fn part2(file_path: String) -> i32 {
    let content = parse_input(file_path);
    let mut sum = 0;
    let half = content.len() / 2;
    for i in 0..content.len() {
        if content[i] == content[(i + half) % content.len()] {
            sum += char_to_int(content[i]).unwrap();
        }
    }
    sum
}

fn parse_input(file_path: String) -> Vec<char> {
    let file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::<u8>::new();
    reader.read_to_end(&mut buffer).unwrap();
    String::from_utf8(buffer)
        .unwrap()
        .chars()
        .filter(|c| is_digit(c))
        .collect::<Vec<char>>()
}

fn is_digit(c: &char) -> bool {
    match c {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
        _ => false,
    }
}

fn char_to_int(c: char) -> Option<i32> {
    match c {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        _ => None,
    }
}
