use super::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::str::FromStr;

const INPUT: &str = include_str!("input.dat");
#[allow(dead_code)]
const SAMPLE: &str = include_str!("sample.dat");

#[derive(Debug, Clone)]
struct Map {
    inputs: Vec<Vec<char>>,
}

impl Map {
    fn width(&self) -> usize {
        self.inputs[0].len()
    }

    fn height(&self) -> usize {
        self.inputs.len()
    }

    fn identify_expanded_rows(&self) -> HashSet<usize> {
        self.inputs
            .iter()
            .enumerate()
            .filter_map(|(i, row)| row.iter().all(|c| c == &'.').then_some(i))
            .collect()
    }

    fn identify_expanded_columns(&self) -> HashSet<usize> {
        (0..self.inputs[0].len())
            .filter_map(|i| self.inputs.iter().all(|row| row[i] == '.').then_some(i))
            .collect()
    }

    fn galaxy_distances(&self, expansion_factor: usize) -> Vec<usize> {
        let expanded_rows = self.identify_expanded_rows();
        let expanded_columns = self.identify_expanded_columns();

        let galaxy_locs: Vec<_> = self.galaxy_locs().collect();
        let mut result = Vec::new();

        for i in 0..(galaxy_locs.len() - 1) {
            for y in (i + 1)..galaxy_locs.len() {
                let (x1, y1) = galaxy_locs[i];
                let (x2, y2) = galaxy_locs[y];

                let num_er_between = expanded_rows
                    .iter()
                    .filter(|r| **r > min(y1, y2) && **r < max(y1, y2))
                    .count();

                let num_ec_between = expanded_columns
                    .iter()
                    .filter(|c| **c > min(x1, x2) && **c < max(x1, x2))
                    .count();

                let dx = (x2 as i32 - x1 as i32).abs();
                let dy = (y2 as i32 - y1 as i32).abs();
                let reg_dist = (dx + dy) as usize;

                let grid_between = num_er_between + num_ec_between;
                let expanded_dist = reg_dist - grid_between + expansion_factor * grid_between;
                result.push(expanded_dist);
            }
        }

        result
    }

    fn galaxy_locs(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        itertools::iproduct!(0..self.width(), 0..self.height())
            .filter(|(x, y)| self.inputs[*y][*x] == '#')
    }
}

impl FromStr for Map {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inputs = s.lines().map(|l| l.chars().collect()).collect();

        Ok(Map { inputs })
    }
}

lazy_static! {
    static ref EXAMPLE: Regex = Regex::new(r"example").unwrap();
}

pub fn pt1() {
    let input_map: Map = INPUT.parse().unwrap();
    let result: usize = input_map.galaxy_distances(2).iter().sum();

    println!("{}", result);
}

pub fn pt2() {
    let input_map: Map = INPUT.parse().unwrap();
    let result: usize = input_map.galaxy_distances(1000000).iter().sum();

    println!("{}", result);
}
