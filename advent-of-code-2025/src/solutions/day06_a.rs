use std::fs;

pub fn run() {
    let mut total: u64 = 0;
    let input = fs::read_to_string("src/inputs/day06_a.txt").expect("Failed to read file");
    let split: Vec<Vec<&str>> = input
        .split('\n')
        .into_iter()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect();

    let operators: Vec<char> = split[4].iter().map(|s| s.chars().next().unwrap()).collect();

    let operands: Vec<Vec<u64>> = split[0..4]
        .iter()
        .map(|row| row.iter().map(|i| i.parse().unwrap()).collect())
        .collect();

    for (i, operator) in operators.iter().enumerate() {
        let mut amount = operands[0][i];

        for j in 1..4 {
            match operator {
                '*' => amount *= operands[j][i],
                '+' => amount += operands[j][i],
                _ => {}
            }
        }

        total += amount;
    }

    println!("{total}");
}
