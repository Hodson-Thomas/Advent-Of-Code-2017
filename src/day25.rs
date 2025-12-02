use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};

use indicatif::ProgressBar;

#[derive(Debug)]
pub struct State {
    pub name: String,
    pub write_if_0: bool,
    pub write_if_1: bool,
    pub shift_cursor_if_0: i32,
    pub shift_cursor_if_1: i32,
    pub next_state_if_0: String,
    pub next_state_if_1: String,
}

#[derive(Debug)]
pub struct Machine {
    pub tape: HashMap<i32, bool>,
    pub cursor: i32,
    pub states: Vec<State>,
    pub current_state: String,
    pub steps_to_perform: usize,
}

pub fn part1(file_path: &str) -> usize {
    println!("");
    let mut machine = parse_input(file_path);

    let bar = ProgressBar::new(machine.steps_to_perform as u64);

    for _ in 0..machine.steps_to_perform {
        bar.inc(1);
        machine.run();
    }

    bar.finish();
    println!("");

    machine
        .tape
        .iter()
        .map(|(_, cell)| if *cell { 1 } else { 0 })
        .sum()
}

fn parse_input(file_path: &str) -> Machine {
    let file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).unwrap();
    let buffer = buffer.replace('\r', "");
    let states: Vec<&str> = buffer.trim().split("\n\n").collect();

    let first: Vec<&str> = states[0].split('\n').collect();

    Machine::new(
        states
            .into_iter()
            .skip(1)
            .map(|s| State::new(s))
            .filter(|e| e.is_some())
            .map(|e| e.unwrap())
            .collect(),
        first[0]
            .split(' ')
            .last()
            .unwrap()
            .replace('.', "")
            .to_owned(),
        first[1].split(' ').collect::<Vec<&str>>()[5]
            .parse()
            .unwrap(),
    )
}

impl Machine {
    pub fn new(states: Vec<State>, start_state: String, steps_to_perform: usize) -> Self {
        let mut map = HashMap::new();
        map.insert(0, false);
        Self {
            states,
            tape: map,
            cursor: 0,
            current_state: start_state,
            steps_to_perform,
        }
    }

    pub fn run(&mut self) {
        for state in self.states.iter() {
            if state.name == self.current_state {
                let shift = if !*self.tape.get(&self.cursor).unwrap_or(&false) {
                    self.current_state = state.next_state_if_0.clone();
                    self.tape.insert(self.cursor, state.write_if_0);
                    state.shift_cursor_if_0
                } else {
                    self.current_state = state.next_state_if_1.clone();
                    self.tape.insert(self.cursor, state.write_if_1);
                    state.shift_cursor_if_1
                };

                self.cursor += shift;
                self.tape
                    .insert(self.cursor, *self.tape.get(&self.cursor).unwrap_or(&false));

                return;
            }
        }

        panic!("Could not find state")
    }
}

impl State {
    pub fn new(str: &str) -> Option<Self> {
        let content: Vec<&str> = str.split('\n').collect();
        if content.len() != 9 {
            return None;
        }

        Some(Self {
            name: content[0].split(' ').last()?.replace(':', "").to_owned(),
            write_if_0: content[2].split(' ').last()?.replace('.', "") == "1",
            write_if_1: content[6].split(' ').last()?.replace('.', "") == "1",
            shift_cursor_if_0: if content[3].split(' ').last()?.replace('.', "") == "left" {
                -1
            } else {
                1
            },
            shift_cursor_if_1: if content[7].split(' ').last()?.replace('.', "") == "left" {
                -1
            } else {
                1
            },
            next_state_if_0: content[4].split(' ').last()?.replace('.', "").to_owned(),
            next_state_if_1: content[8].split(' ').last()?.replace('.', "").to_owned(),
        })
    }
}
