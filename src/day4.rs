use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part1(file_path: &str) -> u32 {
    let passphrases = parse_input(file_path);
    let mut count = 0;

    for passphrase in passphrases {
        let words: Vec<&str> = passphrase.split(" ").collect();
        let words_set: HashSet<&str> = words.clone().into_iter().collect();
        if words.len() == words_set.len() {
            count += 1;
        }
    }

    count
}

pub fn part2(file_path: &str) -> u32 {
    let passphrases = parse_input(file_path);
    let mut count = 0;

    for passphrase in passphrases {
        let words: Vec<&str> = passphrase.split(" ").collect();
        let elements: Vec<HashMap<char, u32>> = words.iter().map(|s| word_to_char_map(s)).collect();
        let mut valid = true;
        'outer: for i in 0..(elements.len() - 1) {
            for j in (i + 1)..elements.len() {
                if are_maps_equal(&elements[i], &elements[j]) {
                    valid = false;
                    break 'outer;
                }
            }
        }

        if valid {
            count += 1;
        }
    }

    count
}

pub fn word_to_char_map(word: &str) -> HashMap<char, u32> {
    let mut map = HashMap::new();
    for c in word.chars() {
        map.insert(c, map.get(&c).unwrap_or(&0) + 1);
    }
    map
}

pub fn are_maps_equal(map1: &HashMap<char, u32>, map2: &HashMap<char, u32>) -> bool {
    if map1.keys().len() != map2.keys().len() {
        return false;
    }

    let keys1 = map1.keys();

    for k in keys1 {
        if !map2.contains_key(k) {
            return false;
        }

        if map1[k] != map2[k] {
            return false;
        }
    }

    true
}

pub fn parse_input(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap().trim().to_owned());
    }
    lines
}
