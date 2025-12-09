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

    for i in 0..split[0].len() {
        match split[split.len() - 1][i] {
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

        let column: Vec<char> = split[..split.len() - 1].iter().map(|row| row[i]).collect();

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
    }

    total += current;

    println!("{total}");
}
