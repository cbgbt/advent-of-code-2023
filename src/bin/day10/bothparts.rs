use super::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::Infallible;
use std::str::FromStr;

const INPUT: &str = include_str!("input.dat");
#[allow(dead_code)]
const SAMPLE: &str = include_str!("sample.dat");
#[allow(dead_code)]
const SAMPLE2: &str = include_str!("sample2.dat");

#[derive(Debug, Clone)]
struct Map {
    pipes: Vec<Vec<Pipe>>,
}

impl Map {
    fn height(&self) -> usize {
        self.pipes.len()
    }

    fn width(&self) -> usize {
        self.pipes[0].len()
    }

    fn check_bounds(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        if x >= self.width() || y >= self.height() {
            None
        } else {
            Some((x, y))
        }
    }

    fn adjacent_pipes(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let pipe = self.pipes[y][x];

        let mut result = vec![];

        if pipe.points_up() && y != 0 && self.pipes[y - 1][x].points_down() {
            result.push((x, y - 1));
        }

        if pipe.points_down() && y + 1 < self.height() && self.pipes[y + 1][x].points_up() {
            result.push((x, y + 1));
        }

        if pipe.points_left() && x != 0 && self.pipes[y][x - 1].points_right() {
            result.push((x - 1, y));
        }

        if pipe.points_right() && x + 1 < self.width() && self.pipes[y][x + 1].points_left() {
            result.push((x + 1, y));
        }

        result
    }

    fn find_start(&self) -> (usize, usize) {
        for x in 0..self.width() {
            for y in 0..self.height() {
                if self.pipes[y][x] == Pipe::Start {
                    return (x, y);
                }
            }
        }

        panic!("No start found");
    }

    fn find_furthest_dist_from_start(&self) -> u64 {
        let start = self.find_start();

        let mut visited = HashSet::new();
        let mut to_visit: VecDeque<((usize, usize), u64)> =
            [(start.clone(), 0)].into_iter().collect();

        let mut max = 0;
        while !to_visit.is_empty() {
            let ((x, y), dist) = to_visit.pop_front().unwrap();
            if dist > max {
                max = dist;
            }

            visited.insert((x, y));
            let adjacent = self.adjacent_pipes(x, y);
            for (x, y) in adjacent {
                if !visited.contains(&(x, y)) {
                    to_visit.push_back(((x, y), dist + 1));
                }
            }
        }
        max
    }

    fn find_loop_tiles(&self) -> HashSet<(usize, usize)> {
        let start = self.find_start();

        let mut visited = HashSet::new();
        let mut to_visit: VecDeque<((usize, usize), u64)> =
            [(start.clone(), 0)].into_iter().collect();

        while !to_visit.is_empty() {
            let ((x, y), dist) = to_visit.pop_front().unwrap();

            visited.insert((x, y));
            let adjacent = self.adjacent_pipes(x, y);
            for (x, y) in adjacent {
                if !visited.contains(&(x, y)) {
                    to_visit.push_back(((x, y), dist + 1));
                }
            }
        }

        visited
    }

    fn find_num_outside_loop(&self) -> u64 {
        let loop_tiles = self.find_loop_tiles();

        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        enum Status {
            Loop,
            Outside,
            Unknown,
        }

        fn overlay_pos(x: usize, y: usize) -> (usize, usize) {
            (x * 2 + 1, y * 2 + 1)
        }

        // Create an overlay that intersperses a tile between each actual tile
        let mut statuses: Vec<Vec<Status>> =
            vec![vec![Status::Unknown; self.width() * 2 + 1]; self.height() * 2 + 1];

        for (x, y) in loop_tiles {
            let curr_pipe = self.pipes[y][x];
            let (ox, oy) = overlay_pos(x, y);

            statuses[oy][ox] = Status::Loop;
            if curr_pipe.points_up() && y != 0 && self.pipes[y - 1][x].points_down() {
                statuses[oy - 1][ox] = Status::Loop;
            }

            if curr_pipe.points_down() && y + 1 < self.height() && self.pipes[y + 1][x].points_up()
            {
                statuses[oy + 1][ox] = Status::Loop;
            }

            if curr_pipe.points_left() && x != 0 && self.pipes[y][x - 1].points_right() {
                statuses[oy][ox - 1] = Status::Loop;
            }

            if curr_pipe.points_right()
                && x + 1 < self.width()
                && self.pipes[y][x + 1].points_left()
            {
                statuses[oy][ox + 1] = Status::Loop;
            }
        }

        let mut to_infect: VecDeque<(usize, usize)> = [(0, 0)].into_iter().collect();
        while !to_infect.is_empty() {
            let (cx, cy) = to_infect.pop_front().unwrap();

            if cx != 0 && statuses[cy][cx - 1] == Status::Unknown {
                statuses[cy][cx - 1] = Status::Outside;
                to_infect.push_back((cx - 1, cy));
            }
            if cx != statuses[0].len() - 1 && statuses[cy][cx + 1] == Status::Unknown {
                statuses[cy][cx + 1] = Status::Outside;
                to_infect.push_back((cx + 1, cy));
            }

            if cy != 0 && statuses[cy - 1][cx] == Status::Unknown {
                statuses[cy - 1][cx] = Status::Outside;
                to_infect.push_back((cx, cy - 1));
            }
            if cy != statuses.len() - 1 && statuses[cy + 1][cx] == Status::Unknown {
                statuses[cy + 1][cx] = Status::Outside;
                to_infect.push_back((cx, cy + 1));
            }
        }

        let mut count = 0;

        for y in 0..self.height() {
            for x in 0..self.width() {
                let (ox, oy) = overlay_pos(x, y);
                if statuses[oy][ox] == Status::Unknown {
                    count += 1;
                }
            }
        }
        count
    }
}

impl FromStr for Map {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pipes = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse().unwrap())
                    .collect()
            })
            .collect();

        Ok(Map { pipes })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Pipe {
    Vert,
    Hori,
    NE,
    NW,
    SE,
    SW,
    Ground,
    Start,
}

impl Pipe {
    fn points_up(&self) -> bool {
        match self {
            Pipe::Vert => true,
            Pipe::Hori => false,
            Pipe::NE => true,
            Pipe::NW => true,
            Pipe::SE => false,
            Pipe::SW => false,
            Pipe::Ground => false,
            Pipe::Start => true,
        }
    }

    fn points_down(&self) -> bool {
        match self {
            Pipe::Vert => true,
            Pipe::Hori => false,
            Pipe::NE => false,
            Pipe::NW => false,
            Pipe::SE => true,
            Pipe::SW => true,
            Pipe::Ground => false,
            Pipe::Start => true,
        }
    }

    fn points_left(&self) -> bool {
        match self {
            Pipe::Vert => false,
            Pipe::Hori => true,
            Pipe::NE => false,
            Pipe::NW => true,
            Pipe::SE => false,
            Pipe::SW => true,
            Pipe::Ground => false,
            Pipe::Start => true,
        }
    }

    fn points_right(&self) -> bool {
        match self {
            Pipe::Vert => false,
            Pipe::Hori => true,
            Pipe::NE => true,
            Pipe::NW => false,
            Pipe::SE => true,
            Pipe::SW => false,
            Pipe::Ground => false,
            Pipe::Start => true,
        }
    }
}

impl FromStr for Pipe {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "|" => Ok(Pipe::Vert),
            "-" => Ok(Pipe::Hori),
            "L" => Ok(Pipe::NE),
            "J" => Ok(Pipe::NW),
            "7" => Ok(Pipe::SW),
            "F" => Ok(Pipe::SE),
            "." => Ok(Pipe::Ground),
            "S" => Ok(Pipe::Start),
            _ => panic!("unknown pipe {}", s),
        }
    }
}

lazy_static! {
    static ref EXAMPLE: Regex = Regex::new(r"example").unwrap();
}

pub fn pt1() {
    let map = Map::from_str(INPUT).unwrap();
    let result = map.find_furthest_dist_from_start();

    println!("{}", result);
}

pub fn pt2() {
    let map = Map::from_str(INPUT).unwrap();
    let result = map.find_num_outside_loop();

    println!("{}", result);
}
