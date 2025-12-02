use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub enum Value {
    Literal(i32),
    Register(char),
}

pub enum Instruction {
    Set(char, Value),
    Sub(char, Value),
    Mul(char, Value),
    Jnz(Value, Value),
}

pub struct Program {
    instructions: Vec<Instruction>,
    registers: HashMap<char, i32>,
    mul_calls: usize,
    cursor: i32,
}

pub fn part1(file_path: &str) -> usize {
    parse_input(file_path).part1()
}

fn parse_input(file_path: &str) -> Program {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut instructions = vec![];
    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some(instruction) = Instruction::new(line.trim()) {
                instructions.push(instruction);
            }
        }
    }

    Program::new(instructions)
}

fn get_register_name(str: &str) -> Option<char> {
    let c = str.trim().chars().nth(0)?;
    if c.is_ascii_alphabetic() {
        Some(c)
    } else {
        None
    }
}

impl Program {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            cursor: 0,
            mul_calls: 0,
            registers: HashMap::new(),
        }
    }

    pub fn part1(mut self) -> usize {
        while self.cursor >= 0 && self.cursor < self.instructions.len() as i32 {
            match &self.instructions[self.cursor as usize] {
                Instruction::Set(reg, value) => {
                    self.registers
                        .insert(*reg, value.get_value(&self.registers));
                }
                Instruction::Sub(reg, value) => {
                    self.registers.insert(
                        *reg,
                        self.registers.get(reg).unwrap_or(&0) - value.get_value(&self.registers),
                    );
                }
                Instruction::Mul(reg, value) => {
                    self.mul_calls += 1;
                    self.registers.insert(
                        *reg,
                        self.registers.get(reg).unwrap_or(&0) * value.get_value(&self.registers),
                    );
                }
                Instruction::Jnz(value, value1) => {
                    if value.get_value(&self.registers) != 0 {
                        self.cursor += value1.get_value(&self.registers);
                        continue;
                    }
                }
            }

            self.cursor += 1;
        }
        dbg!(self.registers.get(&'h'));
        self.mul_calls
    }
}

impl Value {
    pub fn new(str: &str) -> Option<Self> {
        if let Ok(literal) = str.parse() {
            Some(Self::Literal(literal))
        } else {
            Some(Self::Register(get_register_name(str)?))
        }
    }

    pub fn get_value(&self, registers: &HashMap<char, i32>) -> i32 {
        match self {
            Value::Literal(literal) => *literal,
            Value::Register(reg) => *registers.get(reg).unwrap_or(&0),
        }
    }
}

impl Instruction {
    pub fn new(str: &str) -> Option<Self> {
        let content: Vec<&str> = str.trim().split(" ").collect();
        if content.len() != 3 {
            return None;
        }
        match content[0] {
            "set" => Some(Self::Set(
                get_register_name(content[1])?,
                Value::new(content[2])?,
            )),
            "sub" => Some(Self::Sub(
                get_register_name(content[1])?,
                Value::new(content[2])?,
            )),
            "mul" => Some(Self::Mul(
                get_register_name(content[1])?,
                Value::new(content[2])?,
            )),
            "jnz" => Some(Self::Jnz(Value::new(content[1])?, Value::new(content[2])?)),
            _ => None,
        }
    }
}
