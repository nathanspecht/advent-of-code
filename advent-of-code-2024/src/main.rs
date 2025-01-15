mod solutions;
mod util;

const CURRENT: &str = "11A";

fn main() {
    match CURRENT {
        "1A" => solutions::day01_a::run(),
        "1B" => solutions::day01_b::run(),
        "2A" => solutions::day02_a::run(),
        "2B" => solutions::day02_b::run(),
        "3A" => solutions::day03_a::run(),
        "3B" => solutions::day03_b::run(),
        "4A" => solutions::day04_a::run(),
        "4B" => solutions::day04_b::run(),
        "5A" => solutions::day05_a::run(),
        "5B" => solutions::day05_b::run(),
        "6A" => solutions::day06_a::run(),
        "6B" => solutions::day06_b::run(),
        "7A" => solutions::day07_a::run(),
        "7B" => solutions::day07_b::run(),
        "8A" => solutions::day08_a::run(),
        "8B" => solutions::day08_b::run(),
        "9A" => solutions::day09_a::run(),
        "9B" => solutions::day09_b::run(),
        "9BB" => solutions::day09_bb::run(),
        "10A" => solutions::day10_a::run(),
        "10B" => solutions::day10_b::run(),
        "11A" => solutions::day11_a::run(),
        "11B" => solutions::day11_b::run(),
        _ => println!("No solution found for {}", CURRENT),
    }
}
