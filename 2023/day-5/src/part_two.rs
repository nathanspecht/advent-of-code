use crate::conversion_map::ConversionMap;
use crate::seed::Seed;
use crate::util::parse_input;

pub fn run() {
    let mut seeds: Vec<Seed> = vec![];
    let mut maps: Vec<Vec<ConversionMap>> = vec![];

    parse_input(&mut seeds, &mut maps);

    let mut min_location: Option<u64> = None;

    for seed in seeds {
        let converted_seeds = map_seed_locations(0, &maps, vec![seed]);

        for converted_seed in converted_seeds {
            min_location = match min_location {
                None => Some(converted_seed.start),
                Some(i) => {
                    if converted_seed.start < i {
                        Some(converted_seed.start)
                    } else {
                        Some(i)
                    }
                }
            }
        }
    }

    let actual = min_location.unwrap();

    println!("{actual}");
}

fn map_seed_locations(
    index: usize,
    map_groups: &Vec<Vec<ConversionMap>>,
    seeds: Vec<Seed>,
) -> Vec<Seed> {
    let mut converted_seeds: Vec<Seed> = vec![];
    let mut extra_seeds: Vec<Seed> = vec![];

    let maps = &map_groups[index];

    for mut seed in seeds {
        for map in maps {
            if !seed.is_empty() {
                let (converted_seed, extra_seed) = map.get_overlapping_seed(&mut seed);

                match converted_seed {
                    None => {}
                    Some(i) => converted_seeds.push(i),
                }

                match extra_seed {
                    None => {}
                    Some(i) => extra_seeds.push(i),
                }
            }
        }

        if !seed.is_empty() {
            converted_seeds.push(seed)
        }
    }

    for mut seed in extra_seeds {
        for map in maps {
            if !seed.is_empty() {
                let (converted_seed, extra_seed) = map.get_overlapping_seed(&mut seed);

                match converted_seed {
                    None => {}
                    Some(i) => converted_seeds.push(i),
                }

                match extra_seed {
                    None => {}
                    Some(i) => converted_seeds.push(i),
                }
            }
        }

        if !seed.is_empty() {
            converted_seeds.push(seed)
        }
    }

    if index + 1 >= map_groups.len() {
        return converted_seeds;
    }

    map_seed_locations(index + 1, map_groups, converted_seeds)
}
