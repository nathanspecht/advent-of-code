use regex::Regex;
use std::fs;

pub fn part_one() {
    let mut total: u32 = 0;

    fs::read_to_string("./src/input.txt")
        .expect("reads file")
        .lines()
        .for_each(|line| {
            let mut count: u32 = 0;

            let reg = Regex::new(r"[\d ]+").unwrap();
            let caps: Vec<_> = reg.find_iter(line).map(|m| m.as_str()).collect();

            let winning_numbers: Vec<_> = caps[1]
                .trim()
                .split(' ')
                .filter_map(|s| s.parse::<u32>().ok())
                .collect();

            let chosen_numbers = caps[2]
                .trim()
                .split(' ')
                .filter_map(|s| s.parse::<u32>().ok());

            for number in chosen_numbers {
                if winning_numbers.contains(&number) {
                    count += 1
                }
            }

            let base: u32 = 2;
            let points = if count == 0 { 0 } else { base.pow(count - 1) };

            total += points;
        });

    println!("{total}");
}
