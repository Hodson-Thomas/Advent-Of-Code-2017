use std::fs::File;
use std::io::{BufReader, Read};

struct Group {
    groups: Vec<Box<Group>>,
    removed_chars: u32,
}

pub fn part1(file_path: &str) -> u32 {
    let group = Group::new(file_path).unwrap();
    group.get_score(1)
}

pub fn part2(file_path: &str) -> u32 {
    let group = Group::new(file_path).unwrap();
    group.count_cleaned_chars()
}

impl Group {
    pub fn new(file_path: &str) -> Option<Self> {
        let file = File::open(file_path).ok()?;
        let mut reader = BufReader::new(file);

        let mut buffer = [0; 1];
        while reader.read(&mut buffer).ok()? > 0 {
            let c = buffer[0] as char;
            if c == '{' {
                let (group, _) = Self::parse_group(reader)?;
                return Some(group);
            }
        }

        None
    }

    pub fn count_cleaned_chars(&self) -> u32 {
        let mut total = 0;
        for group in self.groups.iter() {
            total += group.count_cleaned_chars();
        }
        return total + self.removed_chars;
    }

    pub fn get_score(&self, depth: u32) -> u32 {
        let mut total = depth;
        for group in self.groups.iter() {
            total += group.get_score(depth + 1)
        }
        total
    }

    fn parse_group(mut reader: BufReader<File>) -> Option<(Self, BufReader<File>)> {
        let mut buffer = [0; 1];
        let mut children = vec![];
        let mut count = 0;

        while reader.read(&mut buffer).ok()? > 0 {
            let c = buffer[0] as char;
            if c == '{' {
                let (g, r) = Self::parse_group(reader)?;
                children.push(Box::new(g));
                reader = r;
            } else if c == '}' {
                return Some((
                    Self {
                        groups: children,
                        removed_chars: count,
                    },
                    reader,
                ));
            } else if c == '<' {
                let (c, r) = Self::parse_garbage(reader)?;
                count += c;
                reader = r;
            }
        }

        None
    }

    fn parse_garbage(mut reader: BufReader<File>) -> Option<(u32, BufReader<File>)> {
        let mut buffer = [0; 1];

        let mut skip = false;
        let mut count = 0;

        while reader.read(&mut buffer).ok()? > 0 {
            if skip {
                skip = false;
                continue;
            }
            let c = buffer[0] as char;
            if c == '!' {
                skip = true;
                continue;
            }
            if c == '>' {
                return Some((count, reader));
            }
            count += 1
        }

        None
    }
}
