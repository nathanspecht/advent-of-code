use std::fs;

static DIGIT_STRINGS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part_two() {
    let mut total: u32 = 0;
    let mut first_digit: Option<char> = None;
    let mut last_digit: Option<char> = None;
    let mut current_digit_string = String::new();

    let input: Vec<char> = fs::read_to_string("./src/input_2.txt")
        .expect("Should read file")
        .chars()
        .collect();

    let mut i = 0;

    while i < input.len() {
        let x = input[i];

        // 0xA is newline
        if x == 0xA as char {
            let mut num_str = String::new();

            num_str.push(first_digit.expect("first digit to be string"));
            num_str.push(last_digit.expect("second digit to be string"));

            let num = num_str.parse::<u32>().expect("to parse u32");

            total += num;

            first_digit = None;
            last_digit = None;

            current_digit_string = String::new();
        } else if x.is_ascii_digit() {
            if first_digit.is_none() {
                first_digit = Some(x);
            }

            last_digit = Some(x);

            current_digit_string = String::new();
        } else {
            current_digit_string.push(x);

            let digit = get_digit(&current_digit_string);

            if digit.is_some() {
                if first_digit.is_none() {
                    first_digit = digit;
                }

                last_digit = digit;

                i -= current_digit_string.len() - 1;
                current_digit_string = String::new();
            } else if !is_digit_string(&current_digit_string) {
                i -= current_digit_string.len() - 1;
                current_digit_string = String::new();
            }
        }

        i += 1;
    }

    println!("{total}")
}

fn is_digit_string(input: &str) -> bool {
    for digit_string in DIGIT_STRINGS {
        if digit_string.starts_with(input) {
            return true;
        }
    }

    false
}

fn get_digit(digit_string: &str) -> Option<char> {
    match digit_string {
        "one" => Some('1'),
        "two" => Some('2'),
        "three" => Some('3'),
        "four" => Some('4'),
        "five" => Some('5'),
        "six" => Some('6'),
        "seven" => Some('7'),
        "eight" => Some('8'),
        "nine" => Some('9'),
        _ => None,
    }
}
