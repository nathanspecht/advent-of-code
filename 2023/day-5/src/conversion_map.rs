use crate::seed::Seed;

pub struct ConversionMap {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

impl ConversionMap {
    pub fn new(line: &str) -> ConversionMap {
        let parts = line
            .split(' ')
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        ConversionMap {
            destination_range_start: parts[0],
            source_range_start: parts[1],
            range_length: parts[2],
        }
    }

    pub fn get_overlapping_seed(&self, seed: &mut Seed) -> (Option<Seed>, Option<Seed>) {
        let map_start = self.source_range_start;
        let map_end = self.source_range_start + self.range_length - 1;

        let seed_start = seed.start;
        let seed_end = seed.start + seed.range - 1;

        let range = seed.range;

        if seed_end < map_start || seed_start > map_end {
            return (None, None);
        }

        if seed_start >= map_start && seed_end <= map_end {
            seed.update(0, 0);
            return (
                Some(Seed::new(
                    self.destination_range_start + seed_start - map_start,
                    range,
                )),
                None,
            );
        }

        if seed_start >= map_start && seed_end > map_end {
            seed.update(map_end + 1, seed_end - map_end - 1);
            return (
                Some(Seed::new(
                    self.destination_range_start + seed_start - map_start,
                    range - (seed_end - map_end),
                )),
                None,
            );
        }

        if seed_start < map_start && seed_end <= map_end {
            seed.update(seed_start, map_start - seed_start);
            return (
                Some(Seed::new(
                    self.destination_range_start,
                    range - (map_start - seed_start),
                )),
                None,
            );
        }

        if seed_start < map_start && seed_end > map_end {
            println!("start out, end out");
            seed.update(seed_start, map_start - seed_start);

            return (
                Some(Seed::new(self.destination_range_start, self.range_length)),
                Some(Seed::new(map_end + 1, seed_end - map_end)),
            );
        }

        (None, None)
    }
}
