fn parse_input() -> Vec<Vec<char>> {
    let file_contents = std::fs::read_to_string("./input.txt").unwrap();

    let mut chars = vec![];

    for line in file_contents.split('\n') {
        if line.len() == 0 {
            continue;
        }

        chars.push(line.chars().collect());
    }

    chars
}

// used for numbers above/below a character
fn get_numbers_from_line(line: &Vec<char>, ch_index: usize) -> Vec<usize>{
    if line[ch_index].is_numeric() {
        // there is a number here
        let mut i = ch_index;
        let mut number: String = Default::default();

        // first, walk back to the beginning of the number
        while i > 0 {
            i -= 1;
            if !line[i].is_numeric() {
                i += 1;
                break;
            }
        }

        while i < line.len() && line[i].is_numeric() {
            number.push(line[i]);
            i += 1;
        }

        vec![number.parse::<usize>().unwrap()]
    } else {
        // we need to check diagonals
        // there could be both a left and right diagonal!
        let mut result = vec![];

        if ch_index > 0 && line[ch_index - 1].is_numeric() {
            // left diagonal hit
            let mut i = ch_index - 1;
            let mut number: String = Default::default();

            // first, walk back to the beginning of the number
            while i > 0 {
                i -= 1;
                if !line[i].is_numeric() {
                    i += 1;
                    break;
                }
            }

            while i < line.len() && line[i].is_numeric() {
                number.push(line[i]);
                i += 1;
            }

            result.push(number.parse::<usize>().unwrap());
        }

        if ch_index < line.len() - 1 && line[ch_index + 1].is_numeric() {
            // right diagonal hit
            let mut i = ch_index + 1;
            let mut number: String = Default::default();

            while i < line.len() && line[i].is_numeric() {
                number.push(line[i]);
                i += 1;
            }

            result.push(number.parse::<usize>().unwrap());
        }

        result
    }
}

struct SymbolNumbers {
    symbol: char,
    numbers: Vec<usize>,
}

fn get_symbol_numbers(chars: &Vec<Vec<char>>) -> Vec<SymbolNumbers> {
    let mut sn_vec = vec![];

    for (line_index, line) in chars.iter().enumerate() {
        for (ch_index, ch) in line.iter().enumerate() {
            let mut sn = SymbolNumbers { symbol: *ch, numbers: vec![] };

            if *ch == '.' || ch.is_numeric() {
                continue;
            }

            // check to the right
            if ch_index < line.len() - 1 && line[ch_index + 1].is_numeric() {
                let mut i = ch_index + 1;
                let mut number: String = Default::default();

                while i < line.len() && line[i].is_numeric() {
                    number.push(line[i]);
                    i += 1;
                }

                sn.numbers.push(number.parse::<usize>().unwrap());
            }

            // return sum;

            // check to the left
            if ch_index > 0 && line[ch_index - 1].is_numeric() {
                // let mut i = ch_index - 1;
                let mut number: String = Default::default();

                for i in (0..ch_index).rev() {
                    if !line[i].is_numeric() {
                        break;
                    }

                    number.insert(0, line[i]);
                }

                sn.numbers.push(number.parse::<usize>().unwrap());
            }

            // check above
            if line_index > 0 {
                let prev_line = &chars[line_index - 1];
                let mut nums = get_numbers_from_line(prev_line, ch_index);
                sn.numbers.append(&mut nums);
            }

            // check below
            if line_index < chars.len() - 1 {
                let next_line = &chars[line_index + 1];
                let mut nums = get_numbers_from_line(next_line, ch_index);
                sn.numbers.append(&mut nums);
            }

            sn_vec.push(sn);
        }
    }

    sn_vec
}

fn part1(sn_vec: &Vec<SymbolNumbers>) -> usize {
    sn_vec.iter().map(|sn| sn.numbers.iter().sum::<usize>()).sum::<usize>()
}

fn part2(sn_vec: &Vec<SymbolNumbers>) -> usize {
    sn_vec.iter().map(|sn| {
        if sn.symbol == '*' && sn.numbers.len() == 2 {
            sn.numbers.iter().product::<usize>()
        } else {
            0
        }
    }).sum::<usize>()
}

fn main() {
    let chars = parse_input();
    let sn_vec = get_symbol_numbers(&chars);
    println!("part 1: {}", part1(&sn_vec));
    println!("part 2: {}", part2(&sn_vec));
}
