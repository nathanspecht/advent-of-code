use crate::util;
use std::collections::{HashMap, HashSet};

pub fn run() {
    let grid = util::read_to_grid("src/inputs/day08_a.txt");
    let mut nodes: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] != '.' {
                if nodes.contains_key(&grid[y][x]) {
                    let node = nodes.get_mut(&grid[y][x]).unwrap();
                    node.push((x as i32, y as i32));
                } else {
                    nodes.insert(grid[y][x], vec![(x as i32, y as i32)]);
                }
            }
        }
    }

    for nodes in nodes.values() {
        for (i, (x1, y1)) in nodes.iter().enumerate() {
            for (j, (x2, y2)) in nodes.iter().enumerate() {
                if i == j {
                    continue;
                }

                antinodes.insert((*x2, *y2));

                let dx = x1 - x2;
                let dy = y1 - y2;

                let mut antinode = (x1 + dx, y1 + dy);

                while util::check_bounds(&grid, antinode) {
                    antinodes.insert(antinode.clone());
                    antinode = (antinode.0 + dx, antinode.1 + dy);
                }
            }
        }
    }

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let is_antinode = antinodes.contains(&(x as i32, y as i32));

            if is_antinode && grid[y][x] == '.' {
                print!("#");
            } else {
                print!("{}", grid[y][x]);
            }
        }
        println!();
    }

    println!("{:?}", antinodes.len());
}