use std::fs;

pub fn run() {
    let input = fs::read_to_string("src/inputs/day07_a.txt").expect("Failed to read file");

    let mut sum = 0;

    input.lines().for_each(|line| {
        let (total, values) = {
            let mut iter = line.trim().split(": ");
            let total = iter.next().unwrap().parse::<i64>().unwrap();
            let values = iter
                .next()
                .unwrap()
                .split(" ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            (total, values)
        };

        if is_viable(total, 0, &values) {
            sum += total;
        }
    });

    println!("{}", sum);
}

fn is_viable(target: i64, current: i64, values: &Vec<i64>) -> bool {
    if values.len() == 0 {
        return target == current;
    }

    let next = values[0];

    let remaining = values[1..].to_vec();

    return is_viable(target, current + next, &remaining)
        || is_viable(target, current * next, &remaining);
}
