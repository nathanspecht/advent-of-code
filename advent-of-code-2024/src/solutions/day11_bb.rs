use std::fs;

const ITERATIONS: usize = 75;

struct State {
    zero: u64,
    odd: u64,
}

pub fn run() {
    let input = fs::read_to_string("src/inputs/day11_a.txt").expect("Failed to read file");
    let stones = input.trim().split(' ').collect::<Vec<_>>();

    let mut evens = vec![];

    // Initialize state counts
    let mut state = State { zero: 0, odd: 0 };

    for stone in stones {
        let value = stone.parse::<u64>().unwrap();
        if value == 0 {
            state.zero += 1;
        } else if stone.len() % 2 == 1 {
            state.odd += 1;
        } else {
            evens.push(stone);
        }
    }

    // Simulate iterations
    for _ in 0..ITERATIONS {
        let new_odd = state.zero; // 0 → 1 (odd)
        let new_even = state.odd; // odd → even
        let (split_odd, split_even) = split_evens(state.even);

        state = State {
            zero: 0,
            odd: new_odd + split_odd,
        };
    }

    println!("Result: {}", state.odd + evens.len() as u64);
}

// Precompute splitting behavior for even-length stones
fn split_evens(even_count: u64) -> (u64, u64) {
    // Assume even-length stones split into two odd-length stones after 1 iteration
    // Adjust this based on your input's actual even-length patterns
    (even_count * 2, 0)
}
