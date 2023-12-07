use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::str::FromStr;

const INPUT: &str = include_str!("input.dat");
#[allow(dead_code)]
const SAMPLE: &str = include_str!("sample.dat");

lazy_static! {
    static ref EXAMPLE: Regex = Regex::new(r"example").unwrap();
}

#[derive(Debug, Clone)]
struct Set {
    hands: Vec<Hand>,
}

impl FromStr for Set {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hands = s.lines().map(|line| line.parse().unwrap()).collect();

        Ok(Self { hands })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
}

enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut card_counts: HashMap<Card, u64> = HashMap::new();

        let mut jokers = Vec::new();
        self.cards.iter().for_each(|card| {
            let Card(card_char) = *card;
            if card_char == 'J' {
                jokers.push(*card);
            } else {
                *card_counts.entry(*card).or_default() += 1;
            }
        });

        if jokers.len() == 5 {
            return HandType::FiveOfAKind;
        } else {
            jokers.iter().for_each(|_| {
                // Add to the highest val card, unless its 5, in which case the next
                let (high_card, count) =
                    card_counts.iter().max_by_key(|(_, count)| *count).unwrap();
                card_counts.insert(*high_card, *count + 1);
            });
        }

        let mut counts = card_counts
            .into_iter()
            .map(|(_, count)| count)
            .collect::<Vec<_>>();
        counts.sort();
        counts.reverse();
        let mut counts = counts.into_iter();

        match (counts.next().unwrap(), counts.next()) {
            (5, _) => HandType::FiveOfAKind,
            (4, Some(1)) => HandType::FourOfAKind,
            (3, Some(2)) => HandType::FullHouse,
            (3, _) => HandType::ThreeOfAKind,
            (2, Some(2)) => HandType::TwoPair,
            (2, _) => HandType::OnePair,
            (_, _) => HandType::HighCard,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.hand_type() as u64, self.cards).cmp(&(other.hand_type() as u64, other.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.hand_type() as u64, self.cards).partial_cmp(&(other.hand_type() as u64, other.cards))
    }
}

impl FromStr for Hand {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut iter = line.split_whitespace();

        let card_part = iter.next().unwrap();
        let cards: Vec<_> = card_part.chars().map(|s| Card(s)).collect();
        let cards: [Card; 5] = cards.try_into().unwrap();

        let bid = iter.next().unwrap().parse().unwrap();

        Ok(Hand { cards, bid })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Card(char);

impl Card {
    fn score(&self) -> u64 {
        match self.0 {
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            '9' => 8,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Invalid card"),
        }
    }

    fn pt2_score(&self) -> u64 {
        match self.0 {
            'J' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Invalid card"),
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.pt2_score().cmp(&other.pt2_score())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.pt2_score().partial_cmp(&other.pt2_score())
    }
}

mod pt1 {
    use super::*;

    pub fn pt1() {
        let set = Set::from_str(INPUT).unwrap();
        let mut hands = set.hands;
        hands.sort();

        let result = hands
            .iter()
            .enumerate()
            .map(|(i, hand)| hand.bid * (i as u64 + 1))
            .sum::<u64>();

        println!("{}", result);
    }
}

mod pt2 {
    use super::*;

    pub fn pt2() {
        let set = Set::from_str(INPUT).unwrap();
        let mut hands = set.hands;
        hands.sort();

        let result = hands
            .iter()
            .enumerate()
            .map(|(i, hand)| hand.bid * (i as u64 + 1))
            .sum::<u64>();

        println!("{}", result);
    }
}

fn main() {
    // Part 1 requires some minor changes to the code that I'm too lazy to fix.
    println!("Part 2:");
    pt2::pt2();
}
