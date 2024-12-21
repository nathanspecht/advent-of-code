mod solutions;

const CURRENT: &str = "5A";

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
        _ => println!("No solution found for {}", CURRENT),
    }
}
