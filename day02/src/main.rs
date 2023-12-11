#[derive(Debug, Default)]
struct Reveal {
    r: usize,
    g: usize,
    b: usize,
}

#[derive(Debug)]
struct Game {
    reveals: Vec<Reveal>,
    id: usize,
}

fn parse_input() -> Vec<Game> {
    let file_contents = std::fs::read_to_string("./input.txt").unwrap();

    file_contents.split('\n').map(|line| {
        let mut reveals = Vec::<Reveal>::new();
        let mut prev_num: String = Default::default();
        let mut id = 0;
        let mut last_reveal: Reveal = Default::default();

        for ch in line.chars() {
            if ch.is_numeric() {
                prev_num.push(ch);
            }

            match ch {
                ':' => {
                    id = prev_num.parse::<usize>().unwrap();
                    prev_num = Default::default();
                }
                ';' => {
                    reveals.push(last_reveal);
                    last_reveal = Default::default();
                }
                'd' => { // r appears in "green", but d is unique to "red"
                    last_reveal.r = prev_num.parse::<usize>().unwrap();
                    prev_num = Default::default();
                }
                'g' => {
                    last_reveal.g = prev_num.parse::<usize>().unwrap();
                    prev_num = Default::default();
                }
                'b' => {
                    last_reveal.b = prev_num.parse::<usize>().unwrap();
                    prev_num = Default::default();
                }
                _ => (),

            };
        }
        
        // final reveal has no semicolon delimiter
        reveals.push(last_reveal);

        Game { reveals, id }
    }).collect()
}

fn part1(games: &[Game]) -> usize {
    let desired_red = 12;
    let desired_green = 13;
    let desired_blue = 14;

    let mut sum = 0;

    for game in games {
        let mut is_viable = true;

        for reveal in game.reveals.iter() {
            if reveal.r > desired_red || reveal.g > desired_green || reveal.b > desired_blue {
                is_viable = false;
            } else {
                // println!("{reveal:?}");
            }
        }

        if is_viable {
            sum += game.id;
        }
    }

    sum
}

fn part2(games: &[Game]) -> usize {
    games.iter().map(|game| {
        let mut maxes: Reveal = Default::default();

        for reveal in game.reveals.iter() {
            if reveal.r > maxes.r {
                maxes.r = reveal.r;
            }

            if reveal.g > maxes.g {
                maxes.g = reveal.g;
            }

            if reveal.b > maxes.b {
                maxes.b = reveal.b;
            }
        }

        maxes.r * maxes.g * maxes.b
    }).sum()
}

fn main() {
    let games = parse_input();
    println!("part 1: {}", part1(&games));
    println!("part 2: {}", part2(&games));
}
