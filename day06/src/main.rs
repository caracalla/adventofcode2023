#[derive(Debug, Default)]
struct Races {
    times: Vec<usize>,
    distances: Vec<usize>,
}

fn parse_input() -> Races {
    let file_contents = std::fs::read_to_string("./input.txt").unwrap();

    let mut lines = file_contents.split('\n');

    fn get_numbers_from_line(line: &str) -> Vec<usize> {
        line.split(':')
            .nth(1)
            .unwrap()
            .split(' ')
            .filter(|time| time.len() > 0)
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    }

    Races {
        times: get_numbers_from_line(lines.nth(0).unwrap()),
        distances: get_numbers_from_line(lines.nth(0).unwrap()),
    }
}

fn ways_to_beat_record(time: usize, record_distance: usize) -> usize {
    let mid_time = time / 2;
    let mut low_time = mid_time;
    let mut high_time = mid_time;

    // find low time
    loop {
        let new_distance = low_time * (time - low_time);
        if new_distance <= record_distance {
            low_time += 1;
            break;
        }
        low_time -= 1;
    }

    // find high time
    loop {
        let new_distance = high_time * (time - high_time);
        if new_distance <= record_distance {
            high_time -= 1;
            break;
        }
        high_time += 1;
    }

    high_time - low_time + 1
}

fn part1(races: &Races) -> usize {
    // start from the middle and walk each way until the product of the times
    // is less than the record
    // the smarter way might be to do a binary search from the middle in both
    // directions

    races
        .times
        .iter()
        .enumerate()
        .map(|(race_index, time)| {
            let record_distance = races.distances[race_index];
            ways_to_beat_record(*time, record_distance)
        })
        .product()
}

fn merge_nums(nums: &Vec<usize>) -> usize {
    nums.iter()
        .map(|num| num.to_string())
        .collect::<Vec<_>>()
        .join("")
        .parse::<_>()
        .unwrap()
}

fn part2(races: &Races) -> usize {
    let new_time = merge_nums(&races.times);
    let new_record_distance = merge_nums(&races.distances);

    ways_to_beat_record(new_time, new_record_distance)
}

fn main() {
    let races = parse_input();
    println!("{races:#?}");
    println!("part 1: {}", part1(&races));
    println!("part 2: {}", part2(&races));
}
