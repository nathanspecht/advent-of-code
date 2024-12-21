use std::fs;

pub fn run() {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut count: i32 = 0;

    fs::read_to_string("src/inputs/day04_b.txt")
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
            let y_bound = grid.len() - 1;
            let x_bound = grid[y].len() - 1;

            if x < 1 || x >= x_bound || y < 1 || y >= y_bound {
                continue;
            }

            if grid[y][x] != 'A' {
                continue;
            }

            let cross = (
                grid[y - 1][x - 1],
                grid[y - 1][x + 1],
                grid[y + 1][x - 1],
                grid[y + 1][x + 1],
            );

            match cross {
                ('M', 'M', 'S', 'S')
                | ('M', 'S', 'M', 'S')
                | ('S', 'M', 'S', 'M')
                | ('S', 'S', 'M', 'M') => {
                    count += 1;
                }
                _ => continue,
            }
        }
    }

    println!("Result: {}", count);
}
