use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::convert::Infallible;
use std::str::FromStr;

const INPUT: &str = include_str!("input.dat");
#[allow(dead_code)]
const SAMPLE: &str = include_str!("sample.dat");

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct HeatMap {
    tiles: Vec<Vec<u8>>,
}

impl HeatMap {
    fn get(&self, x: usize, y: usize) -> u8 {
        self.tiles[y][x]
    }

    fn height(&self) -> usize {
        self.tiles.len()
    }

    fn width(&self) -> usize {
        self.tiles[0].len()
    }
}

impl FromStr for HeatMap {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
            .collect();

        Ok(HeatMap { tiles })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Ship {
    direction: Direction,
    num_straight: u8,
    pos: (usize, usize),
    heat_loss: u64,
}

impl PartialOrd for Ship {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Reverse(self.heat_loss).partial_cmp(&Reverse(other.heat_loss))
    }
}

impl Ord for Ship {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Reverse(self.heat_loss).cmp(&Reverse(other.heat_loss))
    }
}

fn navigate_ship(heatmap: &HeatMap, ship: Ship, dir: Direction) -> Option<(usize, usize)> {
    let (x, y) = (ship.pos.0 as i64, ship.pos.1 as i64);
    let (x, y) = match dir {
        Up => (x, y - 1),
        Down => (x, y + 1),
        Left => (x - 1, y),
        Right => (x + 1, y),
    };

    if x < 0 || y < 0 || x >= heatmap.width() as i64 || y >= heatmap.width() as i64 {
        None
    } else {
        Some((x as usize, y as usize))
    }
}

fn find_minimal_route(
    heatmap: &HeatMap,
    ships: Vec<Ship>,
    min_straight: u8,
    max_straight: u8,
) -> u64 {
    let mut queue: BinaryHeap<_> = ships.into_iter().collect();
    let mut visited: HashMap<((usize, usize), Direction), u64> = HashMap::new();

    while !queue.is_empty() {
        let mut ship = queue.pop().unwrap();

        if let Some(&heat_loss) = visited.get(&(ship.pos, ship.direction)) {
            if heat_loss <= ship.heat_loss {
                continue;
            }
        }

        visited.insert((ship.pos, ship.direction), ship.heat_loss);

        if ship.pos == (heatmap.width() - 1, heatmap.height() - 1) {
            return ship.heat_loss;
        }

        let mut directions = HashSet::new();
        match ship.direction {
            Up | Down => {
                directions.insert(Left);
                directions.insert(Right);
            }
            Left | Right => {
                directions.insert(Up);
                directions.insert(Down);
            }
        }

        while ship.num_straight < max_straight {
            let new_pos = navigate_ship(heatmap, ship, ship.direction);
            if new_pos.is_none() {
                break;
            }
            let new_pos = new_pos.unwrap();
            ship.pos = new_pos;
            ship.heat_loss += heatmap.get(new_pos.0, new_pos.1) as u64;
            ship.num_straight += 1;

            if ship.num_straight >= min_straight {
                for direction in directions.iter() {
                    let mut new_ship = ship.clone();
                    new_ship.direction = *direction;
                    new_ship.num_straight = 0;

                    queue.push(new_ship);
                }
            }
        }
    }

    unreachable!()
}

pub fn pt1() {
    let heatmap: HeatMap = INPUT.parse().unwrap();
    let result = find_minimal_route(
        &heatmap,
        vec![Ship {
            direction: Down,
            num_straight: 0,
            pos: (0, 0),
            heat_loss: 0,
        }],
        0,
        3,
    );

    println!("{}", result);
}

pub fn pt2() {
    let heatmap: HeatMap = INPUT.parse().unwrap();
    let result = find_minimal_route(
        &heatmap,
        vec![
            Ship {
                direction: Right,
                num_straight: 0,
                pos: (0, 0),
                heat_loss: 0,
            },
            Ship {
                direction: Down,
                num_straight: 0,
                pos: (0, 0),
                heat_loss: 0,
            },
        ],
        4,
        10,
    );

    println!("{}", result);
}
