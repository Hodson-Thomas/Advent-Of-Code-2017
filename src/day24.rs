use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part1(file_path: &str) -> usize {
    rec_solve1(0, 0, parse_input(file_path))
}

pub fn part2(file_path: &str) -> usize {
    rec_solve2(0, 0, 0, parse_input(file_path)).1
}

fn rec_solve1(score: usize, port: usize, remaining: Vec<(usize, usize)>) -> usize {
    let mut max = score;
    for (index, tube) in remaining.iter().enumerate() {
        if tube.0 != port && tube.1 != port {
            continue;
        }

        let new_port = if tube.0 == port { tube.1 } else { tube.0 };
        let mut new_remaining = remaining.clone();
        new_remaining.remove(index);
        max = usize::max(
            max,
            rec_solve1(score + tube.0 + tube.1, new_port, new_remaining),
        )
    }
    max
}

fn rec_solve2(
    strength: usize,
    port: usize,
    length: usize,
    remaining: Vec<(usize, usize)>,
) -> (usize, usize) {
    let mut max_strength = strength;
    let mut max_length = length;

    for (index, tube) in remaining.iter().enumerate() {
        if tube.0 != port && tube.1 != port {
            continue;
        }

        let new_port = if tube.0 == port { tube.1 } else { tube.0 };
        let mut new_remaining = remaining.clone();
        new_remaining.remove(index);

        let (l, s) = rec_solve2(
            strength + tube.0 + tube.1,
            new_port,
            length + 1,
            new_remaining,
        );

        if l > max_length {
            max_length = l;
            max_strength = s;
        } else if l == max_length {
            max_strength = usize::max(max_strength, s);
        }
    }

    (max_length, max_strength)
}

fn parse_input(file_path: &str) -> Vec<(usize, usize)> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut res = vec![];

    for line in reader.lines() {
        if let Ok(line) = line {
            let content: Vec<&str> = line.trim().split("/").collect();
            if content.len() != 2 {
                continue;
            }
            if let (Ok(left), Ok(right)) = (content[0].parse(), content[1].parse()) {
                res.push((left, right));
            }
        }
    }

    res
}
