use crate::conversion_map::ConversionMap;
use crate::seed::Seed;
use crate::util::parse_input;

pub fn run() {
    let mut seeds: Vec<Seed> = vec![];
    let mut maps: Vec<Vec<ConversionMap>> = vec![];

    parse_input(&mut seeds, &mut maps);

    let mut min_location: Option<u64> = None;

    for seed in seeds {
        for value in seed.get_range() {
            let mut current = value;

            for map_group in &mut maps {
                for map in map_group {
                    if map.is_in_range(current) {
                        current = map.convert(current);
                        break;
                    }
                }
            }

            match min_location {
                None => min_location = Some(current),
                Some(i) => {
                    if current < i {
                        min_location = Some(current)
                    }
                }
            }
        }
    }

    let actual = min_location.unwrap();

    println!("{actual}");
}
