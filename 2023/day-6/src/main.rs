use std::fs;

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("to read file");
    let lines = input.lines().collect::<Vec<_>>();

    let time = parse_line_two(lines[0]);
    let distance = parse_line_two(lines[1]);

    println!("{} {}", time, distance);

    let mut total: u32 = 1;

    let mut count = 0;
    let mut time_held = 0;

    while time_held < time {
        if get_distance(time_held, time) > distance {
            count += 1;
        }

        time_held += 1;
    }

    total *= count;

    println!("{}", total);
}

fn get_distance(time_held: u64, total_time: u64) -> u64 {
    (total_time - time_held) * time_held
}

fn parse_line(line: &str) -> Vec<u32> {
    line.split(':')
        .last()
        .unwrap()
        .split(' ')
        .flat_map(|s| s.parse::<u32>())
        .collect::<Vec<_>>()
}

fn parse_line_two(line: &str) -> u64 {
    line.split(':')
        .last()
        .unwrap()
        .replace(' ', "")
        .parse::<u64>()
        .unwrap()
}
