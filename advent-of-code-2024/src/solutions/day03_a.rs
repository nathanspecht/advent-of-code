use regex::Regex;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("src/inputs/day03_a.txt").expect("Failed to read file");
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;

    re.captures_iter(&input).for_each(|cap| {
        let (_, [left, right]) = cap.extract();

        let a = left.parse::<i32>().unwrap();
        let b = right.parse::<i32>().unwrap();

        sum += a * b;
    });

    println!("Result: {}", sum);
}
