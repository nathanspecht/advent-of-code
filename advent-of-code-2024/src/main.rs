mod solutions;

const CURRENT: &str = "1B";

fn main() {
    match CURRENT {
        "1A" => solutions::day01_a::run(),
        "1B" => solutions::day01_b::run(),
        "2A" => solutions::day02_a::run(),
        "2B" => solutions::day02_b::run(),
        _ => println!("No solution found for {}", CURRENT),
    }
}
