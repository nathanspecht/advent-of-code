use std::fs;

pub fn run() {
    let mut total_distance: i32 = 0;
    let mut list_a: Vec<i32> = vec![];
    let mut list_b: Vec<i32> = vec![];

    fs::read_to_string("src/inputs/day01_a.txt")
        .expect("Failed to read file")
        .lines()
        .for_each(|line| {
            let arr = line
                .split("   ")
                .collect::<Vec<_>>()
                .iter()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            list_a.push(arr[0]);
            list_b.push(arr[1]);
        });

    list_a.sort();
    list_b.sort();

    for i in 0..list_a.len() {
        total_distance += (list_a[i] - list_b[i]).abs();
    }

    println!("Total distance: {}", total_distance);
}
