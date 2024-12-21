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

    let mut updates = input
        .next()
        .unwrap()
        .trim()
        .split("\n")
        .map(|x| x.split(",").map(|x| x.parse::<i32>().unwrap()).collect())
        .collect::<Vec<Vec<i32>>>();

    let mut corrected_updates = vec![];

    for (pos, update) in updates.iter_mut().enumerate() {
        let mut i = 0;

        'update: while i < update.len() {
            let slice = update[0..i].to_vec();

            if let Some(rule) = rules.get(&update[i]) {
                for r in rule {
                    if slice.contains(r) {
                        let j = slice.iter().position(|x| x == r).unwrap();
                        let current = update[i];

                        update.remove(i);
                        update.insert(j, current);

                        if !corrected_updates.contains(&pos) {
                            corrected_updates.push(pos);
                        }

                        i = 0;
                        continue 'update;
                    }
                }
            }

            i += 1;
        }
    }

    for i in corrected_updates {
        let update = &updates[i];
        sum += update[update.len() / 2];
    }

    println!("Sum: {}", sum);
}
