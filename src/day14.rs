use std::collections::{HashSet, VecDeque};

const KEY: &str = "oundnydw";
const BLOCK_SIZE: usize = 16;

pub fn part1() -> usize {
    let mut counts = 0;
    for i in 0..128 {
        let hash = knot_hash(&format!("{KEY}-{i}"));
        counts += hash
            .chars()
            .map(|c| hexa_char_to_bools(c))
            .map(|bools| bools.iter().filter(|b| **b == true).count())
            .sum::<usize>()
    }
    counts
}

pub fn part2() -> usize {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut regions_count = 0;
    let mut board = vec![];
    for i in 0..128 {
        let hash = knot_hash(&format!("{KEY}-{i}"));
        board.push(vec![]);
        for c in hash.chars() {
            board[i].extend(hexa_char_to_bools(c));
        }
    }

    for row in 0..board.len() {
        for col in 0..board[row].len() {
            if visited.contains(&(row, col)) {
                continue;
            }
            let region = create_regions((row, col), &board, HashSet::new());
            if region.len() == 0 {
                continue;
            }
            regions_count += 1;
            for tile in region {
                visited.insert(tile);
            }
        }
    }

    regions_count
}

fn create_regions(
    position: (usize, usize),
    board: &Vec<Vec<bool>>,
    mut region: HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let (row, col) = position;
    if row >= board.len() || col >= board[row].len() {
        return region;
    }

    if !board[row][col] {
        return region;
    }

    if region.contains(&position) {
        return region;
    }

    region.insert(position);
    if row > 0 {
        region = create_regions((row - 1, col), board, region);
    }
    region = create_regions((row + 1, col), board, region);
    if col > 0 {
        region = create_regions((row, col - 1), board, region);
    }
    create_regions((row, col + 1), board, region)
}

fn hexa_char_to_bools(c: char) -> [bool; 4] {
    match c.to_ascii_lowercase() {
        '0' => [false; 4],
        '1' => [false, false, false, true],
        '2' => [false, false, true, false],
        '3' => [false, false, true, true],
        '4' => [false, true, false, false],
        '5' => [false, true, false, true],
        '6' => [false, true, true, false],
        '7' => [false, true, true, true],
        '8' => [true, false, false, false],
        '9' => [true, false, false, true],
        'a' => [true, false, true, false],
        'b' => [true, false, true, true],
        'c' => [true, true, false, false],
        'd' => [true, true, false, true],
        'e' => [true, true, true, false],
        'f' => [true, true, true, true],
        _ => panic!("Non hexa char"),
    }
}

fn knot_hash(text: &str) -> String {
    let mut codes: Vec<u8> = text.chars().into_iter().map(|c| c as u8).collect();
    codes.extend(vec![17, 31, 73, 47, 23]);
    let numbers = logic(codes, 64);
    let mut res = String::new();

    for i in 0..16 {
        let mut temp = numbers[i * BLOCK_SIZE];
        for j in 1..BLOCK_SIZE {
            temp = temp ^ numbers[i * BLOCK_SIZE + j];
        }
        res = format!("{res}{:02x}", temp);
    }

    res
}

fn logic(sizes: Vec<u8>, iterations: usize) -> Vec<u8> {
    let mut circle = VecDeque::from_iter(0..=255 as u8);
    let mut skip = 0;
    for _ in 0..iterations {
        for group_size in sizes.iter() {
            let mut knot: Vec<u8> = (0..*group_size)
                .into_iter()
                .map(|_| circle.pop_front().unwrap())
                .collect();
            knot.reverse();
            circle.extend(knot);
            circle.rotate_left(skip % circle.len());
            skip += 1;
        }
    }

    let unwind =
        iterations * sizes.iter().map(|v| *v as usize).sum::<usize>() + skip * (skip - 1) / 2;
    circle.rotate_right(unwind % circle.len());
    return circle.into_iter().collect();
}
