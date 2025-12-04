use std::fs;

pub fn run() {
    let mut total: u64 = 0;
    let mut banks: Vec<&str> = vec![];

    let binding = fs::read_to_string("src/inputs/day03_a.txt").expect("Failed to read file");

    binding.lines().for_each(|line| {
        banks.push(line);
    });

    for bank in banks.iter() {
        let length: usize = 12;
        let mut max_index = 0;
        let mut digits: Vec<u64> = vec![];

        for x in 0..length {
            let mut max: u64 = 0;
            let start = max_index;
            let end = bank.len() - (length - x - 1);

            for i in start..end {
                let val = bank.chars().nth(i).unwrap().to_digit(10).unwrap() as u64;

                if val > max {
                    max = val;
                    max_index = i;
                }
            }

            digits.push(max);

            max_index = max_index + 1;
        }

        let mut joltage: u64 = 0;

        for d in digits {
            joltage = joltage * 10 + d;
        }

        total += joltage;
    }

    println!("{:?}", total);
}
