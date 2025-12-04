use std::fs;

pub fn run() {
    let mut total: u32 = 0;
    let mut banks: Vec<&str> = vec![];

    let binding = fs::read_to_string("src/inputs/day03_a.txt").expect("Failed to read file");

    binding.lines().for_each(|line| {
        banks.push(line);
    });

    for bank in banks.iter() {
        let mut max: u32 = bank.chars().next().unwrap().to_string().parse().unwrap();
        let mut max_index: usize = 0;

        for (i, c) in bank.chars().enumerate() {
            if i == bank.len() - 1 {
                continue;
            }

            let val = c.to_string().parse::<u32>().unwrap();

            if val > max {
                max = val;
                max_index = i;
            }
        }

        let remaining = &bank[(max_index + 1)..];
        let mut next_max = remaining
            .chars()
            .next()
            .unwrap()
            .to_string()
            .parse::<u32>()
            .unwrap();

        for c in remaining.chars() {
            let val = c.to_string().parse::<u32>().unwrap();

            if val > next_max {
                next_max = val;
            }
        }

        let val: u32 = format!("{max}{next_max}").parse().unwrap();

        total += val;
    }

    println!("{:?}", total);
}
