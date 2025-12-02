const INPUT: usize = 265149;

pub fn part1() -> i32 {
    if INPUT == 1 {
        return 0;
    }

    let pos = get_position(INPUT);
    return pos.0.abs() + pos.1.abs();
}

fn get_position(value: usize) -> (i32, i32) {
    let ring = get_ring(value);
    let length = ring * 2 + 1;
    let start = (2 * (ring - 1) + 1).pow(2) + 1;
    let top_right_corner = start + length - 2;
    let top_left_corner = top_right_corner + length - 1;
    let bottom_left_corner = top_left_corner + length - 1;

    if value >= bottom_left_corner {
        let y = ring as i32;
        let mut x = -(ring as i32);
        for i in 0..length {
            if bottom_left_corner + i == value {
                return (x, y);
            }
            x += 1;
        }
    } else if value >= top_left_corner {
        let x = -(ring as i32);
        let mut y = -(ring as i32);
        for i in 0..length {
            if top_left_corner + i == value {
                return (x, y);
            }
            y += 1;
        }
    } else if value >= top_right_corner {
        let y = -(ring as i32);
        let mut x = ring as i32;
        for i in 0..length {
            if top_right_corner + i == value {
                return (x, y);
            }
            x -= 1;
        }
    } else {
        let mut y = (ring as i32) - 1;
        let x = ring as i32;
        for i in 0..(length - 1) {
            if start + i == value {
                return (x, y);
            }
            y -= 1;
        }
    }
    panic!("Exceeded ring.")
}

fn get_ring(value: usize) -> usize {
    let sqrt = (value as f64).sqrt() as usize;

    if sqrt * sqrt == value && sqrt % 2 == 1 {
        return sqrt.div_ceil(2) - 1;
    }

    let sqrt = if sqrt % 2 == 0 { sqrt - 1 } else { sqrt };
    sqrt.div_ceil(2)
}
