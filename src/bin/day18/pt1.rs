use super::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::Infallible;
use std::str::FromStr;

const INPUT: &str = include_str!("input.dat");
#[allow(dead_code)]
const SAMPLE: &str = include_str!("sample.dat");

struct Grid {
    vertices: Vec<(i64, i64)>,
    bound_size: u64,
}

impl Grid {
    fn from_instrs(instrs: &[Instr]) -> Self {
        let mut vertices = Vec::new();
        vertices.push((0, 0));
        let mut bound_size = 0;

        let mut cx = 0;
        let mut cy = 0;
        for instr in instrs {
            bound_size += instr.dist;
            let (dx, dy) = match instr.dir {
                Up => (0, -(instr.dist as i64)),
                Down => (0, instr.dist as i64),
                Left => (-(instr.dist as i64), 0),
                Right => (instr.dist as i64, 0),
            };

            cx += dx;
            cy += dy;

            vertices.push((cx, cy));
        }
        Grid {
            vertices,
            bound_size,
        }
    }

    fn area(&self) -> u64 {
        let sum: u64 = (0..self.vertices.len())
            .map(|i| {
                let p1 = (i + 1) % self.vertices.len();

                let lhx = self.vertices[i].0;
                let lhy = self.vertices[p1].1;

                let rhx = self.vertices[p1].0;
                let rhy = self.vertices[i].1;

                (lhx * lhy) - (rhx * rhy)
            })
            .sum::<i64>() as u64;
        let shoelace = sum / 2;

        shoelace + (self.bound_size / 2) + 1
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use Direction::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Instr {
    dir: Direction,
    dist: u64,
    color: String,
}

impl FromStr for Instr {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_ascii_whitespace();
        let dir = match split.next().unwrap() {
            "R" => Right,
            "D" => Down,
            "U" => Up,
            "L" => Left,
            _ => panic!(),
        };
        let dist = split.next().unwrap().parse().unwrap();
        let color = split
            .next()
            .unwrap()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .to_string();

        Ok(Instr { dir, dist, color })
    }
}

pub fn pt1() {
    let instrs: Vec<Instr> = INPUT.lines().map(|l| l.parse().unwrap()).collect();
    let grid = Grid::from_instrs(&instrs);
    let result = grid.area();

    println!("{}", result);
}
