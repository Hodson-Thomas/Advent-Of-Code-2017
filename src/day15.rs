const GENERATOR_A_FACTOR: usize = 16807;
const GENERATOR_B_FACTOR: usize = 48271;
const MODULO: usize = 2147483647;
const GENERATOR_A_START: usize = 289;
const GENERATOR_B_START: usize = 629;
const PART1_RANGE: usize = 40_000_000;
const PART2_RANGE: usize = 5_000_000;
const MASK: usize = 0xFFFF;
const GENERATOR_A_DIVIDABLE_RULE: usize = 4;
const GENERATOR_B_DIVIDABLE_RULE: usize = 8;

pub fn part1() -> usize {
    let mut score = 0;
    let a = GENERATOR_A_START;
    let b = GENERATOR_B_START;
    for _ in 0..PART1_RANGE {
        let a = a * GENERATOR_A_FACTOR % MODULO;
        let b = b * GENERATOR_B_FACTOR % MODULO;
        if a & MASK == b & MASK {
            score += 1;
        }
    }

    score
}

pub fn part2() -> usize {
    let mut score = 0;
    let a = GENERATOR_A_START;
    let b = GENERATOR_B_START;
    for i in 0..PART2_RANGE {
        println!("{i} / {PART2_RANGE}");
        let a = get_next_generator_a_value(a);
        let b = get_next_generator_b_value(b);
        if a & MASK == b & MASK {
            score += 1;
        }
    }

    score
}

fn get_next_generator_a_value(last_generated: usize) -> usize {
    let value = last_generated;
    loop {
        let value = value * GENERATOR_A_FACTOR % MODULO;
        if value % GENERATOR_A_DIVIDABLE_RULE == 0 {
            break value;
        }
    }
}

fn get_next_generator_b_value(last_generated: usize) -> usize {
    let value = last_generated;
    loop {
        let value = value * GENERATOR_B_FACTOR % MODULO;
        if value % GENERATOR_B_DIVIDABLE_RULE == 0 {
            break value;
        }
    }
}
