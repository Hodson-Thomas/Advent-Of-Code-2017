use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part1(file_path: &str) -> usize {
    let mut set: HashSet<usize> = HashSet::new();
    let mut data = parse_input(file_path);

    let mut changed: bool = true;
    loop {
        if !changed {
            break;
        }

        changed = false;

        let d = data.clone();
        let keys = d.keys();
        for key in keys {
            if *key == 0 || set.contains(key) {
                set.extend(data[key].iter());
                data.remove(key);
                changed = true;
            }
        }
    }

    set.len()
}

pub fn part2(file_path: &str) -> usize {
    let mut set: HashSet<usize> = HashSet::new();
    let mut target;
    let mut data = parse_input(file_path);
    let mut changed: bool;
    let mut count = 0;

    loop {
        let d = data.clone();
        if d.keys().len() == 0 {
            break;
        }

        target = d.keys().nth(0).unwrap();
        changed = true;
        set.clear();
        loop {
            if !changed {
                break;
            }

            changed = false;
            let d = data.clone();
            let keys = d.keys();
            for key in keys {
                if key == target || set.contains(key) {
                    set.extend(data[key].iter());
                    data.remove(key);
                    changed = true;
                }
            }
        }
        count += 1;
    }

    count
}

fn parse_input(file_path: &str) -> HashMap<usize, Vec<usize>> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut map = HashMap::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let content: Vec<&str> = line.trim().split(" <-> ").collect();
            if content.len() != 2 {
                continue;
            }

            if let Ok(val) = content[0].parse::<usize>() {
                map.insert(val, vec![]);

                for value in content[1].split(", ") {
                    if let Ok(v) = value.parse() {
                        map.get_mut(&val).unwrap().push(v);
                    }
                }
            }
        }
    }

    map
}
