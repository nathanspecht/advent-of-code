use std::fs;

pub fn run() {
    let mut total: u64 = 0;
    let input = fs::read_to_string("src/inputs/day06_a.txt").expect("Failed to read file");

    let split: Vec<Vec<char>> = input
        .split('\n')
        .into_iter()
        .map(|line| line.chars().collect())
        .filter(|line: &Vec<char>| line.len() > 0)
        .collect();

    let mut operator = '+';
    let mut current = 0;

    let operator_index = split.len() - 1;

    for i in 0..split[0].len() {
        match split[operator_index][i] {
            '*' => {
                operator = '*';
                current = 1;
            }
            '+' => {
                operator = '+';
                current = 0;
            }
            _ => {}
        }

        let column = vec![split[0][i], split[1][i], split[2][i], split[3][i]];

        if column.iter().all(|x| *x == ' ') {
            total += current;
            current = 0;
            continue;
        }

        let val: u64 = column
            .iter()
            .collect::<String>()
            .trim()
            .parse()
            .unwrap_or_else(|_err| {
                println!("{:?}", column);
                return 0;
            });

        match operator {
            '*' => current *= val,
            '+' => current += val,
            _ => {}
        }

        if i == split[0].len() - 1 {
            total += current;
        }
    }

    println!("{total}");
}
