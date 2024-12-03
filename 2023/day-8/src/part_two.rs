use regex::Regex;
use std::{collections::HashMap, fs};

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

    let mut steps: u32 = 0;
    let mut current: Vec<&str> = map
        .keys()
        .filter(|key| key.ends_with('A'))
        .cloned()
        .collect::<Vec<_>>();

    let mut paths: Vec<Vec<String>> = vec![vec![]; current.len()];
    let mut completed: Vec<bool> = vec![false; current.len()];

    while !is_finished(&completed) {
        for instruction in &instructions {
            if is_finished(&completed) {
                break;
            }

            steps += 1;

            for index in 0..current.len() {
                let current_key = current[index];
                let path = format!("{}-{}", current_key, instruction);

                if paths[index].contains(&path) {
                    completed[index] = true;
                    continue;
                } else {
                    paths[index].push(path);
                }

                let (left, right) = map.get(current_key).unwrap();

                match instruction {
                    'L' => current[index] = left,
                    'R' => current[index] = right,
                    _ => panic!("Invalid instruction"),
                }
            }
        }
    }

    println!("{:?}", paths);

    println!("{}", steps);
}

fn is_finished(completed: &[bool]) -> bool {
    completed.iter().all(|complete| *complete)
}
