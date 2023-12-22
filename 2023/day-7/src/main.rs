use std::{cmp::Ordering, collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("to read file");
    let lines = input.lines().collect::<Vec<_>>();

    let mut hand_bids = lines
        .iter()
        .map(|line| {
            let mut parts = line.split(' ');
            (
                parts.next().unwrap(),
                parts.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    hand_bids.sort_by(|(hand_one, _), (hand_two, _)| {
        let hand_rank_one = get_hand_rank(hand_one);
        let hand_rank_two = get_hand_rank(hand_two);

        if hand_rank_one == hand_rank_two {
            return compare_high_card_ranks(hand_one, hand_two);
        }

        hand_rank_one.cmp(&hand_rank_two)
    });

    let sorted_bids = hand_bids.iter().enumerate().collect::<Vec<_>>();

    let mut total = 0;

    for (index, (hand, bid)) in sorted_bids {
        println!("{} {}", hand, index);
        total += bid * (index + 1) as u32;
    }

    println!("{}", total);
}

fn compare_high_card_ranks(hand_one: &str, hand_two: &str) -> Ordering {
    let mut cmp: Ordering = Ordering::Equal;

    for index in 0..hand_one.len() {
        let rank_one = get_high_card_rank(hand_one, index);
        let rank_two = get_high_card_rank(hand_two, index);

        if rank_one == rank_two {
            continue;
        }

        if rank_one > rank_two {
            cmp = Ordering::Greater;
            break;
        }

        if rank_one < rank_two {
            cmp = Ordering::Less;
            break;
        }
    }

    cmp
}

fn get_high_card_rank(hand: &str, index: usize) -> u32 {
    let ranks = [
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ];

    ranks
        .iter()
        .position(|x| x == &hand.chars().nth(index).unwrap())
        .unwrap() as u32
}

fn get_hand_rank(hand: &str) -> u32 {
    let five_of_a_kind: Vec<u32> = vec![5];
    let four_of_a_kind: Vec<u32> = vec![4, 1];
    let full_house: Vec<u32> = vec![3, 2];
    let three_of_a_kind: Vec<u32> = vec![3, 1, 1];
    let two_pair: Vec<u32> = vec![2, 2, 1];
    let one_pair: Vec<u32> = vec![2, 1, 1, 1];
    let high_card: Vec<u32> = vec![1, 1, 1, 1, 1];

    let hand_order = vec![
        high_card,
        one_pair,
        two_pair,
        three_of_a_kind,
        full_house,
        four_of_a_kind,
        five_of_a_kind,
    ];

    let matches = count_matches(hand);

    hand_order.iter().position(|x| x == &matches).unwrap() as u32
}

fn count_matches(hand: &str) -> Vec<u32> {
    let mut counts: HashMap<char, u32> = HashMap::new();

    for c in hand.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    let j_count = *counts.get(&'J').unwrap_or(&0);

    counts.remove(&'J');

    let mut count_vec = counts.values().cloned().collect::<Vec<_>>();

    count_vec.sort_by(|a, b| b.cmp(a));

    let mut added_j = false;

    let mut with_added_j = count_vec
        .iter()
        .map(|count| {
            if !added_j && count + j_count <= 5 {
                added_j = true;
                return *count + j_count;
            }

            *count
        })
        .collect::<Vec<_>>();

    let sum: u32 = with_added_j.iter().sum();

    if sum < 5 {
        with_added_j.push(5 - sum);
    }

    with_added_j.sort_by(|a, b| b.cmp(a));

    with_added_j
}
