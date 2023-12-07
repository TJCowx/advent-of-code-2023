use std::{cmp::Ordering, collections::HashMap};

use crate::file_reader::file_reader;

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
enum CardValue {
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING,
    ACE,
}

impl CardValue {
    fn to_rank(&self) -> i32 {
        match self {
            CardValue::TWO => 2,
            CardValue::THREE => 3,
            CardValue::FOUR => 4,
            CardValue::FIVE => 5,
            CardValue::SIX => 6,
            CardValue::SEVEN => 7,
            CardValue::EIGHT => 8,
            CardValue::NINE => 9,
            CardValue::TEN => 10,
            CardValue::JACK => 11,
            CardValue::QUEEN => 12,
            CardValue::KING => 13,
            CardValue::ACE => 14,
        }
    }
}

impl From<char> for CardValue {
    fn from(c: char) -> Self {
        match c {
            '2' => CardValue::TWO,
            '3' => CardValue::THREE,
            '4' => CardValue::FOUR,
            '5' => CardValue::FIVE,
            '6' => CardValue::SIX,
            '7' => CardValue::SEVEN,
            '8' => CardValue::EIGHT,
            '9' => CardValue::NINE,
            'T' => CardValue::TEN,
            'J' => CardValue::JACK,
            'Q' => CardValue::QUEEN,
            'K' => CardValue::KING,
            'A' => CardValue::ACE,
            _ => panic!("Invalid card value: {}", c),
        }
    }
}

impl PartialOrd for CardValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_rank().partial_cmp(&other.to_rank())
    }
}

impl Ord for CardValue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_rank().cmp(&other.to_rank())
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
enum Hand {
    FiveOfAKind(String),
    FourOfAKind(String),
    FullHouse(String),
    ThreeOfAKind(String),
    TwoPair(String),
    OnePair(String),
    HighCard(String),
}

impl Hand {
    fn get_rank(&self) -> i32 {
        match self {
            Hand::FiveOfAKind(_) => 1,
            Hand::FourOfAKind(_) => 2,
            Hand::FullHouse(_) => 3,
            Hand::ThreeOfAKind(_) => 4,
            Hand::TwoPair(_) => 5,
            Hand::OnePair(_) => 6,
            Hand::HighCard(_) => 7,
        }
    }

    fn get_hand_string(&self) -> String {
        match self {
            Hand::FiveOfAKind(s) => s.to_string(),
            Hand::FourOfAKind(s) => s.to_string(),
            Hand::FullHouse(s) => s.to_string(),
            Hand::ThreeOfAKind(s) => s.to_string(),
            Hand::TwoPair(s) => s.to_string(),
            Hand::OnePair(s) => s.to_string(),
            Hand::HighCard(s) => s.to_string(),
        }
    }

    fn get_cards(&self) -> Vec<CardValue> {
        let hand_string = self.get_hand_string();
        hand_string
            .chars()
            .into_iter()
            .map(|c| CardValue::from(c))
            .collect()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let hand_cmp = self.get_rank().cmp(&other.get_rank());
        if hand_cmp == Ordering::Equal {
            let self_cards = self.get_cards();
            let other_cards = other.get_cards();

            // Loop through the cards and compare them, if one is stronger than the other return that comparison
            for (self_card, other_card) in self_cards.iter().zip(other_cards.iter()) {
                let card_cmp = other_card.cmp(self_card);
                if card_cmp != Ordering::Equal {
                    println!(
                        "Card Comparison: {:#?} vs {:#?} = {:#?}",
                        self_card, other_card, card_cmp
                    );
                    return Some(card_cmp);
                }
            }

            // If we get here then the hands are equal
            return Some(Ordering::Equal);
        }

        self.get_rank().partial_cmp(&other.get_rank())
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_rank().cmp(&other.get_rank())
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Play {
    hand: Hand,
    bet: i32,
}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.hand.partial_cmp(&self.hand)
    }
}

impl Ord for Play {
    fn cmp(&self, other: &Self) -> Ordering {
        other.hand.cmp(&self.hand)
    }
}

fn get_hand_value(hand: &str) -> Hand {
    let cards: Vec<CardValue> = hand
        .chars()
        .into_iter()
        .map(|c| CardValue::from(c))
        .collect();

    let mut card_counts: HashMap<CardValue, i32> = HashMap::new();
    for card in &cards {
        let count = card_counts.entry(*card).or_insert(0);
        *count += 1;
    }

    // Check for 5 of a kind
    if card_counts.iter().find(|&(_, &count)| count == 5).is_some() {
        return Hand::FiveOfAKind(hand.to_string());
    }

    // Check for 4 of a kind
    if card_counts.iter().find(|(_, &count)| count == 4).is_some() {
        return Hand::FourOfAKind(hand.to_string());
    }

    let has_three_of_a_kind = card_counts.iter().find(|(_, &count)| count == 3).is_some();
    let two_pairs = card_counts
        .iter()
        .filter(|(_, &count)| count == 2)
        .map(|(&card, _)| card)
        .collect::<Vec<CardValue>>();

    // Check for full house
    if has_three_of_a_kind && two_pairs.len() == 1 {
        return Hand::FullHouse(hand.to_string());
    }

    // Check for 3 of a kind
    if has_three_of_a_kind {
        return Hand::ThreeOfAKind(hand.to_string());
    }

    // Check if there is 2 two of a kinds
    if two_pairs.len() == 2 {
        return Hand::TwoPair(hand.to_string());
    }

    // Check for 1 pair
    if two_pairs.len() == 1 {
        return Hand::OnePair(hand.to_string());
    }

    // High card

    Hand::HighCard(hand.to_string())
}

pub fn solve() {
    let lines = file_reader::read_as_vec("inputs/day07.txt");

    let mut hands: Vec<Play> = vec![];
    for line in lines {
        let mut vals_iter = line.split(" ").collect::<Vec<&str>>().into_iter();

        let hand = vals_iter.next().unwrap();
        let bet = vals_iter.next().unwrap().parse::<i32>().unwrap();

        let hand_value = get_hand_value(hand);

        hands.push(Play {
            hand: hand_value,
            bet,
        });
    }

    hands.sort();

    let mut res = 0;
    for (rank, hand) in hands.iter().enumerate() {
        println!(
            "Hand: {:#?}, Rank: {}, Value: {}",
            hand,
            rank + 1,
            hand.bet * (rank + 1) as i32
        );

        res += hand.bet * (rank + 1) as i32;
    }

    println!("Day 7 Part 1: {}", res);
}
