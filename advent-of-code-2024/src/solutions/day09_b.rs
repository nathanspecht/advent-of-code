use std::fs;

pub fn run() {
    let mut files: Vec<i64> = vec![];
    let mut free_space: Vec<i64> = vec![];

    let input = fs::read_to_string("src/inputs/day09_b.txt").expect("Failed to read file");

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

    for i in 0..blocks.len() {
        if blocks[i] != -1 {
            print!("[{}]", blocks[i]);
        } else {
            print!("[ ]");
        }
    }

    println!();
    println!();
    println!();

    let mut end_index = (blocks.len() - 1, blocks.len() - 1);
    let mut start_index = (0, 0);

    'outer: loop {
        while blocks[end_index.1] == -1 && end_index.1 > 0 {
            end_index.1 -= 1;
        }

        if end_index.1 == 0 {
            break;
        }

        end_index.0 = end_index.1;

        while blocks[end_index.0] == blocks[end_index.1] && end_index.0 > 0 {
            end_index.0 -= 1;
        }

        if end_index.0 == 0 {
            break;
        }

        let length_needed = end_index.1 - end_index.0;

        'inner: loop {
            while blocks[start_index.0] != -1 {
                start_index.0 += 1;

                if start_index.0 >= end_index.0 {
                    end_index.1 = end_index.0;
                    start_index = (0, 0);
                    continue 'outer;
                }
            }

            start_index.1 = start_index.0;

            while blocks[start_index.1] == -1 && start_index.1 - start_index.0 < length_needed {
                start_index.1 += 1;

                if start_index.1 >= end_index.0 {
                    end_index.1 = end_index.0;
                    start_index = (0, 0);
                    continue 'outer;
                }
            }

            if start_index.1 - start_index.0 == length_needed {
                break 'inner;
            } else if start_index.1 - start_index.0 > length_needed {
                panic!("Too long");
            } else {
                start_index.0 = start_index.1;
                continue 'inner;
            }
        }

        let val = blocks[end_index.1];

        for i in 0..length_needed {
            blocks[start_index.0 + i] = val;
        }

        for i in 1..=length_needed {
            blocks[end_index.0 + i] = -1;
        }

        start_index = (0, 0);
        end_index.1 = end_index.0;
    }

    let mut sum = 0;

    for i in 0..blocks.len() {
        if blocks[i] != -1 {
            sum += blocks[i] * i as i64;
        }
    }

    for i in 0..blocks.len() {
        if blocks[i] != -1 {
            print!("[{}]", blocks[i]);
        } else {
            print!("[ ]");
        }
    }

    println!();

    println!("{:?}", sum);
}
