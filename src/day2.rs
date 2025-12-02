use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part1(file_path: String) -> i32 {
    let data = parse_input(file_path);
    data.iter()
        .map(|row| row.iter().max().unwrap() - row.iter().min().unwrap())
        .sum()
}

pub fn part2(file_path: String) -> i32 {
    let data = parse_input(file_path);
    let mut sum = 0;
    for row in data {
        for i in 0..(row.len() - 1) {
            for j in (i + 1)..row.len() {
                let max = i32::max(row[i], row[j]);
                let min = i32::min(row[i], row[j]);
                if max % min == 0 {
                    sum += max / min;
                    break;
                }
            }
        }
    }
    sum
}

fn parse_input(file_path: String) -> Vec<Vec<i32>> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut result: Vec<Vec<i32>> = vec![];
    for line in reader.lines() {
        if let Ok(line) = line {
            result.push(
                line.trim()
                    .split("\t")
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
            );
        }
    }
    result
}
