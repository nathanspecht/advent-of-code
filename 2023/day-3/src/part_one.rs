use std::fs;

pub fn part_one() {
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
        let mut current = String::new();
        let mut is_near = false;

        while col < cols {
            let a = grid[row][col];

            if a.is_ascii_digit() {
                if is_near_symbol(&grid, row as i32, col as i32) {
                    is_near = true;
                }

                current.push(a);
            }

            if !a.is_ascii_digit() || col == cols - 1 {
                if is_near {
                    total += current.parse::<i32>().unwrap();
                }

                current = String::new();
                is_near = false;
            }

            col += 1;
        }

        row += 1;
        col = 0;
    }

    println!("{total}")
}

fn is_near_symbol(grid: &Vec<Vec<char>>, row: i32, col: i32) -> bool {
    let mut val = false;

    for x in [-1, 0, 1] {
        for y in [-1, 0, 1] {
            let both_zero = x == 0 && y == 0;
            let r = row + x;
            let c = col + y;

            if !both_zero
                && r < grid.len() as i32
                && c < grid[0].len() as i32
                && r >= 0
                && c >= 0
                && is_symbol(grid[r as usize][c as usize])
            {
                val = true;
            }
        }
    }

    val
}

fn is_symbol(input: char) -> bool {
    !input.is_ascii_digit() && input != '.'
}
