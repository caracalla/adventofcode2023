fn card_rank(card: char) -> usize {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1, // 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => unreachable!(),
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Hand {
    HighCard = 0,
    OnePair = 100,
    TwoPair = 200,
    ThreeOfAKind = 300,
    FullHouse = 400,
    FourOfAKind = 500,
    FiveOfAKind = 600,
}

#[derive(Clone, Debug)]
struct Camel {
    cards: String,
    bid: usize,
    hand: Hand,
}

fn sort_cards(cards: &str) -> Vec<usize> {
    let mut hand_ranks = cards.chars().map(|card| card_rank(card)).collect::<Vec<_>>();
    hand_ranks.sort_unstable_by(|a, b| b.cmp(a)); // sort descending
    hand_ranks
}

fn determine_hand(cards: &str) -> Hand {
    #[derive(Debug)]
    struct Sequence {
        card: usize,
        len: usize,
    }

    let sorted_cards = sort_cards(cards);

    let mut first_sequence = Sequence {
        card: sorted_cards[0],
        len: 1,
    };
    let mut second_sequence = Sequence {
        card: sorted_cards[0], // dummy value, ugh
        len: 0,
    };

    for card in sorted_cards.iter().skip(1).copied() {
        if card == card_rank('J') {
            if first_sequence.len == second_sequence.len {
                if first_sequence.card > second_sequence.card {
                    first_sequence.len += 1;
                } else {
                    second_sequence.len += 1;
                }
            } else {
                if first_sequence.len > second_sequence.len {
                    first_sequence.len += 1;
                } else {
                    second_sequence.len += 1;
                }
            }
        } else {
            if second_sequence.len > 0 {
                if card != second_sequence.card {
                    if second_sequence.len == 1 {
                        second_sequence = Sequence { card, len: 1 };
                    }
                } else {
                    second_sequence.len += 1;
                }
            } else {
                if card != first_sequence.card {
                    if first_sequence.len == 1 {
                        first_sequence.card = card;
                    } else {
                        second_sequence = Sequence { card, len: 1 };
                    }
                } else {
                    first_sequence.len += 1;
                }
            }
        }
    }

    match (first_sequence.len, second_sequence.len) {
        (5, 0) => Hand::FiveOfAKind,
        // second_sequence.len can be 0 if the sequence comes after a single card
        (4, 1) | (4, 0) => Hand::FourOfAKind,
        (3, 2) | (2, 3) => Hand::FullHouse,
        (3, 1) | (3, 0) => Hand::ThreeOfAKind,
        (2, 2) => Hand::TwoPair,
        (2, 1) | (2, 0) => Hand::OnePair,
        (1, 0) => Hand::HighCard,
        _ => unreachable!(),
    }
}

fn parse_input() -> Vec<Camel> {
    let file_contents = std::fs::read_to_string("./input.txt").unwrap();

    file_contents
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| {
            let mut parts = line.split(' ');

            let cards = parts.nth(0).unwrap().to_string();
            let bid = parts.nth(0).unwrap().parse::<usize>().unwrap();
            let hand = determine_hand(&cards);

            Camel { cards, bid, hand }
        })
        .collect()
}

fn part1(camels: &Vec<Camel>) -> usize {
    let mut sorted_camels = camels.clone();
    sorted_camels.sort_unstable_by(|a, b| {
        if a.hand != b.hand {
            (a.hand as usize).cmp(&(b.hand as usize))
        } else {
            for (card1, card2) in a.cards.chars().zip(b.cards.chars()) {
                if card1 != card2 {
                    return card_rank(card1).cmp(&card_rank(card2));
                }
            }
            std::cmp::Ordering::Equal
        }
    });
    sorted_camels.into_iter().enumerate().map(|(index, camel)| {
        // println!("{camel:?}");
        camel.bid * (index + 1)
    }).sum()
}

fn main() {
    let camels = parse_input();
    println!("part 1: {}", part1(&camels));
}
