use std::fs;

pub fn run() {
    let mut total: u64 = 0;
    let input = fs::read_to_string("src/inputs/day02_a.txt").expect("Failed to read file");

    let ranges: Vec<&str> = input.trim().split(',').collect();

    for range in &ranges {
        let (start, end) = range
            .split_once('-')
            .map(|(a, b)| (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()))
            .unwrap();

        for x in start..=end {
            let id = x.to_string();
            let len = id.chars().count();

            let mid = len / 2;

            if id[..mid] == id[mid..] {
                total += x;
            }
        }
    }

    println!("{total}");
}
