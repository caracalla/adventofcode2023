use std::ops::Range;

#[derive(Debug, Default)]
struct CategoryPair {
    source: Range<usize>,
    dest: Range<usize>,
}

#[derive(Debug, Default)]
struct Input {
    seeds: Vec<usize>,
    maps: [Vec<CategoryPair>; 7],
}

fn parse_input() -> Input {
    let file_contents = std::fs::read_to_string("./input.txt").unwrap();

    let mut input: Input = Default::default();
    // let mut categories_to_set = &mut input.seed_to_soils;
    let mut map_index_to_set = 0;

    for (line_index, line) in file_contents.split('\n').enumerate() {
        if line.len() == 0 {
            // skip empty lines
            continue;
        }

        if line_index == 0 {
            // this is the seeds line
            input.seeds = line
                .split(' ')
                .skip(1)
                .map(|part| part.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            continue;
        }

        if line.chars().nth(line.len() - 1).unwrap() == ':' {
            // map name line
            let map_name = line.split(' ').nth(0).unwrap();

            match map_name {
                "seed-to-soil" => map_index_to_set = 0,
                "soil-to-fertilizer" => map_index_to_set = 1,
                "fertilizer-to-water" => map_index_to_set = 2,
                "water-to-light" => map_index_to_set = 3,
                "light-to-temperature" => map_index_to_set = 4,
                "temperature-to-humidity" => map_index_to_set = 5,
                "humidity-to-location" => map_index_to_set = 6,
                _ => unreachable!(),
            }
        } else {
            // map entry line
            let numbers = line
                .split(' ')
                .map(|part| part.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            assert!(
                numbers.len() == 3,
                "expect exactly 3 numbers for a map entry"
            );
            let category_pair = CategoryPair {
                source: Range {
                    start: numbers[1],
                    end: numbers[1] + numbers[2],
                },
                dest: Range {
                    start: numbers[0],
                    end: numbers[0] + numbers[2],
                },
            };
            input.maps[map_index_to_set].push(category_pair);
        }
    }

    input
}

fn get_location_for_seed(seed: usize, input: &Input) -> usize {
    let mut value = seed;

    for category_pairs in input.maps.iter() {
        for pair in category_pairs.iter() {
            if pair.source.contains(&value) {
                // update value
                let offset = value - pair.source.start;
                value = pair.dest.start + offset;
                break;
            }
        }
    }

    value
}

fn part1(input: &Input) -> usize {
    let mut min_location = usize::MAX;

    for seed in input.seeds.iter() {
        let location = get_location_for_seed(*seed, input);
        if location < min_location {
            min_location = location;
        }
    }

    min_location
}

fn part2(input: &Input) -> usize {
    let mut min_location = usize::MAX;
    let seed_ranges = input
        .seeds
        .chunks(2)
        .map(|pair| Range {
            start: pair[0],
            end: pair[0] + pair[1],
        })
        .collect::<Vec<_>>();

    let seed_count = seed_ranges.iter().map(|range| range.len()).sum::<usize>();
    let mut processed_count = 0;
    let percentages = (0..100)
        .map(|pct| (seed_count as f64 * 0.01 * pct as f64) as usize)
        .collect::<Vec<usize>>();
    let mut pct_index = 0;

    for range in seed_ranges {
        // /*
        // brute force solution (get ready to wait)
        for item in range {
            if pct_index < percentages.len() && processed_count == percentages[pct_index] {
                println!("{pct_index}% done");
                pct_index += 1;
            }

            let location = get_location_for_seed(item, input);
            if location < min_location {
                min_location = location;
            }

            processed_count += 1;
        }
        // */
    }

    min_location
}

fn main() {
    let input = parse_input();
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}
