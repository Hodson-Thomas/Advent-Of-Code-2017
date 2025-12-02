use std::{
    collections::HashMap,
    fs::File,
    i32,
    io::{BufRead, BufReader},
};

struct Operation {
    register: String,
    operation_type: OperationType,
    increment: i32,
    left_condition: String,
    condition_type: Condition,
    right_condition: i32,
}

pub enum OperationType {
    Increment,
    Decrement,
}

pub enum Condition {
    Le,
    Leq,
    Ge,
    Geq,
    Eq,
    Neq,
}

pub fn part1(file_path: &str) -> i32 {
    let mut registers = HashMap::<String, i32>::new();
    let operations = parse_input(file_path);
    for op in operations {
        registers = op.evaluate(registers);
    }

    let mut max = i32::MIN;
    for (_, value) in registers.iter() {
        if max < *value {
            max = *value;
        }
    }

    max
}

pub fn part2(file_path: &str) -> i32 {
    let mut registers = HashMap::<String, i32>::new();
    let operations = parse_input(file_path);
    let mut max = i32::MIN;

    for op in operations {
        registers = op.evaluate(registers);
        for (_, value) in registers.iter() {
            if max < *value {
                max = *value;
            }
        }
    }

    max
}

fn parse_input(file_path: &str) -> Vec<Operation> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut res = vec![];
    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some(op) = Operation::new(line) {
                res.push(op);
            }
        }
    }
    res
}

impl Operation {
    pub fn new(line: String) -> Option<Self> {
        let split: Vec<&str> = line.trim().split(" ").collect();
        if split.len() != 7 {
            return None;
        }
        Some(Self {
            register: split[0].to_string(),
            operation_type: OperationType::new(split[1])?,
            increment: split[2].parse().ok()?,
            left_condition: split[4].to_string(),
            condition_type: Condition::new(split[5])?,
            right_condition: split[6].parse().ok()?,
        })
    }

    pub fn evaluate(&self, mut registers: HashMap<String, i32>) -> HashMap<String, i32> {
        let left = *registers.get(&self.left_condition).unwrap_or(&0);
        let right = self.right_condition;

        if !self.condition_type.eval(left, right) {
            return registers;
        }

        let value =
            registers.get(&self.register).unwrap_or(&0) + self.operation_type.eval(self.increment);
        registers.insert(self.register.clone(), value);
        registers
    }
}

impl OperationType {
    pub fn new(str: &str) -> Option<Self> {
        match str {
            "inc" => Some(Self::Increment),
            "dec" => Some(Self::Decrement),
            _ => None,
        }
    }

    pub fn eval(&self, value: i32) -> i32 {
        match self {
            OperationType::Increment => value,
            OperationType::Decrement => -value,
        }
    }
}

impl Condition {
    pub fn new(str: &str) -> Option<Self> {
        match str {
            "<" => Some(Self::Le),
            "<=" => Some(Self::Leq),
            ">" => Some(Self::Ge),
            ">=" => Some(Self::Geq),
            "==" => Some(Self::Eq),
            "!=" => Some(Self::Neq),
            _ => None,
        }
    }

    pub fn eval(&self, left: i32, right: i32) -> bool {
        match self {
            Condition::Le => left < right,
            Condition::Leq => left <= right,
            Condition::Ge => left > right,
            Condition::Geq => left >= right,
            Condition::Eq => left == right,
            Condition::Neq => left != right,
        }
    }
}
