use std::fs;

pub fn run() {
    let mut count: u64 = 0;
    let mut fresh: Vec<(u64, u64)> = vec![];

    let input = fs::read_to_string("src/inputs/day05_a.txt").expect("Failed to read file");

    match input.split_once("\n\n") {
        Some((top, _bottom)) => {
            top.split('\n').for_each(|row| {
                let (min, max) = row.split_once('-').unwrap();
                let min: u64 = min.parse().unwrap();
                let max: u64 = max.parse().unwrap();

                fresh.push((min, max));
            });
        }
        _ => {}
    }

    let mut i = 0;

    while i < fresh.len() {
        let overlaps = find_overlaps(&fresh, i);

        if overlaps.len() == 0 {
            i += 1;
            continue;
        }

        let mut overlap_ranges: Vec<(u64, u64)> = overlaps.iter().map(|j| fresh[*j]).collect();
        overlap_ranges.push(fresh[i]);

        let new_range = combine_overlaps(overlap_ranges);

        fresh[i] = new_range;

        for overlap in overlaps {
            fresh.remove(overlap);
        }

        i = 0;
    }

    for (min, max) in &fresh {
        count += max - min + 1
    }

    println!("{:?}", count);
}

fn combine_overlaps(input: Vec<(u64, u64)>) -> (u64, u64) {
    let min = input.iter().map(|(min, _)| min).min().unwrap();
    let max = input.iter().map(|(_, max)| max).max().unwrap();

    return (*min, *max);
}

fn find_overlaps(input: &Vec<(u64, u64)>, i: usize) -> Vec<usize> {
    let (min, max) = input[i];
    let mut overlaps: Vec<usize> = vec![];

    for (j, (cmp_min, cmp_max)) in input.iter().enumerate() {
        if i == j {
            continue;
        }

        if (min <= *cmp_min && max >= *cmp_min)
            || (min <= *cmp_max && max >= *cmp_max)
            || (min >= *cmp_min && max <= *cmp_max)
        {
            overlaps.push(j);
        }
    }

    overlaps.sort_by(|a, b| b.cmp(a));

    return overlaps;
}
