use std::collections::HashSet;

use crate::util;

pub fn run() {
    let grid = util::read_to_grid("src/inputs/day06_a.txt");

    let mut total = 0;
    let mut progress = 0;
    let mut count = 0;
    let mut pos: (i32, i32) = (0, 0);

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            match grid[y][x] {
                '^' => pos = (x as i32, y as i32),
                '.' => total += 1,
                _ => (),
            }
        }
    }

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '.' {
                if does_loop(&grid, (x as i32, y as i32), pos) {
                    count += 1;
                }
                progress += 1;

                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                println!("Progress: {}/{}", progress, total);
            }
        }
    }

    println!("{}", count);
}

fn does_loop(grid: &Vec<Vec<char>>, ob_pos: (i32, i32), start_pos: (i32, i32)) -> bool {
    let mut grid = grid.clone();

    grid[ob_pos.1 as usize][ob_pos.0 as usize] = '#';

    let mut steps = HashSet::new();
    let mut pos = start_pos;
    let mut dir_index = 0;

    let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    loop {
        let dir = dirs[dir_index];
        let pos_key = format!("{},{},{},{}", pos.0, pos.1, dir.0, dir.1);

        if steps.contains(&pos_key) {
            return true;
        }

        steps.insert(pos_key);

        let mut next = util::apply_dir(pos, dir);

        if !util::check_bounds(&grid, next) {
            return false;
        }

        while grid[next.1 as usize][next.0 as usize] == '#' {
            dir_index = (dir_index + 1) % 4;
            next = util::apply_dir(pos, dirs[dir_index]);
        }

        pos = next;
    }
}
