use std::fs;

struct Block {
    id: i64,
    length: i64,
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for _ in 0..self.length {
            let _ = match self.id {
                -1 => write!(f, "[ ]"),
                _ => write!(f, "[{}]", self.id),
            };
        }

        Ok(())
    }
}

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

    let mut blocks: Vec<Block> = vec![];

    for i in 0..files.len() {
        blocks.push(Block {
            id: i as i64,
            length: files[i] as i64,
        });

        if i < free_space.len() {
            blocks.push(Block {
                id: -1,
                length: free_space[i] as i64,
            });
        }
    }

    let mut end_index = blocks.len() - 1;
    let mut start_index = 0;

    loop {
        while start_index < blocks.len() - 1 && blocks[start_index].id != -1 {
            start_index += 1;
        }

        while blocks[end_index].id == -1 {
            end_index -= 1;
        }

        if end_index == 0 {
            break;
        }

        if start_index >= blocks.len() {
            start_index = 0;

            if end_index == 0 {
                break;
            }

            end_index -= 1;

            continue;
        }

        if blocks[end_index].length > blocks[start_index].length {
            start_index += 1;
            continue;
        }

        if start_index >= end_index {
            start_index = 0;
            end_index -= 1;
            continue;
        }

        blocks[start_index].length -= blocks[end_index].length;

        let end_id = blocks[end_index].id;
        let end_length = blocks[end_index].length;

        blocks[end_index].id = -1;

        blocks.insert(
            start_index,
            Block {
                id: end_id,
                length: end_length,
            },
        );

        let mut to_remove = vec![];

        for i in 0..blocks.len() {
            if blocks[i].length == 0 {
                to_remove.push(i);
            }
        }

        to_remove.reverse();

        for k in to_remove {
            blocks.remove(k);
        }

        start_index = 0;

        if end_index >= blocks.len() {
            end_index = blocks.len() - 1;
        }
    }

    let mut sum = 0;
    let mut k = 0;

    for i in 0..blocks.len() {
        for _ in 0..blocks[i].length {
            if blocks[i].id != -1 {
                sum += blocks[i].id * k as i64;
            }
            k += 1;
        }
    }

    println!("{:?}", sum);
}
