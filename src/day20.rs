use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const FRAMES: usize = 500;

#[derive(Debug, Clone)]
pub struct Particle {
    pub position: (i64, i64, i64),
    pub acceleration: (i64, i64, i64),
    pub velocity: (i64, i64, i64),
}

pub fn part1(file_path: &str) -> usize {
    let mut particles = parse_input(file_path);
    for _ in 0..FRAMES {
        for p in particles.iter_mut() {
            p.render();
        }
    }

    let distances: Vec<i64> = particles.iter().map(|p| p.distance()).collect();
    let mut number = 0;
    let mut min = i64::MAX;
    for i in 0..distances.len() {
        if distances[i] < min {
            number = i;
            min = distances[i];
        }
    }

    number
}

pub fn part2(file_path: &str) -> usize {
    let mut particles = parse_input(file_path);

    for _ in 0..FRAMES {
        for p in particles.iter_mut() {
            p.render();
        }

        let mut temp = vec![];
        'outer: for i in 0..particles.len() {
            'inner: for j in 0..particles.len() {
                if i == j {
                    continue 'inner;
                }

                if particles[i].collides(&particles[j]) {
                    continue 'outer;
                }
            }
            temp.push(particles[i].clone());
        }

        particles = temp;
    }

    particles.len()
}

fn parse_input(file_path: &str) -> Vec<Particle> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut res = vec![];
    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some(particle) = Particle::new(&line) {
                res.push(particle);
            }
        }
    }
    res
}

fn parse_coordinate(str: &str) -> Option<(i64, i64, i64)> {
    let str = str.trim().replace(">", "");
    let content: Vec<&str> = str.split('<').collect();
    if content.len() != 2 {
        return None;
    }

    let content: Vec<&str> = content[1].trim().split(',').collect();
    if content.len() != 3 {
        return None;
    }

    if let (Ok(x), Ok(y), Ok(z)) = (content[0].parse(), content[1].parse(), content[2].parse()) {
        Some((x, y, z))
    } else {
        None
    }
}

impl Particle {
    pub fn new(str: &str) -> Option<Self> {
        let content: Vec<&str> = str.trim().split(", ").collect();
        if content.len() != 3 {
            return None;
        }

        Some(Self {
            position: parse_coordinate(content[0])?,
            velocity: parse_coordinate(content[1])?,
            acceleration: parse_coordinate(content[2])?,
        })
    }

    pub fn distance(&self) -> i64 {
        i64::abs(self.position.0) + i64::abs(self.position.1) + i64::abs(self.position.2)
    }

    pub fn render(&mut self) {
        self.velocity = (
            self.velocity.0 + self.acceleration.0,
            self.velocity.1 + self.acceleration.1,
            self.velocity.2 + self.acceleration.2,
        );
        self.position = (
            self.position.0 + self.velocity.0,
            self.position.1 + self.velocity.1,
            self.position.2 + self.velocity.2,
        );
    }

    pub fn collides(&self, particle: &Particle) -> bool {
        self.position.0 == particle.position.0
            && self.position.1 == particle.position.1
            && self.position.2 == particle.position.2
    }
}
