use crate::grid::{Grid, DOWN, DOWNLEFT, DOWNRIGHT};
use crate::util::read_to_grid;

pub fn run() {
    let input = read_to_grid("src/inputs/day07_b.txt");
    let grid = Grid::from(input);
    let (x, y) = grid.position('S').unwrap();

    let count = go_forward(&grid, x, y);

    println!("{}", count);
}

fn go_forward(grid: &Grid<char>, x: isize, y: isize) -> u64 {
    if !grid.in_bounds(x, y) {
        return 1;
    }

    match grid.get(x, y).unwrap() {
        '.' | 'S' => {
            let (x1, y1) = DOWN + (x, y);

            return go_forward(grid, x1, y1);
        }
        '^' => {
            let (x1, y1) = DOWNLEFT + (x, y);
            let (x2, y2) = DOWNRIGHT + (x, y);

            return go_forward(grid, x1, y1) + go_forward(grid, x2, y2);
        }
        _ => {
            panic!("Unexpected character: {}", grid.get(x, y).unwrap());
        }
    }
}
