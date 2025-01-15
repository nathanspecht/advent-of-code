use std::fs;

pub fn read_to_grid(path: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = vec![];

    fs::read_to_string(path)
        .expect("Failed to read file")
        .lines()
        .for_each(|line| {
            let mut row: Vec<char> = vec![];

            line.chars().for_each(|c| {
                row.push(c);
            });

            grid.push(row);
        });

    grid
}

pub fn check_bounds(grid: &Vec<Vec<char>>, pos: (i32, i32)) -> bool {
    pos.0 >= 0 && pos.0 < grid[0].len() as i32 && pos.1 >= 0 && pos.1 < grid.len() as i32
}

pub fn apply_dir(pos: (i32, i32), dir: (i32, i32)) -> (i32, i32) {
    (pos.0 + dir.0, pos.1 + dir.1)
}

//pub fn access_grid_char(grid: &Vec<Vec<char>>, pos: (i32, i32)) -> char {
//    grid[pos.1 as usize][pos.0 as usize]
//}

pub fn access_grid_int(grid: &Vec<Vec<char>>, pos: (i32, i32)) -> u32 {
    grid[pos.1 as usize][pos.0 as usize].to_digit(10).unwrap()
}
