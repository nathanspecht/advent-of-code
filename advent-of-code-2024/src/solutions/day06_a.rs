use std::collections::HashSet;

use crate::util;

pub fn run() {
    let grid = util::read_to_grid("src/inputs/day06_a.txt");

    let mut steps = HashSet::new();
    let mut pos: (i32, i32) = (0, 0);
    let mut dir_index = 0;

    let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '^' {
                pos = (x as i32, y as i32);
            }
        }
    }

    loop {
        steps.insert(pos);

        let mut next = util::apply_dir(pos, dirs[dir_index]);

        if !util::check_bounds(&grid, next) {
            break;
        }

        while grid[next.1 as usize][next.0 as usize] == '#' {
            dir_index = (dir_index + 1) % 4;
            next = util::apply_dir(pos, dirs[dir_index]);
        }

        pos = next;
    }

    println!("{:?}", steps.len());
}
