use std::fs;

const ITERATIONS: i64 = 1;

pub fn run() {
    let input = fs::read_to_string("src/inputs/day11_a.txt").expect("Failed to read file");
    let mut stones = input
        .trim()
        .split(" ")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    for _ in 0..ITERATIONS {
        for rev_idx in (0..stones.len()).rev() {
            let idx = stones.len() - rev_idx - 1;
            let stone = stones.get_mut(idx).unwrap();
            let value = stone.parse::<i64>().unwrap();
            let length = stone.chars().count() as i64;

            if value == 0 {
                *stone = "1".to_string();
                continue;
            }

            if length % 2 == 0 {
                let mid: usize = usize::try_from(length / 2).unwrap();

                let left = stone[..mid].parse::<i64>().unwrap().to_string();
                let right = stone[mid..].parse::<i64>().unwrap().to_string();

                stones.remove(idx);
                stones.insert(idx, left);
                stones.insert(idx + 1, right);

                continue;
            }

            *stone = format!("{}", value * 2024);
        }
    }

    println!("Result: {:?}", stones);
}
