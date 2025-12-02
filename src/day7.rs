use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone)]
pub struct Program {
    pub name: String,
    pub weight: u32,
    pub children: Vec<String>,
}

pub struct ToweredProgram {
    pub name: String,
    pub weight: u32,
    pub children: Vec<Box<ToweredProgram>>,
}

pub fn part1(file_path: &str) -> String {
    let programs = parse_input(file_path);
    'outer: for i in 0..programs.len() {
        for j in 0..programs.len() {
            if i == j {
                continue;
            }
            for child in programs[j].children.iter() {
                if &programs[i].name == child {
                    continue 'outer;
                }
            }
        }

        return programs[i].name.clone();
    }

    return "Not found".to_owned();
}

pub fn to_towered_program(
    program: String,
    programs: Vec<Program>,
) -> (ToweredProgram, Vec<Program>) {
    let mut programs = programs;

    let mut i = 0;
    while programs[i].name != program {
        i += 1;
    }

    let prog = programs[i].clone();
    programs.remove(i);

    let mut res = ToweredProgram::new(program, prog.weight);
    for child in prog.children {
        let call = to_towered_program(child, programs.clone());
        res.children.push(Box::new(call.0));
        programs = call.1;
    }

    return (res, programs);
}

fn parse_input(file_path: &str) -> Vec<Program> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut result = vec![];

    for line in reader.lines() {
        if let Ok(line) = line {
            let line = line.trim().to_owned();
            if line.len() == 0 {
                continue;
            }
            if let Some(program) = Program::from_string(line) {
                result.push(program);
            }
        }
    }

    result
}

impl ToweredProgram {
    pub fn new(name: String, weight: u32) -> Self {
        Self {
            name: name,
            weight: weight,
            children: vec![],
        }
    }

    pub fn get_total_weight(&self) -> u32 {
        return self
            .children
            .iter()
            .map(|child| child.get_total_weight())
            .sum();
    }
}

impl Program {
    pub fn from_string(line: String) -> Option<Self> {
        if line.contains("->") {
            let split: Vec<String> = line.split("->").map(|s| s.to_owned()).collect();
            return Some(Self {
                children: Self::get_children(split[1].clone())?,
                ..Self::parse_base(split[0].clone())?
            });
        }

        Self::parse_base(line)
    }

    fn parse_base(content: String) -> Option<Self> {
        let content = content.trim().replace("(", "").replace(")", "");
        let split: Vec<&str> = content.split(" ").collect();
        if split.len() != 2 {
            return None;
        }

        Some(Self {
            name: split[0].to_owned(),
            weight: split[1].parse().ok()?,
            children: vec![],
        })
    }

    fn get_children(content: String) -> Option<Vec<String>> {
        let content = content.trim();
        let split: Vec<String> = content.split(", ").map(|s| s.to_owned()).collect();
        if split.len() == 0 {
            None
        } else {
            Some(split)
        }
    }
}
