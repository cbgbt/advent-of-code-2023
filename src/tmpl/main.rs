use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.dat");
#[allow(dead_code)]
const SAMPLE: &str = include_str!("input.dat");

lazy_static! {
    static ref EXAMPLE: Regex = Regex::new(r"example").unwrap();
}

mod pt1 {
    use super::*;

    pub fn pt1() {
        let result: u32 = 0;

        println!("{}", result);
    }
}

mod pt2 {
    use super::*;

    pub fn pt2() {
        let result: u32 = 0;

        println!("{}", result);
    }
}

fn main() {
    println!("Part 1:");
    pt1::pt1();
    println!("Part 2:");
    pt2::pt2();
}
