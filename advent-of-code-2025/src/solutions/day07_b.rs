use crate::grid::{Grid, DOWN, DOWNLEFT, DOWNRIGHT, LEFT, RIGHT, UP};
use crate::util::read_to_grid;

pub fn run() {
    let input = read_to_grid("src/inputs/day07_a.txt");
    let mut grid = Grid::from(input);

    let mut count = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            match grid.get(x, y) {
                Some('S') => {
                    if grid.neighbor(x, y, DOWN) == Some(&'.') {
                        grid.set_neighbor(x, y, DOWN, '|');
                    }
                }
                Some('^') => {
                    if grid.neighbor(x, y, UP) == Some(&'|') {
                        count += 1;
                        grid.set_neighbor(x, y, LEFT, '|');
                        grid.set_neighbor(x, y, RIGHT, '|');
                    }
                }
                Some('.') => {
                    if grid.neighbor(x, y, UP) == Some(&'|') {
                        grid.set(x, y, '|');
                    }
                }
                _ => {}
            }
        }
    }

    grid.print();

    for x in 0..grid.width {
        if grid.get(x, grid.height - 1) == Some(&'|') {
            let (x1, y1) = UP + (x, grid.height - 1);
            count += follow_path(&grid, x1, y1);
        }
    }

    println!("{count}");
}

fn follow_path(grid: &Grid<char>, x: isize, y: isize) -> u64 {
    match grid.get(x, y) {
        Some('S') => {
            return 1;
        }
        Some('|') | Some('^') => {
            let (x1, y1) = UP + (x, y);
            return follow_path(grid, x1, y1);
        }
        Some('.') => {
            match (
                grid.neighbor(x, y, DOWNLEFT),
                grid.neighbor(x, y, DOWNRIGHT),
            ) {
                (Some('^'), Some('^')) => {
                    let (x1, y1) = DOWNLEFT + (x, y);
                    let (x2, y2) = DOWNRIGHT + (x, y);

                    return follow_path(grid, x1, y1) + follow_path(grid, x2, y2);
                }
                (Some('^'), _) => {
                    let (x1, y1) = DOWNLEFT + (x, y);

                    return follow_path(grid, x1, y1);
                }
                (_, Some('^')) => {
                    let (x1, y1) = DOWNRIGHT + (x, y);

                    return follow_path(grid, x1, y1);
                }
                _ => {
                    panic!("Invalid path");
                }
            }
        }
        _ => {
            panic!("Invalid character");
        }
    }
}
