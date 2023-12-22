use regex::Regex;
use std::fs;

pub fn part_two() {
    let mut total: u32 = 0;
    let mut multipliers = vec![1; 400];

    fs::read_to_string("./src/input.txt")
        .expect("reads file")
        .lines()
        .enumerate()
        .for_each(|(index, line)| {
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

            for n in 0..count {
                multipliers[index + n as usize + 1] += multipliers[index];
            }

            total += multipliers[index];
        });

    println!("{total}");
}
