mod pipe;

use std::{collections::HashMap, fs};

use pipe::{Pipe, PipeShape, Pos};

type Pipes = HashMap<Pos, Pipe>;

fn main() {
    let mut start: Pos = (0, 0);
    let mut pipes: Pipes = HashMap::new();

    let input = fs::read_to_string("./src/input.txt").expect("to read file");

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos: Pos = (x as i32, y as i32);

            if c == 'S' {
                start = pos;
            }

            pipes.insert(pos, Pipe::new(c, pos));
        }
    }

    let loop_positions = get_loop_positions(&pipes, &start);

    for pos in &loop_positions {
        pipes.get_mut(pos).unwrap().is_loop = true;
    }

    let mut num_inside = 0;

    let mut inside: Vec<Pos> = vec![];

    for pipe in pipes.values() {
        if pipe.is_loop {
            continue;
        }

        let mut num_above = 0;
        let mut num_below = 0;
        let mut num_left = 0;
        let mut num_right = 0;

        for pos in &loop_positions {
            let loop_pipe = &pipes.get(pos).unwrap();

            if pipe.pos.0 == pos.0 && loop_pipe.shape != PipeShape::V {
                if pipe.pos.1 > pos.1 {
                    num_below += 1;
                } else {
                    num_above += 1;
                }
            }

            if pipe.pos.1 == pos.1 && loop_pipe.shape != PipeShape::H {
                if pipe.pos.0 > pos.0 {
                    num_right += 1;
                } else {
                    num_left += 1;
                }
            }
        }

        if pipe.is_at((7, 4)) {
            println!("{:?}", pipe.pos);
            println!(
                "above {}, below {}, left {}, right {}",
                num_above, num_below, num_left, num_right
            );
        }

        if num_above % 2 == 1 && num_below % 2 == 1 && num_left % 2 == 1 && num_right % 2 == 1 {
            num_inside += 1;
            inside.push(pipe.pos);
        }
    }

    for pos in inside {
        pipes.get_mut(&pos).unwrap().is_inside = true;
    }

    for (y, line) in input.lines().enumerate() {
        let mut to_print = String::new();

        for (x, _) in line.chars().enumerate() {
            let pos: Pos = (x as i32, y as i32);
            let pipe = pipes.get(&pos).unwrap();

            let m = if pipe.is_at((7, 4)) {
                '*'
            } else if pipe.is_loop {
                if pipe.shape == PipeShape::V {
                    '|'
                } else if pipe.shape == PipeShape::H {
                    '-'
                } else {
                    'O'
                }
            } else if pipe.is_inside {
                'I'
            } else {
                ' '
            };

            to_print.push(m);
        }

        println!("{to_print}");
    }

    println!("{num_inside}");
}

// TODO: collect array of sides
fn get_loop_positions(pipes: &Pipes, start: &Pos) -> Vec<Pos> {
    let mut loop_pos: Vec<Pos> = vec![*start];
    let mut current_pipe = pipes.get(start).unwrap();
    let mut prev_pipe = pipes.get(start).unwrap();

    loop {
        let offsets = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        for (dx, dy) in offsets {
            let pos = current_pipe.add((dx, dy));

            if prev_pipe.is_at(pos) {
                continue;
            }

            if !pipes.contains_key(&pos) {
                continue;
            }

            let next_pipe = pipes.get(&pos).unwrap();

            if Pipe::do_pipes_connect(current_pipe, next_pipe) {
                if next_pipe.shape == PipeShape::S {
                    return loop_pos;
                }

                loop_pos.push(pos);
                prev_pipe = current_pipe;
                current_pipe = next_pipe;
            }
        }
    }
}
