use crate::grid::{Grid, DOWN, LEFT, RIGHT, UP};
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

    println!("{}", count);
}
