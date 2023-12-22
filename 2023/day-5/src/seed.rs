#[derive(Copy, Clone)]
pub struct Seed {
    pub start: u64,
    pub range: u64,
}

impl Seed {
    pub fn new(start: u64, range: u64) -> Seed {
        Seed { start, range }
    }

    pub fn is_empty(&self) -> bool {
        self.range == 0
    }

    pub fn update(&mut self, start: u64, range: u64) {
        self.start = start;
        self.range = range;
    }

    // pub fn end(&self) -> u64 {
    //     self.start + self.range - 1
    // }
}
