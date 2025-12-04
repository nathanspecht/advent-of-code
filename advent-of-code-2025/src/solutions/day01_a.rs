use std::fs;

pub fn run() {
    let mut dial: i32 = 50;
    let mut num_zeroes: i32 = 0;
    let mut rotations: Vec<(char, i32)> = vec![];

    fs::read_to_string("src/inputs/day01_a.txt")
        .expect("Failed to read file")
        .lines()
        .for_each(|line| {
            let mut chars = line.chars();
            let direction = chars.next().unwrap();
            let count = chars.as_str().parse::<i32>().unwrap();

            rotations.push((direction, count))
        });

    for (dir, count) in rotations {
        let offset = count % 100;

        match dir {
            'L' => dial -= offset,
            'R' => dial += offset,
            _ => panic!("Invalid direction"),
        }

        if dial > 99 {
            dial = dial - 100;
        } else if dial < 0 {
            dial = dial + 100;
        }

        if dial == 0 {
            num_zeroes += 1;
        }
    }

    println!("Num zeroes: {:?}", num_zeroes);
}
