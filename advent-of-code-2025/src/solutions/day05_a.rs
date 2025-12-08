use std::fs;

pub fn run() {
    let mut count: u64 = 0;
    let mut fresh: Vec<(u64, u64)> = vec![];
    let mut available: Vec<u64> = vec![];

    let input = fs::read_to_string("src/inputs/day05_a.txt").expect("Failed to read file");

    match input.split_once("\n\n") {
        Some((top, bottom)) => {
            top.split('\n').for_each(|row| {
                let (min, max) = row.split_once('-').unwrap();
                let min: u64 = min.parse().unwrap();
                let max: u64 = max.parse().unwrap();

                fresh.push((min, max));
            });

            bottom.trim().split('\n').for_each(|row| {
                let id: u64 = row.parse().unwrap();
                available.push(id);
            })
        }
        _ => {}
    }

    'available: for id in available {
        for (min, max) in &fresh {
            if id >= *min && id <= *max {
                count += 1;
                continue 'available;
            }
        }
    }

    println!("{count}");
}
