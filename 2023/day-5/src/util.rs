use crate::conversion_map::ConversionMap;
use crate::seed::Seed;
use std::fs;

pub fn parse_input(seeds: &mut Vec<Seed>, maps: &mut Vec<Vec<ConversionMap>>) {
    for (index, line) in fs::read_to_string("./src/input.txt")
        .expect("should open file")
        .lines()
        .enumerate()
    {
        if index == 0 {
            get_seeds(line, seeds);
            continue;
        }

        let first_char = line.chars().next().unwrap_or(' ');

        if first_char.is_ascii_alphabetic() {
            maps.push(vec![]);
            continue;
        }

        if first_char.is_ascii_digit() {
            let current_map_index = maps.len() - 1;
            maps[current_map_index].push(ConversionMap::new(line))
        }
    }
}

fn get_seeds(line: &str, seeds: &mut Vec<Seed>) {
    let seeds_str = line.split(": ").collect::<Vec<_>>();

    let values = seeds_str[1]
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut i = 0;

    while i < values.len() - 1 {
        seeds.push(Seed::new(values[i], values[i + 1]));

        i += 2;
    }
}
