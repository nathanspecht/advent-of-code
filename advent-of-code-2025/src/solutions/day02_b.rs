use std::fs;

pub fn run() {
    let mut total: u64 = 0;
    let input = fs::read_to_string("src/inputs/day02_a.txt").expect("Failed to read file");

    let ranges: Vec<&str> = input.trim().split(',').collect();

    for range in &ranges {
        let (start, end) = range
            .split_once('-')
            .map(|(a, b)| (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()))
            .unwrap();

        'range: for x in start..=end {
            let id = x.to_string();
            let len = id.chars().count();

            let min_groups = 2;
            let max_groups = len;

            for num_groups in min_groups..=max_groups {
                if len % num_groups != 0 {
                    continue;
                }

                let group_len = len / num_groups;

                let mut groups: Vec<&str> = vec![];

                for i in 0..num_groups {
                    let start = i * group_len;
                    let end = start + group_len;

                    groups.push(&id[start..end]);
                }

                if groups.iter().all(|group| group == &groups[0]) {
                    total += x;
                    continue 'range;
                }
            }
        }
    }

    println!("{total}");
}
