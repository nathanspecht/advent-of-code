use std::fs;

pub fn run() {
    let input = fs::read_to_string("src/inputs/day09_a.txt").expect("Failed to read file");
    let mut coords: Vec<(u64, u64)> = vec![];

    for line in input.lines() {
        let (a, b) = line.split_once(',').unwrap();
        let a: u64 = a.parse().unwrap();
        let b: u64 = b.parse().unwrap();

        coords.push((a, b));
    }

    let mut max_string: String = String::new();
    let mut max = 0;

    for a in &coords {
        for b in &coords {
            let width = a.0.abs_diff(b.0) + 1;
            let height = a.1.abs_diff(b.1) + 1;
            let area = width * height;

            if area > max {
                max = area;
                max_string = format!("{:?}, {:?}", a, b);
            }
        }
    }

    println!("{max}");
    println!("{max_string}");
}
