use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    id: usize,
    winning: Vec<usize>,
    have: Vec<usize>,
}

enum Position {
    CardId,
    WinningNumbers,
    HaveNumbers,
}

fn parse_input() -> Vec<Card> {
    let file_contents = std::fs::read_to_string("./input.txt").unwrap();

    file_contents.split('\n').map(|line| {
        let mut card = Card { id: 0, winning: vec![], have: vec![] };
        let mut number = String::new();
        let mut position = Position::CardId;
        
        for ch in line.chars() {
            if ch.is_numeric() {
                number.push(ch);
            }

            match ch {
                ':' => {
                    card.id = number.parse::<usize>().unwrap();
                    number = String::new();
                    position = Position::WinningNumbers;
                }
                '|' => position = Position::HaveNumbers,
                ' ' => {
                   if number.len() > 0 {
                       match position {
                           Position::WinningNumbers => {
                               card.winning.push(number.parse::<usize>().unwrap());
                               number = String::new();
                           }
                           Position::HaveNumbers => {
                               card.have.push(number.parse::<usize>().unwrap());
                               number = String::new();
                           }
                           _ => (),
                       }
                   }
                }
                _ => (),
            };
        }

        // get the straggling have number, if there is one
        if number.len() > 0 {
            card.have.push(number.parse::<usize>().unwrap());
        }

        card
    }).collect()
}

fn part1(cards: &Vec<Card>) -> usize {
    cards.iter().map(|card| {
        let set = card.winning.iter().collect::<HashSet<_>>();
        let mut hits = 0;
        
        for number in card.have.iter() {
            if set.contains(number) {
                hits += 1;
            }
        }

        if hits > 0 {
            2_usize.pow(hits - 1)
        } else {
            0
        }
    }).sum::<usize>()
}

fn part2(cards: &Vec<Card>) -> usize {
    let mut card_counts = vec![1; cards.len()];

    for (card_index, card) in cards.iter().enumerate() {
        let set = card.winning.iter().collect::<HashSet<_>>();
        let hits = card.have.iter().filter(|num| set.contains(num)).count();
        let current_card_count = card_counts[card_index];
        
        for i in 0..hits {
            let counts_index_to_update = card_index + i + 1;
            if counts_index_to_update < card_counts.len() {
                card_counts[counts_index_to_update] += current_card_count;
            }
        }
    }

    card_counts.into_iter().sum::<usize>()
}

fn main() {
    let mut cards = parse_input();
    // the empty line at the end creates a "zero" entry, which usually isn't
    // a problem, but in this case does cause issues in part 2
    cards.pop();
    println!("part1: {}", part1(&cards));
    println!("part2: {}", part2(&cards));
}
