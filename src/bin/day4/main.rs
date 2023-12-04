use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::convert::Infallible;
use std::str::FromStr;

const INPUT: &str = include_str!("input.dat");
const SAMPLE: &str = include_str!("sample.dat");

lazy_static! {
    static ref EXAMPLE: Regex = Regex::new(r"example").unwrap();
}

#[derive(Debug)]
struct Game {
    id: u32,
    winners: HashSet<u32>,
    actual: HashSet<u32>,
}

impl Game {
    fn num_winners(&self) -> u32 {
        self.winners.intersection(&self.actual).count() as u32
    }
}

impl FromStr for Game {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut id_split = s.split(':');

        let card_id_part = id_split.next().unwrap();
        let id = card_id_part
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let hand_part = id_split.next().unwrap();

        let mut winners_and_hand = hand_part.split('|');

        let winners_part = winners_and_hand.next().unwrap();
        let winners = winners_part
            .split_ascii_whitespace()
            .map(|n| str::parse(n).unwrap())
            .collect();

        let hand_part = winners_and_hand.next().unwrap();
        let actual = hand_part
            .split_ascii_whitespace()
            .map(|n| str::parse(n).unwrap())
            .collect();

        Ok(Self {
            id,
            winners,
            actual,
        })
    }
}

mod pt1 {
    use super::*;

    impl Game {
        fn evaluate(&self) -> u32 {
            let num_winners = self.num_winners();

            if num_winners == 0 {
                0
            } else {
                2u32.pow(num_winners - 1)
            }
        }
    }

    pub fn pt1() {
        let result: u32 = INPUT
            .lines()
            .map(|l| l.parse::<Game>().unwrap().evaluate())
            .sum();

        println!("{}", result);
    }
}

mod pt2 {
    use super::*;

    impl Game {
        fn get_next_games<'a>(&self, games: &'a Vec<Game>) -> Vec<&'a Game> {
            let fits_both = self.winners.intersection(&self.actual);
            let num_matches = fits_both.count() as u32;

            (0..num_matches)
                .map(|i| &games[(self.id + i) as usize])
                .collect()
        }
    }

    pub fn pt2() {
        let games = INPUT
            .lines()
            .map(|l| l.parse::<Game>().unwrap())
            .collect::<Vec<_>>();

        let mut needs_processing: VecDeque<&Game> = games.iter().collect();

        let mut count = 0u32;
        while !needs_processing.is_empty() {
            count += 1;
            let curr_game = needs_processing.pop_front().unwrap();
            let next_games = curr_game.get_next_games(&games);
            next_games
                .into_iter()
                .for_each(|g| needs_processing.push_back(g));
        }

        println!("{}", count);
    }
}

fn main() {
    println!("Part 1:");
    pt1::pt1();
    println!("Part 2:");
    pt2::pt2();
}
