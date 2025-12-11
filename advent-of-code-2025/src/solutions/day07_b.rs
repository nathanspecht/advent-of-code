use std::collections::HashMap;

use crate::grid::{Grid, DOWN, DOWNLEFT, DOWNRIGHT};
use crate::util::read_to_grid;

type Cache = HashMap<(isize, isize), u64>;

pub fn run() {
    let input = read_to_grid("src/inputs/day07_a.txt");
    let grid = Grid::from(input);
    let (x, y) = grid.position('S').unwrap();

    let mut cache: Cache = HashMap::new();
    let count = go_forward(&grid, x, y, &mut cache);

    println!("{}", count);
}

fn go_forward(grid: &Grid<char>, x: isize, y: isize, cache: &mut Cache) -> u64 {
    if !grid.in_bounds(x, y) {
        return 1;
    }

    if let Some(val) = cache.get(&(x, y)) {
        return *val;
    }

    match grid.get(x, y).unwrap() {
        '.' | 'S' => {
            let (x1, y1) = DOWN + (x, y);

            let val = go_forward(grid, x1, y1, cache);

            cache.insert((x, y), val);

            return val;
        }
        '^' => {
            let (x1, y1) = DOWNLEFT + (x, y);
            let (x2, y2) = DOWNRIGHT + (x, y);

            let val = go_forward(grid, x1, y1, cache) + go_forward(grid, x2, y2, cache);

            cache.insert((x, y), val);

            return val;
        }
        _ => {
            panic!("Unexpected character: {}", grid.get(x, y).unwrap());
        }
    }
}
