use std::fs;

pub fn run() {
    let mut files: Vec<i64> = vec![];
    let mut free_space: Vec<i64> = vec![];

    let input = fs::read_to_string("src/inputs/day09_a.txt").expect("Failed to read file");

    for (i, c) in input.trim().chars().enumerate() {
        let val = c.to_string().parse::<i64>().unwrap();

        if i % 2 == 0 {
            files.push(val);
        } else {
            free_space.push(val);
        }
    }

    let mut blocks: Vec<i64> = vec![];

    for i in 0..files.len() {
        let file = files[i] as usize;

        blocks.extend(vec![i as i64; file]);

        if i < free_space.len() {
            let free_space = free_space[i] as usize;

            blocks.extend(vec![-1; free_space]);
        }
    }

    let mut end_index = blocks.len() - 1;
    let mut start_index = 0;

    loop {
        while blocks[start_index] != -1 {
            start_index += 1;
        }

        while blocks[end_index] == -1 {
            end_index -= 1;
        }

        if start_index >= end_index {
            break;
        }

        blocks[start_index] = blocks[end_index];
        blocks[end_index] = -1;
    }

    let mut sum = 0;

    for i in 0..blocks.len() {
        if blocks[i] != -1 {
            sum += blocks[i] * i as i64;
        }
    }

    println!("{:?}", sum);
}
