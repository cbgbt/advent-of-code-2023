use super::*;
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

impl Map {
    fn from_folded(s: &str) -> Self {
        let lines = s
            .lines()
            .map(|line| {
                let mut type_split = line.split_ascii_whitespace();
                let springs: String = type_split.next().unwrap().chars().collect();
                let springs = format!("{0}?{0}?{0}?{0}?{0}", springs).chars().collect();

                let groups = type_split.next().unwrap();
                let groups = format!("{0},{0},{0},{0},{0}", groups);

                let groups = groups.split(',').map(|s| s.parse().unwrap()).collect();
                SpringLine { springs, groups }
            })
            .collect();

        Map { lines }
    }
}

#[derive(Debug)]
struct SpringLine {
    springs: Vec<char>,
    groups: Vec<usize>,
}

impl ToString for SpringLine {
    fn to_string(&self) -> String {
        format!(
            "{} {}",
            self.springs.iter().collect::<String>(),
            self.groups
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl SpringLine {
    fn group_sizes(spring_line: &str) -> Vec<usize> {
        spring_line
            .split('.')
            .filter(|s| !s.is_empty())
            .map(|s| s.len())
            .collect()
    }

    fn evaluate_line(spring_line: &str, groups: &[usize]) -> bool {
        let group_sizes = Self::group_sizes(spring_line);
        group_sizes.len() == groups.len() && group_sizes.iter().zip(groups).all(|(a, b)| a == b)
    }

    fn should_evaluate(spring_line: &str, groups: &[usize]) -> bool {
        let curr_front_groups = spring_line.split('?').next().unwrap();
        let curr_front_groups: Vec<usize> = curr_front_groups
            .split('.')
            .filter(|s| !s.is_empty())
            .map(|s| s.len())
            .collect();

        curr_front_groups
            .iter()
            .enumerate()
            .zip(groups.iter())
            .all(|((ndx, a), b)| {
                if ndx == (curr_front_groups.len() - 1) {
                    *a <= *b
                } else {
                    *a == *b
                }
            })
    }

    fn num_arrangements(&self) -> usize {
        let mut sum = 0;
        let mut queue: VecDeque<String> = [self.springs.clone().into_iter().collect()]
            .into_iter()
            .collect();

        while !queue.is_empty() {
            let spring_line = queue.pop_front().unwrap();
            if !spring_line.contains('?') {
                if Self::evaluate_line(&spring_line, &self.groups) {
                    sum += 1;
                }
            } else {
                if let Some(ndx) = spring_line.find('?') {
                    let mut new_line = spring_line.clone();
                    new_line.replace_range(ndx..ndx + 1, "#");
                    if Self::should_evaluate(&new_line, &self.groups) {
                        queue.push_back(new_line);
                    }

                    let mut new_line = spring_line.clone();
                    new_line.replace_range(ndx..ndx + 1, ".");
                    if Self::should_evaluate(&new_line, &self.groups) {
                        queue.push_back(new_line);
                    }
                }
            }
        }

        sum
    }

    fn unfolded_n(&self, n: usize) -> SpringLine {
        let line_str = self.line_str();
        let springs = std::iter::repeat(line_str).take(n);
        let springs: String = itertools::intersperse(springs, "#".to_string()).collect();

        let springs = springs.chars().collect();

        let groups = std::iter::repeat(&self.groups)
            .take(n)
            .cloned()
            .flat_map(|v| v)
            .collect();

        SpringLine { springs, groups }
    }

    fn line_str(&self) -> String {
        self.springs.iter().collect()
    }

    fn group_str(&self) -> String {
        self.groups
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn num_arrangements_unfolded(&self) -> usize {
        let n = self.num_arrangements();
        let u_2 = self.unfolded_n(2).num_arrangements();
        let u_3 = self.unfolded_n(3).num_arrangements();
        let u_4 = self.unfolded_n(4).num_arrangements();
        let u_5 = self.unfolded_n(5).num_arrangements();

        let result = n.pow(5)
            + (4 * n.pow(3) * u_2)
            + (3 * n.pow(2) * u_3)
            + (3 * n * u_2.pow(2))
            + (2 * n * u_4)
            + (2 * u_2 * u_3)
            + u_5;

        result
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
        .map(|line| line.num_arrangements())
        .sum();
    println!("{}", result);
}

pub fn pt2() {
    let spring_map: Map = INPUT.parse().unwrap();

    let result: usize = spring_map
        .lines
        .par_iter()
        .map(|line| line.num_arrangements_unfolded())
        // .map(|res| {
        //     println!("{}", res);
        //     res
        // })
        .sum();
    println!("{}", result);
}
