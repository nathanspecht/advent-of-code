use std::ops::Add;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Dir(pub isize, pub isize);

pub const UP: Dir = Dir(0, -1);
pub const DOWN: Dir = Dir(0, 1);
pub const LEFT: Dir = Dir(-1, 0);
pub const RIGHT: Dir = Dir(1, 0);
pub const DOWNLEFT: Dir = Dir(-1, 1);
pub const DOWNRIGHT: Dir = Dir(1, 1);

impl Add<(isize, isize)> for Dir {
    type Output = (isize, isize);

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        (self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub width: isize,
    pub height: isize,
    data: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    /// Construct from an existing 2D vector.
    pub fn from(data: Vec<Vec<T>>) -> Self {
        let height: isize = data.len() as isize;
        let width: isize = if height > 0 {
            data[0].len() as isize
        } else {
            0
        };

        Grid {
            width,
            height,
            data,
        }
    }

    pub fn in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && x < self.width && y < self.height
    }

    pub fn get(&self, x: isize, y: isize) -> Option<&T> {
        if self.in_bounds(x, y) {
            Some(&self.data[y as usize][x as usize])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: isize, y: isize, value: T) {
        if self.in_bounds(x, y) {
            self.data[y as usize][x as usize] = value;
        }
    }

    pub fn _neighbor_in_bounds(&self, x: isize, y: isize, dir: Dir) -> bool {
        self.in_bounds(x + dir.0, y + dir.1)
    }

    pub fn set_neighbor(&mut self, x: isize, y: isize, dir: Dir, value: T) {
        self.set(x + dir.0, y + dir.1, value);
    }

    pub fn neighbor(&self, x: isize, y: isize, dir: Dir) -> Option<&T> {
        self.get(x + dir.0, y + dir.1)
    }
}

impl<T: PartialEq> Grid<T> {
    pub fn position(&self, val: T) -> Option<(isize, isize)> {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.data[y as usize][x as usize] == val {
                    return Some((x, y));
                }
            }
        }

        None
    }
}

impl<T: std::fmt::Display> Grid<T> {
    /// Print the grid to stdout.
    pub fn print(&self) {
        for row in &self.data {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }
    }
}
