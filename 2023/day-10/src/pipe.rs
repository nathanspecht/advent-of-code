use std::fmt;

pub type Pos = (i32, i32);

#[derive(PartialEq, Clone, Copy)]
pub enum PipeShape {
    V,
    H,
    NE,
    NW,
    SW,
    SE,
    G,
    S,
}

#[derive(Clone, Copy)]
pub struct Pipe {
    pub shape: PipeShape,
    pub pos: Pos,
    pub is_loop: bool,
    pub is_inside: bool,
    pub c: char,
}

impl Pipe {
    pub fn new(c: char, pos: Pos) -> Pipe {
        let shape: PipeShape = match c {
            '|' => PipeShape::V,
            '-' => PipeShape::H,
            'L' => PipeShape::NE,
            'J' => PipeShape::NW,
            '7' => PipeShape::SW,
            'F' => PipeShape::SE,
            '.' => PipeShape::G,
            'S' => PipeShape::S,
            _ => panic!("Invalid pipe char"),
        };

        Pipe {
            shape,
            pos,
            c,
            is_loop: false,
            is_inside: false,
        }
    }

    pub fn do_pipes_connect(pipe_a: &Pipe, pipe_b: &Pipe) -> bool {
        pipe_a.does_connect(pipe_b) && pipe_b.does_connect(pipe_a)
    }

    fn does_connect(&self, other_pipe: &Pipe) -> bool {
        let (x, y) = self.direction(other_pipe);

        if !(-1..=1).contains(&x) || !(-1..=1).contains(&y) {
            return false;
        }

        match self.shape {
            PipeShape::V => do_match((x, y), &[(0, 1), (0, -1)]),
            PipeShape::H => do_match((x, y), &[(1, 0), (-1, 0)]),
            PipeShape::NE => do_match((x, y), &[(0, -1), (1, 0)]),
            PipeShape::NW => do_match((x, y), &[(0, -1), (-1, 0)]),
            PipeShape::SW => do_match((x, y), &[(0, 1), (-1, 0)]),
            PipeShape::SE => do_match((x, y), &[(0, 1), (1, 0)]),
            PipeShape::G => false,
            PipeShape::S => true,
        }
    }

    fn direction(&self, other_pipe: &Pipe) -> Pos {
        (other_pipe.pos.0 - self.pos.0, other_pipe.pos.1 - self.pos.1)
    }

    pub fn add(&self, d_pos: Pos) -> Pos {
        (self.pos.0 + d_pos.0, self.pos.1 + d_pos.1)
    }

    pub fn is_at(&self, pos: Pos) -> bool {
        pos.0 == self.pos.0 && pos.1 == self.pos.1
    }
}

fn do_match(pos: Pos, check: &[Pos]) -> bool {
    check.iter().any(|(x, y)| &pos.0 == x && &pos.1 == y)
}

impl fmt::Display for Pipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {:?}", self.c, self.pos)
    }
}
