use regex::Regex;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("src/inputs/day03_a.txt").expect("Failed to read file");
    let re = Regex::new(r"(do\(\))|(don't\(\))|mul\((\d+),(\d+)\)").unwrap();
    let mut enabled = true;
    let mut sum = 0;

    re.captures_iter(&input).for_each(|cap| {
        let should_enable = cap.get(1).is_some();
        let should_disable = cap.get(2).is_some();

        if should_enable {
            enabled = true;
            return;
        } else if should_disable {
            enabled = false;
            return;
        }

        if !enabled {
            return;
        }

        match (cap.get(3), cap.get(4)) {
            (Some(a), Some(b)) => {
                let a = a.as_str().parse::<i32>().unwrap();
                let b = b.as_str().parse::<i32>().unwrap();

                sum += a * b;
            }
            _ => (),
        }
    });

    println!("Result: {}", sum);
}
