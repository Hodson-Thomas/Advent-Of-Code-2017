const STEP: usize = 337;
const CYCLES1: usize = 2017;
const CYCLES2: usize = 50_000_000;

pub fn part1() -> usize {
    let mut buffer: Vec<usize> = vec![0];
    let mut pos = 0;
    for i in 1..=CYCLES1 {
        pos = (pos + STEP) % buffer.len();
        buffer.insert(pos + 1, i);
        pos = pos + 1;
    }

    let index = buffer.iter().position(|n| *n == 2017).unwrap();
    buffer[(index + 1) % buffer.len()]
}

pub fn part2() -> usize {
    let mut index_0 = 0;
    let mut next_value = 0;
    let mut length = 1;
    let mut pos = 0;
    for i in 1..=CYCLES2 {
        pos = (pos + STEP) % length;

        if pos == index_0 {
            next_value = i;
        }

        if pos < index_0 {
            index_0 += 1;
        }

        length += 1;
        pos += 1;
    }
    next_value
}
