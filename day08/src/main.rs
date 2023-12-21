use std::collections::HashMap;

#[derive(Debug)]
struct NetEntry {
    left: String,
    right: String,
}

#[derive(Debug)]
struct NetMap {
    directions: Vec<char>,
    entries: HashMap<String, NetEntry>,
}

fn parse_input() -> NetMap {
    let file_contents = std::fs::read_to_string("./input.txt").unwrap();
    let mut lines = file_contents.split('\n');
    let directions = lines.nth(0).unwrap().chars().collect::<Vec<_>>();

    let mut entries = HashMap::new();

    for line in lines {
        if line.len() == 0 {
            continue;
        }

        let mut parts = line.split(' ');
        let key = parts.nth(0).unwrap().to_string();
        let left = parts.nth(1).unwrap()[1..4].to_string();
        let right = parts.nth(0).unwrap()[0..3].to_string();

        entries.insert(key, NetEntry { left, right });
    }

    NetMap {
        directions,
        entries,
    }
}

fn part1(map: &NetMap) -> usize {
    let mut steps = 0;
    let mut next_step = "AAA".to_string();

    while next_step != "ZZZ" {
        for direction in map.directions.iter() {
            if next_step == "ZZZ" {
                return steps;
            }

            let entry = map.entries.get(&next_step).unwrap();

            next_step = match direction {
                'L' => entry.left.clone(),
                'R' => entry.right.clone(),
                _ => unreachable!(),
            };

            steps += 1;
        }
    }

    steps
}

fn all_zs(next_steps: &[String]) -> bool {
    for next_step in next_steps {
        if next_step.chars().nth(2).unwrap() != 'Z' {
            return false;
        }
    }

    true
}

fn part2(map: &NetMap) -> usize {
    let mut steps = 0;
    let mut next_steps = map
        .entries
        .keys()
        .filter(|key| key.chars().nth(2).unwrap() == 'A')
        .map(|key| key.clone())
        .collect::<Vec<_>>();

    while !all_zs(&next_steps) {
        for direction in map.directions.iter() {
            if all_zs(&next_steps) {
                return steps;
            }

            for i in 0..next_steps.len() {
                let entry = map.entries.get(&next_steps[i]).unwrap();

                next_steps[i] = match direction {
                    'L' => entry.left.clone(),
                    'R' => entry.right.clone(),
                    _ => unreachable!(),
                }
            }

            steps += 1;
        }
    }

    steps
}

fn main() {
    let map = parse_input();
    println!("{map:#?}");
    println!("part 1: {}", part1(&map));
    println!("part 2: {}", part2(&map));
}
