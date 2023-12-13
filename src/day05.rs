use std::fs;

fn main() {
    let data = fs::read_to_string("data/day05.txt").unwrap();
    println!("day05_part01: {}", day05_part01(&data));
    println!("day05_part02: {}", day05_part02(&data));
}

fn day05_part01(data: &str) -> usize {
    let mappings = data.split("\n\n").collect::<Vec<&str>>();

    // Build mappings once for doing lookup instead of building it for every value.
    let seed_to_soil_map = parse_ranges(mappings[1]);
    let soil_to_fertilizer_map = parse_ranges(mappings[2]);
    let fertilizer_to_water_map = parse_ranges(mappings[3]);
    let water_to_light_map = parse_ranges(mappings[4]);
    let light_to_temperature_map = parse_ranges(mappings[5]);
    let temperature_to_humidity_map = parse_ranges(mappings[6]);
    let humidity_to_location_map = parse_ranges(mappings[7]);

    mappings[0]
        .split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .map(str::parse)
        .filter_map(std::result::Result::ok)
        .map(|n| if_exists(n, &seed_to_soil_map))
        .map(|n| if_exists(n, &soil_to_fertilizer_map))
        .map(|n| if_exists(n, &fertilizer_to_water_map))
        .map(|n| if_exists(n, &water_to_light_map))
        .map(|n| if_exists(n, &light_to_temperature_map))
        .map(|n| if_exists(n, &temperature_to_humidity_map))
        .map(|n| if_exists(n, &humidity_to_location_map))
        .min()
        .unwrap()
}

fn day05_part02(data: &str) -> usize {
    let mappings = data.split("\n\n").collect::<Vec<&str>>();

    // Build mappings once for doing lookup instead of building it for every value.
    let seed_to_soil_map = parse_ranges(mappings[1]);
    let soil_to_fertilizer_map = parse_ranges(mappings[2]);
    let fertilizer_to_water_map = parse_ranges(mappings[3]);
    let water_to_light_map = parse_ranges(mappings[4]);
    let light_to_temperature_map = parse_ranges(mappings[5]);
    let temperature_to_humidity_map = parse_ranges(mappings[6]);
    let humidity_to_location_map = parse_ranges(mappings[7]);

    let seeds = mappings[0]
        .split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .map(str::parse)
        .filter_map(std::result::Result::ok)
        .collect::<Vec<usize>>();

    seeds
        .chunks(2)
        .map(|s| {
            (s[0]..(s[0] + s[1] - 1))
                .map(|n| if_exists(n, &seed_to_soil_map))
                .map(|n| if_exists(n, &soil_to_fertilizer_map))
                .map(|n| if_exists(n, &fertilizer_to_water_map))
                .map(|n| if_exists(n, &water_to_light_map))
                .map(|n| if_exists(n, &light_to_temperature_map))
                .map(|n| if_exists(n, &temperature_to_humidity_map))
                .map(|n| if_exists(n, &humidity_to_location_map))
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

fn if_exists(key: usize, ranges: &Vec<(usize, usize, usize)>) -> usize {
    for (src, dst, length) in ranges {
        if key >= *src && key < *src + *length {
            return *dst + (key - *src);
        }
    }
    key
}

fn parse_ranges(lookup: &str) -> Vec<(usize, usize, usize)> {
    lookup
        .split('\n')
        .skip(1)
        .filter(|s| !s.is_empty())
        .map(|s| {
            let dest_src_split = s
                .split(' ')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            (dest_src_split[1], dest_src_split[0], dest_src_split[2])
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day05_part01() {
        let data = r###"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"###;

        assert_eq!(day05_part01(data), 35);
    }

    #[test]
    fn test_day05_part02() {
        let data = r###"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"###;

        assert_eq!(day05_part02(data), 46);
    }
}
