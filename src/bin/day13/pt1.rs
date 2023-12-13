use super::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::str::FromStr;

const INPUT: &str = include_str!("input.dat");
#[allow(dead_code)]
const SAMPLE: &str = include_str!("sample2.dat");

type Pattern = Vec<Vec<char>>;

fn parse_patterns(input: &str) -> Vec<Pattern> {
    input
        .trim()
        .split("\n\n")
        .map(|pattern| pattern.lines().map(|line| line.chars().collect()).collect())
        .collect()
}

fn find_vertical_refl_site(pattern: &Pattern) -> Vec<Window> {
    fn check_window(window: (usize, usize), pattern: &Pattern) -> bool {
        let mut lhs = window.0 as i64;
        let mut rhs = window.1 as i64;
        while lhs >= 0 && rhs < pattern[0].len() as i64 {
            for y in 0..pattern.len() {
                if pattern[y][lhs as usize] != pattern[y][rhs as usize] {
                    return false;
                }
            }
            lhs -= 1;
            rhs += 1;
        }

        true
    }

    (0..(pattern[0].len() - 1))
        .filter_map(|x| {
            let window = (x, x + 1);
            let window = check_window(window, pattern).then_some(Window {
                window,
                kind: Kind::Vertical,
            });
            window
        })
        .collect()
}

fn find_horizontal_refl_site(pattern: &Pattern) -> Vec<Window> {
    fn check_window(window: (usize, usize), pattern: &Pattern) -> bool {
        let mut uhs = window.0 as i64;
        let mut dhs = window.1 as i64;
        while uhs >= 0 && dhs < pattern.len() as i64 {
            for x in 0..pattern[0].len() {
                if pattern[uhs as usize][x] != pattern[dhs as usize][x] {
                    return false;
                }
            }
            uhs -= 1;
            dhs += 1;
        }

        true
    }

    (0..(pattern.len() - 1))
        .filter_map(|y| {
            let window = (y, y + 1);
            let window = check_window(window, pattern).then_some(Window {
                window,
                kind: Kind::Horizontal,
            });
            window
        })
        .collect()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Kind {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Window {
    kind: Kind,
    window: (usize, usize),
}

fn score_window(window: Window) -> u64 {
    match window.kind {
        Kind::Horizontal => 100 * (window.window.0 + 1) as u64,
        Kind::Vertical => (window.window.0 + 1) as u64,
    }
}

fn score_pattern(pattern: &Pattern) -> Option<u64> {
    find_vertical_refl_site(&pattern)
        .first()
        .cloned()
        .or_else(|| find_horizontal_refl_site(&pattern).first().cloned())
        .map(score_window)
}

fn score_pattern_pt2(ndx: usize, pattern: &Pattern) -> u64 {
    let orig_refl = find_vertical_refl_site(&pattern)
        .first()
        .cloned()
        .or_else(|| find_horizontal_refl_site(&pattern).first().cloned())
        .unwrap();

    let (x, y) = (12, 6);
    for y in 0..pattern.len() {
        for x in 0..pattern[0].len() {
            let mut npattern = pattern.clone();
            npattern[y][x] = if pattern[y][x] == '#' { '.' } else { '#' };

            let mut refls = find_vertical_refl_site(&npattern);
            refls.append(&mut find_horizontal_refl_site(&npattern));

            if let Some(refl) = refls.into_iter().find(|r| *r != orig_refl) {
                return score_window(refl);
            }
        }
    }

    println!("{}", ndx);
    panic!()
}

lazy_static! {
    static ref EXAMPLE: Regex = Regex::new(r"example").unwrap();
}

pub fn pt1() {
    let patterns = parse_patterns(INPUT);

    let result: u64 = patterns.iter().map(score_pattern).map(Option::unwrap).sum();

    println!("{}", result);
}

pub fn pt2() {
    let patterns = parse_patterns(INPUT);

    let result: u64 = patterns
        .iter()
        .enumerate()
        .map(|(i, pattern)| score_pattern_pt2(i, pattern))
        .sum();

    println!("{}", result);
}
