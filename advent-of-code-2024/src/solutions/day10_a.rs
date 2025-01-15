use crate::util;
use std::collections::HashSet;

pub fn run() {
    let grid = util::read_to_grid("src/inputs/day10_a.txt");

    let mut sum = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let height = grid[y][x].to_string().parse::<i32>().unwrap();

            if height == 0 {
                sum += calculate_score(&grid, x as i32, y as i32, 0, &mut HashSet::new());
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
    path: &mut HashSet<(i32, i32)>,
) -> i32 {
    path.insert((x, y));

    let dirs: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    if !util::check_bounds(&grid, (x, y)) {
        return 0;
    }

    let height = grid[y as usize][x as usize].to_digit(10).unwrap();

    if height == 9 {
        return score + 1;
    }

    let mut score_inc = 0;

    for (dx, dy) in dirs {
        let x1 = x + dx;
        let y1 = y + dy;

        if util::check_bounds(&grid, (x1, y1)) && !path.contains(&(x1, y1)) {
            let new_height = grid[y1 as usize][x1 as usize].to_digit(10).unwrap();

            if new_height == height + 1 {
                score_inc += calculate_score(&grid, x1, y1, score, path);
            }
        }
    }

    return score + score_inc;
}
