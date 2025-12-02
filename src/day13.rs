use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

pub fn part1(file_path: &str) -> usize {
    let walls = parse_input1(file_path);
    let mut total = 0;
    for i in 0..walls.len() {
        if let Some((drone, depth)) = walls[i] {
            if drone == 0 {
                total += i * depth;
            }
        }
    }

    total
}

pub fn part2(file_path: &str) -> usize {
    let mut i = 0;
    loop {
        if parse_input_2(file_path, i) {
            break i;
        }
        i += 1;
    }
}

pub fn part3(file_path: &str) -> usize {
    let walls = parse_input_3(file_path);
    return solve_rec(0, 0, &walls, usize::MAX, walls.len() * 2, HashMap::new()).1;
}

fn solve_rec(
    time_ellapsed: usize,
    position: usize,
    walls: &Vec<Option<usize>>,
    min: usize,
    max: usize,
    mut memory: HashMap<(usize, usize), usize>,
) -> (HashMap<(usize, usize), usize>, usize) {
    if memory.contains_key(&(position, time_ellapsed)) {
        let v = memory[&(position, time_ellapsed)].clone();
        return (memory, v);
    }

    if time_ellapsed >= max || time_ellapsed >= min {
        memory.insert((position, time_ellapsed), min);

        return (memory, min);
    }

    if position >= walls.len() {
        memory.insert((position, time_ellapsed), time_ellapsed);
        return (memory, time_ellapsed);
    }

    if let Some(robot) = walls[position] {
        let robot_pos = get_position_of_robot(time_ellapsed, robot);
        if robot_pos != 0 {
            let (m, forward) = solve_rec(time_ellapsed + 1, position + 1, walls, min, max, memory);
            memory = m;
            let min = usize::min(min, forward);
            let (m, wait) = solve_rec(time_ellapsed + 1, position, walls, min, max, memory);
            memory = m;
            let min = usize::min(min, wait);
            memory.insert((position, time_ellapsed), min);
            return (memory, min);
        } else {
            let (m, wait) = solve_rec(time_ellapsed + 1, position, walls, min, max, memory);
            let min = usize::min(min, wait);
            memory = m;
            memory.insert((position, time_ellapsed), min);
            return (memory, min);
        }
    } else {
        let (m, forward) = solve_rec(time_ellapsed + 1, position + 1, walls, min, max, memory);
        let min = usize::min(min, forward);
        memory = m;
        let (m, wait) = solve_rec(time_ellapsed + 1, position, walls, min, max, memory);
        let min = usize::min(min, wait);
        memory = m;
        return (memory, min);
    }
}

fn parse_input_3(file_path: &str) -> Vec<Option<usize>> {
    let mut res = vec![];
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            let content: Vec<&str> = line.trim().split(": ").collect();
            if let (Ok(depth), Ok(range)) =
                (content[0].parse::<usize>(), content[1].parse::<usize>())
            {
                while res.len() != depth {
                    res.push(None);
                }
                res.push(Some(range));
            }
        }
    }

    res
}

fn parse_input_2(file_path: &str, time_ellapsed: usize) -> bool {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            let content: Vec<&str> = line.trim().split(": ").collect();
            if let (Ok(depth), Ok(range)) =
                (content[0].parse::<usize>(), content[1].parse::<usize>())
            {
                if get_position_of_robot(depth + time_ellapsed, range) == 0 {
                    return false;
                }
            }
        }
    }

    true
}

fn parse_input1(file_path: &str) -> Vec<Option<(usize, usize)>> {
    let mut res = vec![];
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            let content: Vec<&str> = line.trim().split(": ").collect();
            if let (Ok(depth), Ok(range)) =
                (content[0].parse::<usize>(), content[1].parse::<usize>())
            {
                while res.len() != depth {
                    res.push(None);
                }

                res.push(Some((get_position_of_robot(depth, range), range)));
            }
        }
    }

    res
}

fn get_position_of_robot(time_ellapsed: usize, range: usize) -> usize {
    let walk = (range - 1) * 2;
    let remaining_steps = time_ellapsed % walk;
    let half = walk / 2;
    if remaining_steps > half {
        half - (remaining_steps - half)
    } else {
        remaining_steps
    }
}
