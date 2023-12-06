use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;

const INPUT: &str = include_str!("input.dat");
#[allow(dead_code)]
const SAMPLE: &str = include_str!("sample.dat");

lazy_static! {
    static ref EXAMPLE: Regex = Regex::new(r"example").unwrap();
}

fn distance(t: u64, h: u64) -> u64 {
    h * (t - h)
}

#[derive(Debug, Clone, Copy)]
struct Race {
    t: u64,
    d: u64,
}

fn parse_races(inp: &str) -> Vec<Race> {
    let mut lines = inp.lines();

    let time_line = lines.next().unwrap();
    let times = time_line
        .split(':')
        .last()
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let distance_line = lines.next().unwrap();
    let distances = distance_line
        .split(':')
        .last()
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(t, d)| Race { t, d })
        .collect()
}

fn parse_single_race(inp: &str) -> Race {
    let mut lines = inp.lines();

    let time_line = lines.next().unwrap();
    let time: u64 = time_line
        .split(':')
        .last()
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .collect::<Vec<_>>()
        .join("")
        .parse()
        .unwrap();

    let distance_line = lines.next().unwrap();
    let distance = distance_line
        .split(':')
        .last()
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .collect::<Vec<_>>()
        .join("")
        .parse()
        .unwrap();

    Race {
        t: time,
        d: distance,
    }
}

mod pt1 {
    use super::*;

    pub fn pt1() {
        let races = parse_races(INPUT);

        let result: u64 = races
            .into_iter()
            .map(|r| {
                (0..r.t)
                    .map(move |h| distance(r.t, h))
                    .filter(move |d| *d > r.d)
                    .count() as u64
            })
            .product();

        println!("{}", result);
    }
}

mod pt2 {
    use rayon::iter::IntoParallelIterator;

    use super::*;

    pub fn pt2() {
        let race = parse_single_race(INPUT);
        let result = (0..race.t)
            .into_par_iter()
            .map(move |h| distance(race.t, h))
            .filter(move |d| *d > race.d)
            .count() as u64;

        println!("{}", result);
    }
}

fn main() {
    println!("Part 1:");
    pt1::pt1();
    println!("Part 2:");
    pt2::pt2();
}
