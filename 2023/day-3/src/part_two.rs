use std::fs;

pub fn part_two() {
    let mut total = 0;
    let mut grid: Vec<Vec<char>> = vec![Vec::new()];

    fs::read_to_string("./src/input.txt")
        .expect("should read file")
        .chars()
        .for_each(|char| {
            if char == 0xA as char {
                grid.push(Vec::new());
            } else {
                let len = grid.len();
                grid[len - 1].push(char);
            }
        });

    grid.pop();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut row: usize = 0;
    let mut col: usize = 0;

    while row < rows {
        while col < cols {
            let part_numbers = get_part_numbers(&mut grid, row as i32, col as i32);

            if part_numbers.len() == 2 {
                total += part_numbers[0] * part_numbers[1];
            }

            col += 1;
        }

        row += 1;
        col = 0;
    }

    println!("{total}");
}

fn get_part_numbers(grid: &mut Vec<Vec<char>>, row: i32, col: i32) -> Vec<u32> {
    if !is_star(grid[row as usize][col as usize]) {
        return Vec::new();
    }

    let mut part_numbers = Vec::new();

    for x in [-1, 0, 1] {
        for y in [-1, 0, 1] {
            if x == 0 && y == 0 {
                continue;
            }

            let r = row + x;
            let c = col + y;

            if out_of_bounds(grid, r, c) {
                continue;
            }

            let cell = grid[r as usize][c as usize];

            grid[r as usize][c as usize] = '.';

            if cell.is_ascii_digit() {
                let mut i = -1;
                let mut current = String::from(cell);

                while c + i >= 0 && grid[r as usize][(c + i) as usize].is_ascii_digit() {
                    current = format!("{}{}", grid[r as usize][(c + i) as usize], current);
                    grid[r as usize][(c + i) as usize] = '.';
                    i -= 1;
                }

                i = 1;

                while c + i < grid[r as usize].len() as i32
                    && grid[r as usize][(c + i) as usize].is_ascii_digit()
                {
                    current = format!("{}{}", current, grid[r as usize][(c + i) as usize]);
                    grid[r as usize][(c + i) as usize] = '.';
                    i += 1;
                }

                part_numbers.push(current.parse::<u32>().unwrap());
            }
        }
    }

    part_numbers
}

fn out_of_bounds(grid: &Vec<Vec<char>>, row: i32, col: i32) -> bool {
    row < 0 || col < 0 || row >= grid.len() as i32 || col >= grid[0].len() as i32
}

fn is_star(input: char) -> bool {
    input == '*'
}
