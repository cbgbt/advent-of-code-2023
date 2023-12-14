use super::*;
use itertools::Itertools;
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug)]
struct Map {
    lines: Vec<SpringLine>,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct SpringLine {
    springs: String,
    groups: Vec<usize>,
}

impl ToString for SpringLine {
    fn to_string(&self) -> String {
        format!(
            "{} {}",
            self.springs,
            self.groups
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl SpringLine {
    fn num_arrangements(&self, memoize: &mut HashMap<SpringLine, usize>) -> usize {
        if memoize.contains_key(&self) {
            return memoize[&self];
        }

        // If there are no springs but groups, we don't match
        if !self.groups.is_empty() && self.springs.len() < *self.groups.iter().next().unwrap() {
            return 0;
        }

        // If there are no more groups left, all ? must be .
        if self.groups.is_empty() {
            if self.springs.contains('#') {
                return 0;
            } else {
                return 1;
            }
        }

        // Trim leading '.' characters
        let trimmed = self.springs.trim_start_matches('.');
        if trimmed != self.springs {
            return SpringLine {
                springs: trimmed.to_string(),
                groups: self.groups.clone(),
            }
            .num_arrangements(memoize);
        }
        let mut chars = self.springs.chars();
        let first_char = chars.next().unwrap();

        let mut groups = self.groups.iter();
        let next_group = groups.next().unwrap();

        let result = if first_char == '#' {
            // This must match the next group
            let mut rest = (&mut chars).take(next_group - 1);
            if rest.any(|c| c == '.') || chars.next() == Some('#') {
                0
            } else {
                let result = SpringLine {
                    springs: chars.collect(),
                    groups: groups.cloned().collect(),
                }
                .num_arrangements(memoize);
                result
            }
        } else if first_char == '?' {
            let dot_answer = SpringLine {
                springs: chars.clone().collect(),
                groups: self.groups.clone(),
            }
            .num_arrangements(memoize);
            let spring_answer = SpringLine {
                springs: ['#'].into_iter().chain(chars).collect(),
                groups: self.groups.clone(),
            }
            .num_arrangements(memoize);

            dot_answer + spring_answer
        } else {
            unreachable!()
        };

        memoize.insert(self.clone(), result);
        result
    }

    fn unfolded_n(&self, n: usize) -> SpringLine {
        let line_str = self.line_str();
        let springs = std::iter::repeat(line_str).take(n);
        let springs: String = itertools::intersperse(springs, "?").collect();

        let springs = springs.chars().collect();

        let groups = std::iter::repeat(&self.groups)
            .take(n)
            .cloned()
            .flat_map(|v| v)
            .collect();

        SpringLine { springs, groups }
    }

    fn line_str(&self) -> &str {
        self.springs.as_ref()
    }
}

impl FromStr for Map {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .lines()
            .map(|line| {
                let mut type_split = line.split_ascii_whitespace();
                let springs = type_split.next().unwrap().chars().collect();

                let groups = type_split
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect();
                SpringLine { springs, groups }
            })
            .collect();

        Ok(Map { lines })
    }
}

const INPUT: &str = include_str!("input.dat");
#[allow(dead_code)]
const SAMPLE: &str = include_str!("sample.dat");

lazy_static! {
    static ref GROUP: Regex = Regex::new(r"#+").unwrap();
}

pub fn pt1() {
    let spring_map: Map = INPUT.parse().unwrap();

    let result: usize = spring_map
        .lines
        .par_iter()
        .map(|line| line.num_arrangements(&mut HashMap::new()))
        .sum();
    println!("{}", result);
}

pub fn pt2() {
    let spring_map: Map = INPUT.parse().unwrap();

    let result: usize = spring_map
        .lines
        .par_iter()
        .map(|line| line.unfolded_n(5).num_arrangements(&mut HashMap::new()))
        .sum();
    println!("{}", result);
}
