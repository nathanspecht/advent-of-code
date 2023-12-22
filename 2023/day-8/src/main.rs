use regex::Regex;
use std::{collections::HashMap, fs};

fn main() {
    #![allow(clippy::needless_range_loop)]

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let input = fs::read_to_string("./src/input.txt").expect("to read file");
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().collect::<Vec<_>>();

    lines.next();

    for line in lines {
        let mut line_split = line.split(" = ");
        let key = line_split.next().unwrap();

        let re = Regex::new(r"([A-Z0-9]+),\s([A-Z0-9]+)").unwrap();
        let caps = re.captures(line_split.next().unwrap()).unwrap();
        let val = (caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str());

        map.insert(key, val);
    }

    let mut steps: u32 = 0;
    let mut current: Vec<&str> = map
        .keys()
        .filter(|key| key.ends_with('A'))
        .cloned()
        .collect::<Vec<_>>();

    while !is_finished(&current) {
        for instruction in &instructions {
            println!("{:?}", current);

            if is_finished(&current) {
                break;
            }

            steps += 1;

            for index in 0..current.len() {
                let current_key = current[index];

                if current_key.ends_with('Z') {}

                let (left, right) = map.get(current_key).unwrap();

                match instruction {
                    'L' => current[index] = left,
                    'R' => current[index] = right,
                    _ => panic!("Invalid instruction"),
                }
            }
        }
    }

    println!("{}", steps);
}

fn is_finished(current: &[&str]) -> bool {
    current.iter().all(|key| key.ends_with('Z'))
}
