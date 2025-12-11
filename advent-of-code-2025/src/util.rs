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
