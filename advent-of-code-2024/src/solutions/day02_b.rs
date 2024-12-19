use std::fs;

pub fn run() {
    let mut num_safe: i32 = 0;

    fs::read_to_string("src/inputs/day02_b.txt")
        .expect("Failed to read file")
        .lines()
        .for_each(|line| {
            let arr = line
                .split(" ")
                .collect::<Vec<_>>()
                .iter()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            let mut is_safe = false;

            for i in 0..arr.len() {
                let mut temp = arr.clone();
                temp.remove(i);

                if check_safety(temp) {
                    is_safe = true;
                    break;
                }
            }

            if is_safe {
                num_safe += 1;
            }
        });

    println!("Num safe: {}", num_safe);
}

fn check_safety(arr: Vec<i32>) -> bool {
    let mut is_safe = true;
    let mut dir = 0;

    for i in 0..arr.len() - 1 {
        let a = arr[i];
        let b = arr[i + 1];

        if (a - b).abs() > 3 || (a - b).abs() == 0 {
            is_safe = false;
            break;
        }

        let current_dir = (a - b) / (a - b).abs();

        if dir == 0 {
            dir = current_dir;
        } else if dir != current_dir {
            is_safe = false;
            break;
        }
    }

    is_safe
}
