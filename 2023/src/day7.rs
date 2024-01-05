#![allow(dead_code)]

use std::collections::HashMap;

const JOKER: i32 = 1;

fn card_to_value(c: char) -> i32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        cv => panic!("Unexpected card value: {}", cv),
    }
}

fn card_to_value2(c: char) -> i32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        cv => panic!("Unexpected card value: {}", cv),
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    Tree,
    Full,
    Four,
    Five,
}

#[derive(Debug, Eq, Ord)]
struct Hand {
    cards: Vec<i32>,
    counts: HandType,
    bet: i64,
    str: String,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.counts == other.counts && self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.counts.partial_cmp(&other.counts) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.cards.partial_cmp(&other.cards)
    }
}

impl Hand {
    fn new(hand: String, bet: String) -> Self {
        let cards = hand.chars().map(card_to_value).collect::<Vec<_>>();
        // mh.sort();
        let counts: HashMap<i32, i32> = cards.iter().fold(HashMap::new(), |mut acc, el| {
            acc.insert(*el, *acc.get(el).unwrap_or(&0) + 1);
            acc
        });

        let mut count_values = counts.iter().map(|(_, value)| value).collect::<Vec<_>>();

        count_values.sort_by(|a, b| b.cmp(a));

        let str = count_values
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let hand_type = match str.as_str() {
            "1,1,1,1,1" => HandType::HighCard,
            "2,1,1,1" => HandType::OnePair,
            "2,2,1" => HandType::TwoPair,
            "3,1,1" => HandType::Tree,
            "3,2" => HandType::Full,
            "4,1" => HandType::Four,
            "5" => HandType::Five,
            unk => panic!("{}", unk),
        };

        Self {
            cards,
            counts: hand_type,
            bet: bet.parse().unwrap(),
            str: hand,
        }
    }

    fn new2(hand: String, bet: String) -> Self {
        let cards = hand.chars().map(card_to_value2).collect::<Vec<_>>();
        // mh.sort();
        let mut counts: HashMap<i32, i32> = cards.iter().fold(HashMap::new(), |mut acc, el| {
            acc.insert(*el, *acc.get(el).unwrap_or(&0) + 1);
            acc
        });

        // let mut entries = counts.iter().collect::<Vec<_>>();
        // entries.sort_by(|a, b| b.1.cmp(&(a.1)));

        // println!("sort: {:?}", entries);

        let mut jokers = 0;
        let mut max_count = 0;
        let mut max_card = 0;

        for (card, count) in &counts {
            if card == &JOKER {
                jokers += count;
            }
            if count > &max_count && *card != JOKER {
                max_card = *card;
                max_count = *count;
            }
        }

        counts = counts
            .iter()
            .fold(HashMap::new(), |mut acc, (card, count)| {
                println!("{}", hand);
                if jokers == 5 {
                    acc.insert(*card, *count);
                    return acc;
                }
                if *card == JOKER {
                    return acc;
                }

                if *card == max_card {
                    acc.insert(*card, *count + jokers);
                } else {
                    acc.insert(*card, *count);
                }
                acc
            });

        let mut count_values = counts.iter().map(|(_, value)| value).collect::<Vec<_>>();

        count_values.sort_by(|a, b| b.cmp(a));

        let str = count_values
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let hand_type = match str.as_str() {
            "1,1,1,1,1" => HandType::HighCard,
            "2,1,1,1" => HandType::OnePair,
            "2,2,1" => HandType::TwoPair,
            "3,1,1" => HandType::Tree,
            "3,2" => HandType::Full,
            "4,1" => HandType::Four,
            "5" => HandType::Five,
            unk => panic!("{}", unk),
        };

        Self {
            cards,
            counts: hand_type,
            bet: bet.parse().unwrap(),
            str: hand,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::day7::Hand;

    #[test]
    fn p1() {
        let contents = fs::read_to_string("./problems/d7.txt").expect("");

        let s = contents
            .split("\n")
            .map(|l| l.split_once(" "))
            .filter_map(|l| l)
            .collect::<Vec<_>>();

        // println!("{:?}", s);

        // let hands = Vec::new();

        let mut hands = s
            .iter()
            .map(|(hand, bet)| Hand::new(hand.to_string(), bet.to_string()))
            .collect::<Vec<_>>();

        hands.sort();

        // println!("- {:?}", hands);

        let mut profit: i64 = 0;
        let mut i: i64 = 1;
        for hand in hands {
            println!("hand = {:?}", hand);
            profit += hand.bet * i;
            i += 1;
        }

        println!("profit={}", profit);
    }

    #[test]
    fn p2() {
        let contents = fs::read_to_string("./problems/d7.txt").expect("");

        let s = contents
            .split("\n")
            .map(|l| l.split_once(" "))
            .filter_map(|l| l)
            .collect::<Vec<_>>();

        // println!("{:?}", s);

        // let hands = Vec::new();

        let mut hands = s
            .iter()
            .map(|(hand, bet)| Hand::new2(hand.to_string(), bet.to_string()))
            .collect::<Vec<_>>();

        hands.sort();

        // println!("- {:?}", hands);

        let mut profit: i64 = 0;
        let mut i: i64 = 1;
        for hand in hands {
            println!("hand = {:?}", hand);
            profit += hand.bet * i;
            i += 1;
        }

        println!("profit={}", profit);
    }
}
