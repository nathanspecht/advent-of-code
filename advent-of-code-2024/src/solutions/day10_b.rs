use crate::util::{self, access_grid_int};
use std::collections::HashSet;

pub fn run() {
    let grid = util::read_to_grid("src/inputs/day10_a.txt");

    let mut sum = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let height = grid[y][x].to_string().parse::<i32>().unwrap();

            if height == 0 {
                let score = calculate_score(&grid, x as i32, y as i32, 0, &HashSet::new());

                sum += score;
            }
        }
    }

    println!("Sum: {}", sum);
}

fn calculate_score(
    grid: &Vec<Vec<char>>,
    x: i32,
    y: i32,
    score: i32,
    path: &HashSet<(i32, i32)>,
) -> i32 {
    let mut new_path = path.clone();

    new_path.insert((x, y));

    let dirs: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    if !util::check_bounds(grid, (x, y)) {
        return 0;
    }

    let height = access_grid_int(grid, (x, y));

    if height == 9 {
        return score + 1;
    }

    let mut score_inc = 0;

    for (dx, dy) in dirs {
        let x1 = x + dx;
        let y1 = y + dy;

        if util::check_bounds(grid, (x1, y1)) && !new_path.contains(&(x1, y1)) {
            let new_height = access_grid_int(grid, (x1, y1));

            if new_height == height + 1 {
                score_inc += calculate_score(grid, x1, y1, score, &new_path);
            }
        }
    }

    return score + score_inc;
}
