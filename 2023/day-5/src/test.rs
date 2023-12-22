#[cfg(test)]
mod tests {
    use crate::conversion_map::ConversionMap;
    use crate::seed::Seed;

    #[test]
    fn overlapping_seeds() {
        let mut seed = Seed::new(1, 10);
        let map = ConversionMap::new("11 5 3");

        let (converted_seed, extra_seed) = map.get_overlapping_seed(&mut seed);

        assert_eq!(seed.start, 1);
        assert_eq!(seed.range, 4);

        assert_eq!(converted_seed.unwrap().start, 11);
        assert_eq!(converted_seed.unwrap().range, 3);

        assert_eq!(extra_seed.unwrap().start, 8);
        assert_eq!(extra_seed.unwrap().range, 3);
    }

    #[test]
    fn overlapping_seeds_two() {
        let mut seed = Seed::new(1, 10);
        let map = ConversionMap::new("5 5 6");

        let (converted_seed, extra_seed) = map.get_overlapping_seed(&mut seed);

        assert!(extra_seed.is_none());
        assert_eq!(converted_seed.unwrap().start, 5);
        assert_eq!(converted_seed.unwrap().range, 6);
        assert_eq!(seed.start, 1);
        assert_eq!(seed.range, 4);
    }

    #[test]
    fn overlapping_seeds_three() {
        let mut seed = Seed::new(1, 10);
        let map = ConversionMap::new("10 1 5");

        let (converted_seed, extra_seed) = map.get_overlapping_seed(&mut seed);

        assert!(extra_seed.is_none());
        assert_eq!(converted_seed.unwrap().start, 10);
        assert_eq!(converted_seed.unwrap().range, 5);
        assert_eq!(seed.start, 6);
        assert_eq!(seed.range, 4);
    }
}
