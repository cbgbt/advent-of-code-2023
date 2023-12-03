use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.dat");

struct Engine {
    lines: Vec<String>,
}

impl Engine {
    fn load(inp: &str) -> Self {
        Self {
            lines: inp.lines().map(|line| line.to_string()).collect(),
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<char> {
        if x < 0 || y < 0 {
            return None;
        }
        self.lines
            .get(y as usize)
            .and_then(|line| line.chars().nth(x as usize))
    }

    fn find_possible_part_nums(&self) -> Vec<PartNum> {
        self.lines
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                PART_NO_RE.find_iter(line).map(move |re_match| {
                    let num: u32 = re_match.as_str().parse().unwrap();
                    let x = re_match.start();
                    let len = re_match.len();
                    PartNum { x, y, len, num }
                })
            })
            .collect()
    }
}

struct PartNum {
    pub x: usize,
    pub y: usize,
    pub len: usize,
    pub num: u32,
}

lazy_static! {
    static ref PART_NO_RE: Regex = Regex::new(r"\d+").unwrap();
}

mod pt1 {
    use super::*;

    impl Engine {}

    fn check_surroundings(engine: &Engine, x: i32, y: i32) -> bool {
        for i in (x - 1)..=(x + 1) {
            for j in (y - 1)..=(y + 1) {
                if let Some(c) = engine.get(i, j) {
                    if !c.is_ascii_digit() && c != '.' {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn is_valid_part_num(part_num: &PartNum, engine: &Engine) -> bool {
        (part_num.x..(part_num.x + part_num.len))
            .any(|x| check_surroundings(engine, x as i32, part_num.y as i32))
    }

    pub fn pt1() {
        let engine = Engine::load(INPUT);
        let part_nums = engine.find_possible_part_nums();

        let result: u32 = part_nums
            .into_iter()
            .filter(|part_num| is_valid_part_num(part_num, &engine))
            .map(|part_num| part_num.num)
            .sum();

        println!("{}", result);
    }
}

mod pt2 {
    use super::*;

    impl PartNum {
        fn adjacent_gears(&self, engine: &Engine) -> Vec<(usize, usize)> {
            (self.x..(self.x + self.len))
                .flat_map(|x| self.adjacent_gears_to_loc(x, self.y, engine))
                .collect()
        }

        fn adjacent_gears_to_loc(
            &self,
            x: usize,
            y: usize,
            engine: &Engine,
        ) -> Vec<(usize, usize)> {
            let x = x as i32;
            let y = y as i32;

            ((x - 1)..=(x + 1))
                .flat_map(|i| ((y - 1)..=(y + 1)).map(move |j| (i, j)))
                .filter_map(|(i, j)| {
                    engine
                        .get(i, j)
                        .and_then(|c| (c == '*').then_some((i as usize, j as usize)))
                })
                .collect()
        }
    }

    fn find_gear_ratios(engine: &Engine, part_nums: &Vec<PartNum>) -> Vec<u32> {
        let mut possible_gears: HashMap<usize, HashMap<usize, HashSet<u32>>> = HashMap::new();

        for part_num in part_nums {
            for (x, y) in part_num.adjacent_gears(engine) {
                possible_gears
                    .entry(x)
                    .or_default()
                    .entry(y)
                    .or_default()
                    .insert(part_num.num);
            }
        }

        possible_gears
            .iter()
            .flat_map(|(_, inner)| {
                inner
                    .iter()
                    .filter(|(_, nums)| nums.len() == 2)
                    .map(|(_, nums)| nums.iter().product())
            })
            .collect()
    }

    pub fn pt2() {
        let engine = Engine::load(INPUT);
        let part_nums = engine.find_possible_part_nums();
        let ratios = find_gear_ratios(&engine, &part_nums);
        let result: u32 = ratios.iter().sum();

        println!("{}", result);
    }
}

fn main() {
    println!("Part 1:");
    pt1::pt1();
    println!("Part 2:");
    pt2::pt2();
}
