mod solutions;

const CURRENT: &str = "1A";

fn main() {
    match CURRENT {
        "1A" => solutions::day01_a::run(),
        "1B" => solutions::day01_b::run(),
        _ => println!("No solution found for {}", CURRENT),
    }
}
