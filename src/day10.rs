use std::{
    fs::File,
    io::{BufReader, Read},
};

const SIZE: usize = 256;
const ROUNDS: usize = 64;

pub fn part1(file_path: &str) -> usize {
    let lengths = parse_input1(file_path);
    let mut position = 0;
    let mut skip_size = 0;
    let mut array = [0; SIZE];
    for i in 0..array.len() {
        array[i] = i;
    }

    for length in lengths {
        array = transform(&array, length, position);
        position = position + length + skip_size;
        skip_size += 1;
    }

    return array[0] * array[1];
}

pub fn part2(file_path: &str) -> String {
    let lengths = parse_input2(file_path);
    let mut position = 0;
    let mut skip_size = 0;
    let mut array = [0; SIZE];

    for i in 0..array.len() {
        array[i] = i;
    }

    for _ in 0..ROUNDS {
        for length in lengths.iter() {
            array = transform(&array, *length, position);
            position = position + length + skip_size;
            skip_size += 1;
        }
    }

    let dense = sparse_to_dense(array);
    let mut str = String::new();
    for val in dense {
        str = format!("{}{:02x}", str, val);
    }
    str
}

fn sparse_to_dense(array: [usize; 256]) -> [u8; 16] {
    let mut res = [0; 16];
    for i in 0..16 {
        let mut temp = array[16 * i];
        for j in 1..16 {
            temp = temp ^ array[16 * i + j];
        }
        res[i] = temp as u8;
    }
    res
}

fn transform(array: &[usize; SIZE], length: usize, position: usize) -> [usize; SIZE] {
    let mut res = array.clone();

    for i in 0..length {
        res[(position + i) % res.len()] = array[(position + (length - 1) - i) % res.len()];
        res[(position + (length - 1) - i) % res.len()] = array[(position + i) % res.len()];
    }
    res
}

fn parse_input1(file_path: &str) -> Vec<usize> {
    let file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(file);
    let mut str = String::new();
    reader.read_to_string(&mut str).unwrap();
    let content = str.trim().split(",");
    let mut res = vec![];
    for s in content {
        if let Ok(val) = s.parse::<usize>() {
            res.push(val);
        }
    }
    res
}

fn parse_input2(file_path: &str) -> Vec<usize> {
    let file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(file);
    let mut str = String::new();
    reader.read_to_string(&mut str).unwrap();
    let mut lengths: Vec<usize> = str
        .chars()
        .into_iter()
        .map(|c| (c as u8) as usize)
        .collect();

    lengths.append(&mut vec![17, 31, 73, 47, 23]);
    lengths
}
