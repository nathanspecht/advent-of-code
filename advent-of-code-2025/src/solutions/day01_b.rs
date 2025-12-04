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
        let prev = dial;
        let offset = count % 100;

        match dir {
            'L' => dial -= offset,
            'R' => dial += offset,
            _ => panic!("Invalid direction"),
        }

        let over = dial >= 100;
        let under = dial < 0;
        let zero = dial == 0;
        let was_zero = prev == 0;

        num_zeroes += count / 100;

        if !was_zero && (over || under || zero) {
            num_zeroes += 1;
        }

        if over {
            dial = dial - 100;
        } else if under {
            dial = dial + 100;
        }
    }

    println!("Num zeroes: {:?}", num_zeroes);
}
