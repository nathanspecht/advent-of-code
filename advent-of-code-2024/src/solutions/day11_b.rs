use std::{collections::HashMap, fs};

const ITERATIONS: i64 = 60;

pub fn run() {
    let input = fs::read_to_string("src/inputs/day11_a.txt").expect("Failed to read file");
    let mut cache = HashMap::new();
    let mut stones = input
        .trim()
        .split(" ")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    cache_lengths(ITERATIONS, vec![String::from("0")], &mut cache);

    let mut cache_sum = 0;

    println!("Cache: {:?}", cache);

    for n in 0..ITERATIONS {
        for rev_idx in (0..stones.len()).rev() {
            let idx = stones.len() - rev_idx - 1;
            let stone = stones.get_mut(idx).unwrap();
            let value = stone.parse::<i64>().unwrap();
            let length = stone.chars().count();

            if value == 0 {
                stones.remove(idx);
                cache_sum += cache.get(&(ITERATIONS - n - 1)).unwrap();
                continue;
            }

            if length % 2 == 0 {
                let mid = length / 2;

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

    println!("Result: {:?}", stones.len() as i64 + cache_sum);
}

fn cache_lengths(iterations: i64, stones: Vec<String>, cache: &mut HashMap<i64, i64>) {
    let mut stones = stones.clone();
    let mut cache_sum = 0;

    for n in 0..iterations {
        println!("Iteration: {}, {}", n, stones.len());
        for rev_idx in (0..stones.len()).rev() {
            let idx = stones.len() - rev_idx - 1;
            let stone = stones.get_mut(idx).unwrap();
            let value = stone.parse::<i64>().unwrap();
            let length = stone.chars().count();

            if value == 0 {
                if cache.contains_key(&(iterations - n - 1)) {
                    stones.remove(idx);
                    cache_sum += cache.get(&(iterations - n - 1)).unwrap();
                } else {
                    *stone = "1".to_string();
                }
            } else if length % 2 == 0 {
                let mid = length / 2;

                let left = stone[..mid].parse::<i64>().unwrap().to_string();
                let right = stone[mid..].parse::<i64>().unwrap().to_string();

                stones.remove(idx);
                stones.insert(idx, left);
                stones.insert(idx + 1, right);
            } else {
                *stone = format!("{}", value * 2024);
            }

            cache.insert(n, stones.len() as i64 + cache_sum);
        }
    }
}
