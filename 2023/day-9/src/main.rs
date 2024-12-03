use std::fs;

fn main() {
    let part = 2;

    let input = fs::read_to_string("./src/input.txt").expect("to read file");
    let total: i64 = input
        .lines()
        .map(|line| {
            let input = line
                .split(' ')
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            if part == 1 {
                get_forward_prediction(&input)
            } else {
                get_backwards_prediction(&input)
            }
        })
        .sum();

    println!("{total}");
}

fn get_forward_prediction(input: &[i64]) -> i64 {
    let mut last_values: Vec<i64> = vec![*input.last().unwrap()];
    let mut curr_arr = input.to_vec();
    let mut next_arr = vec![];

    let mut i = 1;
    let mut all_zero = false;

    while !all_zero {
        all_zero = true;

        while i < curr_arr.len() {
            let next_value = curr_arr[i] - curr_arr[i - 1];
            next_arr.push(next_value);

            if next_value != 0 {
                all_zero = false;
            }

            if i == curr_arr.len() - 1 {
                last_values.push(next_value);
            }

            i += 1;
        }

        curr_arr = next_arr.clone();
        next_arr = vec![];
        i = 1;
    }

    last_values.iter().sum()
}

fn get_backwards_prediction(input: &[i64]) -> i64 {
    let mut first_values: Vec<i64> = vec![*input.first().unwrap()];
    let mut curr_arr = input.to_vec();
    let mut next_arr = vec![];

    let mut i = 1;
    let mut all_zero = false;

    while !all_zero {
        all_zero = true;

        while i < curr_arr.len() {
            let next_value = curr_arr[i] - curr_arr[i - 1];
            next_arr.push(next_value);

            if next_value != 0 {
                all_zero = false;
            }

            if i == 1 {
                first_values.push(next_value);
            }

            i += 1;
        }

        curr_arr = next_arr.clone();
        next_arr = vec![];
        i = 1;
    }

    let mut sum: i64 = 0;

    first_values.reverse();

    for value in first_values {
        sum = value - sum;
    }

    sum
}
