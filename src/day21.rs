use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct Transform2D {
    pub rules: [[[char; 2]; 2]; 6],
    pub to: [[char; 3]; 3],
}

pub struct Transform3D {
    pub rules: [[[char; 3]; 3]; 6],
    pub to: [[char; 4]; 4],
}

pub fn part1(file_path: &str) -> usize {
    let mut image = vec![
        vec!['.', '#', '.'],
        vec!['.', '.', '#'],
        vec!['#', '#', '#'],
    ];

    let (rules2, rules3) = parse_input(file_path);

    for _ in 0..5 {
        image = transform_image(image, &rules2, &rules3);
    }

    image
        .iter()
        .map(|row| {
            row.iter()
                .map(|p| if *p == '#' { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

pub fn part2(file_path: &str) -> usize {
    let mut image = vec![
        vec!['.', '#', '.'],
        vec!['.', '.', '#'],
        vec!['#', '#', '#'],
    ];

    let (rules2, rules3) = parse_input(file_path);

    for _ in 0..18 {
        image = transform_image(image, &rules2, &rules3);
    }

    image
        .iter()
        .map(|row| {
            row.iter()
                .map(|p| if *p == '#' { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

fn transform_image(
    image: Vec<Vec<char>>,
    rules2: &Vec<Transform2D>,
    rules3: &Vec<Transform3D>,
) -> Vec<Vec<char>> {
    print_image(&image);
    if image.len() % 2 == 0 {
        let mut res = vec![];
        for i in 0..(image.len() / 2) {
            let mut row1 = vec![];
            let mut row2 = vec![];
            let mut row3 = vec![];

            for j in 0..(image.len() / 2) {
                let pixel = [
                    [image[i * 2][j * 2], image[i * 2][j * 2 + 1]],
                    [image[i * 2 + 1][j * 2], image[i * 2 + 1][j * 2 + 1]],
                ];
                for rule in rules2 {
                    if let Some(transform) = rule.transform(pixel) {
                        row1.extend_from_slice(&transform[0]);
                        row2.extend_from_slice(&transform[1]);
                        row3.extend_from_slice(&transform[2]);
                        break;
                    }
                }
            }
            res.push(row1);
            res.push(row2);
            res.push(row3);
        }
        return res;
    }

    dbg!("Div by 3");

    let mut res = vec![];

    for i in 0..(image.len() / 3) {
        let mut row1 = vec![];
        let mut row2 = vec![];
        let mut row3 = vec![];
        let mut row4 = vec![];
        for j in 0..(image.len() / 3) {
            let pixel = [
                [
                    image[i * 2][j * 2],
                    image[i * 2][j * 2 + 1],
                    image[i * 2][j * 2 + 2],
                ],
                [
                    image[i * 2 + 1][j * 2],
                    image[i * 2 + 1][j * 2 + 1],
                    image[i * 2 + 1][j * 2 + 2],
                ],
                [
                    image[i * 2 + 2][j * 2],
                    image[i * 2 + 2][j * 2 + 1],
                    image[i * 2 + 2][j * 2 + 2],
                ],
            ];

            for rule in rules3 {
                if let Some(transform) = rule.transform(pixel) {
                    row1.extend_from_slice(&transform[0]);
                    row2.extend_from_slice(&transform[1]);
                    row3.extend_from_slice(&transform[2]);
                    row4.extend_from_slice(&transform[3]);
                }
            }
        }
        res.push(row1);
        res.push(row2);
        res.push(row3);
        res.push(row4);
    }

    res
}

fn print_image(image: &Vec<Vec<char>>) {
    println!("");
    for row in image {
        for c in row {
            print!("{c}");
        }
        println!("");
    }
    println!("");
}

fn parse_input(file_path: &str) -> (Vec<Transform2D>, Vec<Transform3D>) {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut rules2 = vec![];
    let mut rules3 = vec![];
    for line in reader.lines() {
        if let Ok(line) = line {
            let split: Vec<&str> = line.trim().split(" => ").collect();
            if split[0].len() == 5 {
                if let Some(rule) = Transform2D::new(split[0], split[1]) {
                    rules2.push(rule);
                }
            } else {
                if let Some(rule) = Transform3D::new(split[0], split[1]) {
                    rules3.push(rule);
                }
            }
        }
    }

    return (rules2, rules3);
}

fn compare_pixels_2(pixel1: &[[char; 2]; 2], pixel2: &[[char; 2]; 2]) -> bool {
    for row in 0..2 {
        for col in 0..2 {
            if pixel1[row][col] != pixel2[row][col] {
                return false;
            }
        }
    }
    return true;
}

fn compare_pixels_3(pixel1: &[[char; 3]; 3], pixel2: &[[char; 3]; 3]) -> bool {
    for row in 0..3 {
        for col in 0..3 {
            if pixel1[row][col] != pixel2[row][col] {
                return false;
            }
        }
    }
    return true;
}

fn try_create_pixel2(str: &str) -> Option<[[char; 2]; 2]> {
    let content: Vec<&str> = str.trim().split('/').collect();
    if content.len() != 2 {
        return None;
    }

    let mut pixel = [['.', '.'], ['.', '.']];
    for i in 0..2 {
        let chars: Vec<char> = content[i].chars().collect();
        if chars.len() != 2 {
            return None;
        }
        for j in 0..2 {
            pixel[i][j] = chars[j];
        }
    }

    Some(pixel)
}

fn try_create_pixel3(str: &str) -> Option<[[char; 3]; 3]> {
    let content: Vec<&str> = str.trim().split('/').collect();
    if content.len() != 3 {
        return None;
    }

    let mut pixel = [['.', '.', '.'], ['.', '.', '.'], ['.', '.', '.']];
    for i in 0..3 {
        let chars: Vec<char> = content[i].chars().collect();
        if chars.len() != 3 {
            return None;
        }
        for j in 0..3 {
            pixel[i][j] = chars[j];
        }
    }

    Some(pixel)
}

fn try_create_pixel4(str: &str) -> Option<[[char; 4]; 4]> {
    let content: Vec<&str> = str.trim().split('/').collect();
    if content.len() != 4 {
        return None;
    }

    let mut pixel = [
        ['.', '.', '.', '.'],
        ['.', '.', '.', '.'],
        ['.', '.', '.', '.'],
        ['.', '.', '.', '.'],
    ];
    for i in 0..4 {
        let chars: Vec<char> = content[i].chars().collect();
        if chars.len() != 4 {
            return None;
        }
        for j in 0..4 {
            pixel[i][j] = chars[j];
        }
    }

    Some(pixel)
}

fn create_rotations_flips_2(pixel: [[char; 2]; 2]) -> [[[char; 2]; 2]; 6] {
    let rotate1 = [[pixel[1][0], pixel[0][0]], [pixel[1][1], pixel[0][1]]];

    let rotate2 = [[pixel[1][1], pixel[1][0]], [pixel[0][1], pixel[0][0]]];

    let rotate3 = [[pixel[0][1], pixel[1][1]], [pixel[0][0], pixel[1][0]]];

    let flip1 = [[pixel[1][0], pixel[1][1]], [pixel[0][0], pixel[0][1]]];

    let flip2 = [[pixel[0][1], pixel[0][0]], [pixel[1][1], pixel[1][0]]];

    [pixel, rotate1, rotate2, rotate3, flip1, flip2]
}

fn create_rotations_flips_3(pixel: [[char; 3]; 3]) -> [[[char; 3]; 3]; 6] {
    let rotate1 = [
        [pixel[2][0], pixel[1][0], pixel[0][0]],
        [pixel[2][1], pixel[1][1], pixel[0][1]],
        [pixel[2][2], pixel[1][2], pixel[0][2]],
    ];

    let rotate2 = [
        [pixel[2][2], pixel[2][1], pixel[2][0]],
        [pixel[1][2], pixel[1][1], pixel[1][0]],
        [pixel[0][2], pixel[0][1], pixel[0][0]],
    ];

    let rotate3 = [
        [pixel[0][2], pixel[1][2], pixel[2][2]],
        [pixel[0][1], pixel[1][1], pixel[2][1]],
        [pixel[0][0], pixel[1][0], pixel[2][0]],
    ];

    let flip1 = [
        [pixel[2][0], pixel[1][0], pixel[0][0]],
        [pixel[2][1], pixel[1][1], pixel[0][1]],
        [pixel[2][2], pixel[1][2], pixel[0][2]],
    ];

    let flip2 = [
        [pixel[2][0], pixel[2][1], pixel[2][2]],
        [pixel[1][0], pixel[1][1], pixel[1][2]],
        [pixel[0][0], pixel[0][1], pixel[0][2]],
    ];
    [pixel, rotate1, rotate2, rotate3, flip1, flip2]
}

impl Transform2D {
    pub fn new(from: &str, to: &str) -> Option<Transform2D> {
        Some(Self {
            rules: create_rotations_flips_2(try_create_pixel2(from)?),
            to: try_create_pixel3(to)?,
        })
    }

    pub fn transform(&self, pixel: [[char; 2]; 2]) -> Option<[[char; 3]; 3]> {
        for rule in self.rules {
            if compare_pixels_2(&rule, &pixel) {
                return Some(self.to.clone());
            }
        }
        return None;
    }
}

impl Transform3D {
    pub fn new(from: &str, to: &str) -> Option<Transform3D> {
        Some(Self {
            rules: create_rotations_flips_3(try_create_pixel3(from)?),
            to: try_create_pixel4(to)?,
        })
    }

    pub fn transform(&self, pixel: [[char; 3]; 3]) -> Option<[[char; 4]; 4]> {
        for rule in self.rules {
            if compare_pixels_3(&rule, &pixel) {
                return Some(self.to.clone());
            }
        }
        return None;
    }
}
