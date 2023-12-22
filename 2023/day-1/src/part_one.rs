use std::fs;

pub fn part_one() {
    let mut total: u32 = 0;
    let mut first_digit: Option<char> = None;
    let mut last_digit: Option<char> = None;

    fs::read_to_string("./src/input.txt")
        .expect("Should read file")
        .chars()
        .for_each(|x| {
            if x == 0xA as char {
                let mut num_str = String::new();

                num_str.push(first_digit.expect("first digit to be string"));
                num_str.push(last_digit.expect("second digit to be string"));

                let num = num_str.parse::<u32>().expect("to parse u32");

                total += num;

                first_digit = None;
                last_digit = None;
            } else if x.is_ascii_digit() {
                if first_digit.is_none() {
                    first_digit = Some(x);
                }

                last_digit = Some(x);
            }
        });

    println!("{total}")
}
