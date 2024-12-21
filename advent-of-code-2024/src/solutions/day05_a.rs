use std::{collections::HashMap, fs};

pub fn run() {
    let input = fs::read_to_string("src/inputs/day05_a.txt").expect("Failed to read file");
    let mut input = input.split("\n\n");

    let mut sum = 0;
    let mut rules = HashMap::new();

    input.next().unwrap().split("\n").for_each(|x| {
        let (a, b) = {
            let mut iter = x.split("|").map(|x| x.parse::<i32>().unwrap());
            (iter.next().unwrap(), iter.next().unwrap())
        };

        if rules.contains_key(&a) {
            let rule: &mut Vec<i32> = rules.get_mut(&a).unwrap();
            rule.push(b);
        } else {
            rules.insert(a, vec![b]);
        }
    });

    let updates = input
        .next()
        .unwrap()
        .trim()
        .split("\n")
        .map(|x| x.split(",").map(|x| x.parse::<i32>().unwrap()).collect())
        .collect::<Vec<Vec<i32>>>();

    'updates: for update in updates {
        for i in 0..update.len() {
            let slice = &update[0..i];
            let rule = rules.get(&update[i]).unwrap();

            for r in rule {
                if slice.contains(r) {
                    continue 'updates;
                }
            }
        }

        sum += update[update.len() / 2];
    }

    println!("Sum: {}", sum);
}
