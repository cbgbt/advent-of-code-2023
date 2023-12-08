use super::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::str::FromStr;

const START: &str = "AAA";
const END: &str = "ZZZ";

#[derive(Debug, Clone)]
struct Map {
    instructions: Vec<Instruction>,
    nodes: HashMap<String, Node>,
}

impl Map {
    fn count_steps(&self) -> usize {
        let mut count = 0;
        let mut curr_name = START;

        while curr_name != END {
            let inst = &self.instructions[count % self.instructions.len()];
            let curr = self.nodes.get(curr_name).unwrap();

            curr_name = match inst {
                Instruction::Left => &curr.left,
                Instruction::Right => &curr.right,
            };

            count += 1;
        }

        count
    }
}

impl FromStr for Map {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let instructions = lines
            .next()
            .unwrap()
            .chars()
            .map(|c| String::from(c).parse().unwrap())
            .collect();

        lines.next().unwrap();
        let nodes = lines
            .map(|l| {
                let node: Node = l.parse().unwrap();
                (node.name.clone(), node)
            })
            .collect();

        Ok(Map {
            instructions,
            nodes,
        })
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Left,
    Right,
}

impl FromStr for Instruction {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "L" {
            Ok(Instruction::Left)
        } else if s == "R" {
            Ok(Instruction::Right)
        } else {
            panic!()
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl FromStr for Node {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut part_itr = s.split('=');
        let name = part_itr.next().unwrap().trim();
        let mut conn_side = part_itr.next().unwrap().trim().split(", ");
        let left = conn_side.next().unwrap()[1..].to_string();
        let right = conn_side.next().unwrap()[0..3].to_string();

        Ok(Node {
            name: name.to_string(),
            left,
            right,
        })
    }
}

const INPUT: &str = include_str!("input.dat");
#[allow(dead_code)]
const SAMPLE: &str = include_str!("sample.dat");

lazy_static! {
    static ref EXAMPLE: Regex = Regex::new(r"example").unwrap();
}

pub fn pt1() {
    let map = INPUT.parse::<Map>().unwrap();
    let result = map.count_steps();

    println!("{}", result);
}
