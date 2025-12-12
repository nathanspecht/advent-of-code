// PART 2
use std::{
    collections::{HashMap, HashSet},
    fs,
};

type Pos = (i64, i64, i64);

struct Junc {
    id: usize,
    pos: Pos,
}

impl Junc {
    fn new(id: usize, x: i64, y: i64, z: i64) -> Self {
        Self { id, pos: (x, y, z) }
    }

    fn dist(&self, other: &Junc) -> i64 {
        let dx = other.pos.0 - self.pos.0;
        let dy = other.pos.1 - self.pos.1;
        let dz = other.pos.2 - self.pos.2;

        dx * dx + dy * dy + dz * dz
    }

    fn _print(&self) {
        println!(
            "{} ({}, {}, {})",
            self.id, self.pos.0, self.pos.1, self.pos.2
        );
    }
}

pub fn run() {
    let mut distances: HashMap<(usize, usize), i64> = HashMap::new();
    let mut juncs: HashMap<usize, Junc> = HashMap::new();

    let input = fs::read_to_string("src/inputs/day08_a.txt").expect("Failed to read file");

    for (i, line) in input.lines().enumerate() {
        let sep: Vec<&str> = line.split(',').collect();

        let x: i64 = sep[0].parse().unwrap();
        let y: i64 = sep[1].parse().unwrap();
        let z: i64 = sep[2].parse().unwrap();

        juncs.insert(i, Junc::new(i, x, y, z));
    }

    for (_, junc_a) in &juncs {
        for (_, junc_b) in &juncs {
            if junc_a.id == junc_b.id {
                continue;
            }

            let id_a = junc_a.id.max(junc_b.id);
            let id_b = junc_a.id.min(junc_b.id);

            let dist = junc_a.dist(&junc_b);

            distances.insert((id_a, id_b), dist);
        }
    }

    let mut sets: Vec<HashSet<usize>> = vec![];

    let mut distances: Vec<((usize, usize), i64)> = distances.into_iter().collect();

    distances.sort_by(|a, b| a.1.cmp(&b.1));

    let mut i = 0;

    let mut answer = 0;

    while !(sets.len() == 1 && sets[0].len() == juncs.len()) {
        let ((a, b), _) = distances[i];

        let mut affected_index: Vec<usize> = vec![];
        let mut affected_sets: Vec<HashSet<usize>> = vec![];

        for (x, set) in sets.iter().enumerate() {
            if set.contains(&a) || set.contains(&b) {
                affected_index.push(x);
                affected_sets.push(set.clone());
            }
        }

        affected_index.reverse();

        if affected_index.len() > 0 {
            for index in affected_index {
                sets.remove(index);
            }

            let mut new_set = HashSet::from([a, b]);

            for next_set in affected_sets {
                new_set.extend(next_set);
            }

            sets.push(new_set);
        } else {
            sets.push(HashSet::from([a, b]))
        }

        i += 1;

        if sets.len() == 1 && sets[0].len() == juncs.len() {
            let junc_a = &juncs[&a];
            let junc_b = &juncs[&b];

            answer = junc_a.pos.0 * junc_b.pos.0;
        }
    }

    println!("{:?}", answer);
}
