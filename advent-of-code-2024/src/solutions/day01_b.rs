use std::collections::HashMap;
use std::fs;

pub fn run() {
    let mut similarity_score: i32 = 0;
    let mut list_a: HashMap<i32, i32> = HashMap::new();
    let mut list_b: Vec<i32> = vec![];

    fs::read_to_string("src/inputs/day01_b.txt")
        .expect("Failed to read file")
        .lines()
        .for_each(|line| {
            let arr = line
                .split("   ")
                .collect::<Vec<_>>()
                .iter()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            list_a.insert(arr[0], 0);
            list_b.push(arr[1]);
        });

    for i in 0..list_b.len() {
        if list_a.contains_key(&list_b[i]) {
            *list_a.get_mut(&list_b[i]).unwrap() += 1;
        }
    }

    for (k, v) in list_a.iter() {
        similarity_score += k * v;
    }

    println!("Similarity score: {}", similarity_score);
}
