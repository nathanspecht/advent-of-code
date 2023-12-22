use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Hand {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let mut total: u32 = 0;
    let file = File::open("./src/input.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let s = line.unwrap();
        let v = s.split(':').collect::<Vec<_>>();

        let mut max = Hand {
            red: 0,
            green: 0,
            blue: 0,
        };

        v[1].split(';').for_each(|group| {
            group.split(',').for_each(|set| {
                let color = set.trim().split(' ').collect::<Vec<_>>()[1].trim();
                let count = set.trim().split(' ').collect::<Vec<_>>()[0]
                    .trim()
                    .parse::<u32>()
                    .unwrap();

                match color {
                    "red" => {
                        if count > max.red {
                            max.red = count
                        }
                    }
                    "blue" => {
                        if count > max.blue {
                            max.blue = count
                        }
                    }
                    "green" => {
                        if count > max.green {
                            max.green = count
                        }
                    }
                    _ => panic!("invalid color recieved"),
                };
            });
        });

        let power = max.red * max.green * max.blue;

        println!("{power}");

        total += power;
    }

    println!("{total}");
}
