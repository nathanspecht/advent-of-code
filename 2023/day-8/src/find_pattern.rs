use num_integer::Integer;
use regex::Regex;
use std::{collections::HashMap, fs};

struct Path {
    pub last_key: String,
}

impl Path {
    pub fn new(last_key: String) -> Path {
        Path { last_key }
    }
}

pub fn run() {
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

    let mut paths: HashMap<String, Path> = HashMap::new();

    for key in map.keys() {
        let mut current = key;
        let mut z_index: Vec<usize> = vec![];

        for (index, instruction) in instructions.iter().enumerate() {
            if current.ends_with('Z') {
                z_index.push(index);
            }

            let (left, right) = map.get(current).unwrap();

            match instruction {
                'L' => current = left,
                'R' => current = right,
                _ => panic!("Invalid instruction"),
            }
        }

        if current.ends_with('Z') {
            z_index.push(instructions.len() - 1);
        }

        paths.insert(key.to_string(), Path::new(current.to_string()));
    }

    let mut current_keys: Vec<&str> = map
        .keys()
        .filter(|key| key.ends_with('A'))
        .cloned()
        .collect::<Vec<_>>();

    let mut factors: Vec<u64> = vec![];

    for i in 0..current_keys.len() {
        let mut steps: u64 = 0;

        loop {
            let current = current_keys[i];

            let next = &paths.get(current).unwrap().last_key;

            current_keys[i] = next.as_str();

            steps += instructions.len() as u64;

            if current_keys[i].ends_with('Z') {
                println!("{steps}");
                factors.push(steps);
                break;
            }
        }
    }

    println!("{:?}", factors);

    let total = lcm_multiple(&factors);

    println!("{:?}", total);
}

fn lcm_multiple(numbers: &[u64]) -> Option<u64> {
    match numbers.len() {
        0 => None,
        1 => Some(numbers[0]),
        _ => Some(numbers[0].lcm(&lcm_multiple(&numbers[1..])?)),
    }
}
