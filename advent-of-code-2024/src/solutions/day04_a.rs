use std::fs;

pub fn run() {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut count: i32 = 0;

    let target = ['X', 'M', 'A', 'S'];

    let dirs = [
        [-1, -1],
        [0, -1],
        [1, -1],
        [-1, 0],
        [1, 0],
        [-1, 1],
        [0, 1],
        [1, 1],
    ];

    fs::read_to_string("src/inputs/day04_a.txt")
        .expect("Failed to read file")
        .lines()
        .for_each(|line| {
            let mut row: Vec<char> = vec![];

            line.chars().for_each(|c| {
                row.push(c);
            });

            grid.push(row);
        });

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            'direction: for dir in dirs.iter() {
                let mut x1 = x as i32;
                let mut y1 = y as i32;

                for t in target.iter() {
                    if x1 < 0 || x1 >= grid[y].len() as i32 || y1 < 0 || y1 >= grid.len() as i32 {
                        continue 'direction;
                    }

                    if grid[y1 as usize][x1 as usize] != *t {
                        continue 'direction;
                    }

                    x1 += dir[0];
                    y1 += dir[1];
                }

                count += 1;
            }
        }
    }

    println!("Result: {}", count);
}
