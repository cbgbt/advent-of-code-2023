use lazy_static::lazy_static;
use regex::Regex;
use std::convert::Infallible;
use std::str::FromStr;

type Result<T> = std::result::Result<T, Infallible>;
const INPUT: &str = include_str!("input.dat");

lazy_static! {
    static ref RED_RE: Regex = Regex::new(r"(\d)* red").unwrap();
    static ref GREEN_RE: Regex = Regex::new(r"(\d)* green").unwrap();
    static ref BLUE_RE: Regex = Regex::new(r"(\d)* blue").unwrap();
}

#[derive(Debug, Clone)]
struct Game {
    game_id: u32,
    grabs: Vec<Grab>,
}

impl FromStr for Game {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self> {
        let mut game_sep = s.split(':');
        let game_id_str = game_sep.next().unwrap();

        let game_id = game_id_str.split(' ').last().unwrap().parse().unwrap();

        let grabs_str = game_sep.next().unwrap();
        let grabs = grabs_str
            .split(';')
            .map(Grab::from_str)
            .collect::<Result<Vec<_>>>()
            .unwrap();

        Ok(Game { game_id, grabs })
    }
}

#[derive(Debug, Copy, Clone)]
struct Grab {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Grab {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self> {
        let red = RED_RE
            .find(s)
            .and_then(|m| m.as_str().split(' ').next().map(|s| s.parse().unwrap()))
            .unwrap_or_default();

        let green = GREEN_RE
            .find(s)
            .and_then(|m| m.as_str().split(' ').next().map(|s| s.parse().unwrap()))
            .unwrap_or_default();

        let blue = BLUE_RE
            .find(s)
            .and_then(|m| m.as_str().split(' ').next().map(|s| s.parse().unwrap()))
            .unwrap_or_default();

        Ok(Grab { red, green, blue })
    }
}

fn games() -> Vec<Game> {
    INPUT
        .lines()
        .map(str::parse)
        .collect::<Result<_>>()
        .unwrap()
}

mod pt1 {
    use super::*;

    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;

    fn pt1_evaluate_game(game: &Game) -> bool {
        for grab in game.grabs.iter() {
            if grab.red > MAX_RED || grab.green > MAX_GREEN || grab.blue > MAX_BLUE {
                return false;
            }
        }
        true
    }

    pub fn pt1() {
        let games = games();
        let result: u32 = games
            .iter()
            .filter_map(|game| pt1_evaluate_game(game).then_some(game.game_id))
            .sum();

        println!("{}", result);
    }
}

mod pt2 {
    use super::*;

    impl Game {
        fn power(&self) -> u32 {
            let mut min_r = 0;
            let mut min_g = 0;
            let mut min_b = 0;

            for grabs in self.grabs.iter() {
                if grabs.red > min_r {
                    min_r = grabs.red;
                }
                if grabs.green > min_g {
                    min_g = grabs.green;
                }
                if grabs.blue > min_b {
                    min_b = grabs.blue;
                }
            }

            min_r * min_g * min_b
        }
    }

    pub fn pt2() {
        let games = games();
        let result: u32 = games.iter().map(|game| game.power()).sum();

        println!("{}", result);
    }
}

fn main() {
    println!("Part 1:");
    pt1::pt1();
    println!("Part 2:");
    pt2::pt2();
}
