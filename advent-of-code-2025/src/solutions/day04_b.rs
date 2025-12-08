use std::fs;

pub fn run() {
    let mut did_change = true;
    let mut total = 0;
    let mut grid: Vec<Vec<char>> = vec![];
    let input = fs::read_to_string("src/inputs/day04_a.txt").expect("Failed to read file");

    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    let dirs: [[i32; 2]; 8] = [
        [0, 1],
        [0, -1],
        [1, 1],
        [1, -1],
        [1, 0],
        [-1, 1],
        [-1, -1],
        [-1, 0],
    ];

    while did_change {
        let mut changed: Vec<(usize, usize)> = vec![];

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == '.' {
                    continue;
                }

                let mut num_rolls = 0;

                for dir in dirs {
                    let (x1, y1) = (x as i32 + dir[0], y as i32 + dir[1]);

                    match grid.get(y1 as usize).and_then(|row| row.get(x1 as usize)) {
                        Some('@') => num_rolls += 1,
                        _ => {}
                    }
                }

                if num_rolls < 4 {
                    total += 1;
                    changed.push((x, y));
                }
            }
        }

        did_change = changed.len() > 0;

        for (x, y) in changed {
            grid[y][x] = '.'
        }

        for row in &grid {
            for ch in row {
                print!("{}", ch);
            }
            println!(); // newline after each row
        }
    }

    println!("{:?}", total);
}
