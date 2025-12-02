use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

pub struct Program {
    cursor: i128,
    last_played_sound: i128,
    registers: HashMap<char, i128>,
    instructions: Vec<Instruction>,
}

pub struct Program2 {
    cursor1: i128,
    cursor2: i128,
    total_send_p1: i128,
    registers1: HashMap<char, i128>,
    registers2: HashMap<char, i128>,
    instructions: Vec<Instruction>,
    queue1: VecDeque<i128>,
    queue2: VecDeque<i128>,
}

pub enum Value {
    Register(char),
    Literal(i128),
}

pub enum Instruction {
    Snd(Value),
    Set(char, Value),
    Add(char, Value),
    Mul(char, Value),
    Mod(char, Value),
    Rcv(Value),
    Jgz(Value, Value),
}

pub fn part1(file_path: &str) -> i128 {
    parse_input1(file_path).run()
}

pub fn part2(file_path: &str) -> i128 {
    parse_input2(file_path).run()
}

fn parse_input1(file_path: &str) -> Program {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut instructions = vec![];

    for line in reader.lines() {
        if let Ok(line) = line {
            let line = line.trim();
            if let Some(instruction) = Instruction::new(line) {
                instructions.push(instruction);
            }
        }
    }

    Program::new(instructions)
}

fn parse_input2(file_path: &str) -> Program2 {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut instructions = vec![];

    for line in reader.lines() {
        if let Ok(line) = line {
            let line = line.trim();
            if let Some(instruction) = Instruction::new(line) {
                instructions.push(instruction);
            }
        }
    }

    Program2::new(instructions)
}

impl Program2 {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            cursor1: 0,
            cursor2: 0,
            total_send_p1: 0,
            registers1: HashMap::new(),
            registers2: HashMap::new(),
            instructions,
            queue1: VecDeque::new(),
            queue2: VecDeque::new(),
        }
    }

    pub fn run(mut self) -> i128 {
        while (self.cursor1 >= 0 && self.cursor1 < self.instructions.len() as i128)
            && (self.cursor2 >= 0 && self.cursor2 < self.instructions.len() as i128)
        {
            if !self.run_prog_1() && !self.run_prog_2() {
                break;
            }
        }

        self.total_send_p1
    }

    pub fn run_prog_1(&mut self) -> bool {
        match &self.instructions[self.cursor1 as usize] {
            Instruction::Snd(value) => {
                self.queue2
                    .push_back(value.get_value_or(&self.registers1, 0));
            }
            Instruction::Set(reg, value) => {
                self.registers1
                    .insert(*reg, value.get_value_or(&self.registers1, 0));
            }
            Instruction::Add(reg, value) => {
                self.registers1.insert(
                    *reg,
                    self.registers1.get(reg).unwrap_or(&0)
                        + value.get_value_or(&self.registers1, 0),
                );
            }
            Instruction::Mul(reg, value) => {
                self.registers1.insert(
                    *reg,
                    self.registers1.get(reg).unwrap_or(&0)
                        * value.get_value_or(&self.registers1, 0),
                );
            }
            Instruction::Mod(reg, value) => {
                self.registers1.insert(
                    *reg,
                    self.registers1.get(reg).unwrap_or(&0)
                        % value.get_value_or(&self.registers1, 0),
                );
            }
            Instruction::Rcv(value) => {
                if let Some(val) = self.queue1.pop_front() {
                    self.registers1.insert(value.as_register(), val);
                } else {
                    return false;
                }
            }
            Instruction::Jgz(value, value1) => {
                if value.get_value_or(&self.registers1, 0) > 0 {
                    self.cursor1 += value1.get_value_or(&self.registers1, 0);
                    return true;
                }
            }
        }
        self.cursor1 += 1;
        true
    }

    pub fn run_prog_2(&mut self) -> bool {
        match &self.instructions[self.cursor2 as usize] {
            Instruction::Snd(value) => {
                self.total_send_p1 += 1;
                self.queue1
                    .push_back(value.get_value_or(&self.registers2, 1));
            }
            Instruction::Set(reg, value) => {
                self.registers2
                    .insert(*reg, value.get_value_or(&self.registers2, 1));
            }
            Instruction::Add(reg, value) => {
                self.registers2.insert(
                    *reg,
                    self.registers2.get(reg).unwrap_or(&1)
                        + value.get_value_or(&self.registers2, 1),
                );
            }
            Instruction::Mul(reg, value) => {
                self.registers2.insert(
                    *reg,
                    self.registers2.get(reg).unwrap_or(&1)
                        * value.get_value_or(&self.registers2, 1),
                );
            }
            Instruction::Mod(reg, value) => {
                self.registers2.insert(
                    *reg,
                    self.registers2.get(reg).unwrap_or(&1)
                        % value.get_value_or(&self.registers2, 1),
                );
            }
            Instruction::Rcv(value) => {
                if let Some(val) = self.queue2.pop_front() {
                    self.registers2.insert(value.as_register(), val);
                } else {
                    return false;
                }
            }
            Instruction::Jgz(value, value1) => {
                if value.get_value_or(&self.registers2, 1) > 0 {
                    self.cursor2 += value1.get_value_or(&self.registers2, 1);
                    return true;
                }
            }
        }
        self.cursor2 += 1;
        true
    }
}

impl Program {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            cursor: 0,
            last_played_sound: 0,
            registers: HashMap::new(),
            instructions: instructions,
        }
    }

    pub fn run(mut self) -> i128 {
        while self.cursor >= 0 && self.cursor < self.instructions.len() as i128 {
            match &self.instructions[self.cursor as usize] {
                Instruction::Snd(value) => {
                    self.last_played_sound = value.get_value(&self.registers);
                }
                Instruction::Set(reg, value) => {
                    self.registers
                        .insert(*reg, value.get_value(&self.registers));
                }
                Instruction::Add(reg, value) => {
                    self.registers.insert(
                        *reg,
                        self.registers.get(reg).unwrap_or(&0) + value.get_value(&self.registers),
                    );
                }
                Instruction::Mul(reg, value) => {
                    self.registers.insert(
                        *reg,
                        self.registers.get(reg).unwrap_or(&0) * value.get_value(&self.registers),
                    );
                }
                Instruction::Mod(reg, value) => {
                    self.registers.insert(
                        *reg,
                        self.registers.get(reg).unwrap_or(&0) % value.get_value(&self.registers),
                    );
                }
                Instruction::Rcv(value) => {
                    if value.get_value(&self.registers) != 0 {
                        break;
                    }
                }
                Instruction::Jgz(value, value1) => {
                    if value.get_value(&self.registers) > 0 {
                        self.cursor += value1.get_value(&self.registers);
                        continue;
                    }
                }
            }
            self.cursor += 1;
        }

        self.last_played_sound
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

    pub fn get_value(&self, registers: &HashMap<char, i128>) -> i128 {
        match self {
            Value::Register(reg) => *registers.get(reg).unwrap_or(&0),
            Value::Literal(val) => *val,
        }
    }

    pub fn get_value_or(&self, registers: &HashMap<char, i128>, or: i128) -> i128 {
        match self {
            Value::Register(reg) => *registers.get(reg).unwrap_or(&or),
            Value::Literal(val) => *val,
        }
    }

    pub fn as_register(&self) -> char {
        match self {
            Value::Register(reg) => *reg,
            Value::Literal(_) => panic!("Value is not a register."),
        }
    }
}

fn get_register_name(str: &str) -> Option<char> {
    let c = str.chars().nth(0)?;
    if c.is_ascii_alphabetic() {
        Some(c)
    } else {
        None
    }
}

impl Instruction {
    pub fn new(str: &str) -> Option<Self> {
        let content: Vec<&str> = str.split(" ").collect();

        match content[0] {
            "snd" => {
                if content.len() != 2 {
                    None
                } else {
                    Some(Self::Snd(Value::new(content[1])?))
                }
            }
            "set" => {
                if content.len() != 3 {
                    None
                } else {
                    Some(Self::Set(
                        get_register_name(content[1])?,
                        Value::new(content[2])?,
                    ))
                }
            }
            "add" => {
                if content.len() != 3 {
                    None
                } else {
                    Some(Self::Add(
                        get_register_name(content[1])?,
                        Value::new(content[2])?,
                    ))
                }
            }
            "mul" => {
                if content.len() != 3 {
                    None
                } else {
                    Some(Self::Mul(
                        get_register_name(content[1])?,
                        Value::new(content[2])?,
                    ))
                }
            }
            "mod" => {
                if content.len() != 3 {
                    None
                } else {
                    Some(Self::Mod(
                        get_register_name(content[1])?,
                        Value::new(content[2])?,
                    ))
                }
            }
            "rcv" => {
                if content.len() != 2 {
                    None
                } else {
                    Some(Self::Rcv(Value::new(content[1])?))
                }
            }
            "jgz" => {
                if content.len() != 3 {
                    None
                } else {
                    Some(Self::Jgz(Value::new(content[1])?, Value::new(content[2])?))
                }
            }
            _ => None,
        }
    }
}
