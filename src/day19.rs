use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Maze {
    pub board: Vec<Vec<Tile>>,
    pub position: (usize, usize),
    pub word: String,
    pub direction: Direction,
    pub steps: usize,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum Tile {
    Vertical,
    Cross,
    Horizontal,
    Empty,
    Char(char),
}

pub fn part1(file_path: &str) -> String {
    let mut board = parse_input(file_path).unwrap();
    board.walk();
    board.word
}

pub fn part2(file_path: &str) -> usize {
    let mut board = parse_input(file_path).unwrap();
    board.walk();
    board.steps
}

fn parse_input(file_path: &str) -> Option<Maze> {
    let mut board = vec![];
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            let mut row = vec![];
            for c in line.chars() {
                if let Some(tile) = Tile::new(c) {
                    row.push(tile);
                }
            }
            board.push(row);
        }
    }

    for c in 0..board[0].len() {
        if board[0][c].is_path() {
            return Some(Maze::new(board, (c, 0)));
        }
    }

    return None;
}

impl Maze {
    pub fn new(board: Vec<Vec<Tile>>, position: (usize, usize)) -> Self {
        Self {
            board,
            position,
            word: String::new(),
            direction: Direction::Down,
            steps: 0,
        }
    }

    pub fn walk(&mut self) {
        loop {
            let (x, y) = self.position;
            match self.board[y][x] {
                Tile::Empty => return,
                Tile::Cross => self.turn_at_cross(),
                Tile::Char(c) => {
                    self.word.push_str(&c.to_string());
                    self.position = self.direction.walk(self.position);
                }
                _ => self.position = self.direction.walk(self.position),
            }
            self.steps += 1
        }
    }

    fn turn_at_cross(&mut self) {
        let (x, y) = self.position;
        match self.direction {
            Direction::Up | Direction::Down => {
                if x != 0 && self.board[y][x - 1].is_path() {
                    self.direction = Direction::Left;
                    self.position = (x - 1, y);
                    return;
                } else if x < self.board[y].len() - 1 && self.board[y][x + 1].is_path() {
                    self.direction = Direction::Right;
                    self.position = (x + 1, y);
                } else {
                    panic!()
                }
            }
            Direction::Left | Direction::Right => {
                if y != 0 && self.board[y - 1][x].is_path() {
                    self.direction = Direction::Up;
                    self.position = (x, y - 1);
                    return;
                } else if y < self.board.len() - 1 && self.board[y + 1][x].is_path() {
                    self.direction = Direction::Down;
                    self.position = (x, y + 1);
                } else {
                    panic!()
                }
            }
        }
    }
}

impl Direction {
    pub fn walk(&self, position: (usize, usize)) -> (usize, usize) {
        let (x, y) = position;
        match self {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        }
    }
}

impl Tile {
    pub fn new(c: char) -> Option<Self> {
        match c {
            '|' => Some(Self::Vertical),
            '-' => Some(Self::Horizontal),
            '+' => Some(Self::Cross),
            ' ' => Some(Self::Empty),
            _ => {
                if c.is_alphabetic() {
                    Some(Self::Char(c))
                } else {
                    None
                }
            }
        }
    }

    pub fn is_path(&self) -> bool {
        match self {
            Self::Empty => false,
            _ => true,
        }
    }
}
