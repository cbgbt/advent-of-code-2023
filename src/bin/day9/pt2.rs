use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::str::FromStr;

struct Puzzle {
    input: Vec<i64>,
}

impl Puzzle {
    fn find_histories(&self) -> Vec<Vec<i64>> {
        let mut histories = vec![self.input.clone()];

        let mut curr = &histories[0];
        loop {
            let mut next = Vec::with_capacity(curr.len() - 1);
            let mut i = 1;
            while i < curr.len() {
                next.push(curr[i] - curr[i - 1]);
                i += 1;
            }
            if next.iter().all(|v| *v == 0) {
                break;
            }

            histories.push(next);
            curr = &histories[histories.len() - 1];
        }

        histories
    }

    fn find_extrapolated_value(&self) -> i64 {
        let histories = self.find_histories();

        let mut diff = 0;
        histories
            .iter()
            .rev()
            .map(|h| {
                diff = h.first().unwrap() - diff;
                diff
            })
            .last()
            .unwrap()
    }
}

impl FromStr for Puzzle {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Puzzle {
            input: s
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        })
    }
}

const INPUT: &str = include_str!("input.dat");
#[allow(dead_code)]
const SAMPLE: &str = include_str!("sample.dat");

pub fn pt2() {
    let puzzles = INPUT.lines().map(|l| l.parse::<Puzzle>().unwrap());
    let result: i64 = puzzles.map(|p| p.find_extrapolated_value()).sum();
    println!("{}", result);
}
