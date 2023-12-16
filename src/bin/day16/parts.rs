use super::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::Infallible;
use std::str::FromStr;

const INPUT: &str = include_str!("input.dat");
#[allow(dead_code)]
const SAMPLE: &str = include_str!("sample.dat");

struct Tiles {
    tiles: Vec<Vec<char>>,
}

impl Tiles {
    fn get(&self, x: usize, y: usize) -> char {
        self.tiles[y][x]
    }

    fn height(&self) -> usize {
        self.tiles.len()
    }

    fn width(&self) -> usize {
        self.tiles[0].len()
    }
}

impl FromStr for Tiles {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s.lines().map(|l| l.chars().collect()).collect();

        Ok(Tiles { tiles })
    }
}

lazy_static! {
    static ref EXAMPLE: Regex = Regex::new(r"example").unwrap();
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Light {
    pos: (usize, usize),
    direction: Direction,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn modify_dir(tiles: &Tiles, x: i64, y: i64, dir: Direction) -> Option<(usize, usize)> {
    let (x, y) = match dir {
        Direction::Up => (x, y - 1),
        Direction::Down => (x, y + 1),
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
    };

    if x < 0 || y < 0 || x >= tiles.width() as i64 || y >= tiles.height() as i64 {
        None
    } else {
        Some((x as usize, y as usize))
    }
}

fn follow_light_beam_once(tiles: &Tiles, light: Light) -> HashSet<Light> {
    let mut results = HashSet::new();
    let curr_tile = tiles.get(light.pos.0, light.pos.1);

    let (x, y) = (light.pos.0 as i64, light.pos.1 as i64);

    match curr_tile {
        '.' => {
            modify_dir(tiles, x, y, light.direction).and_then(|pos| {
                results.insert(Light {
                    pos,
                    direction: light.direction,
                });
                Some(())
            });
        }
        '/' => {
            let new_dir = match light.direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            };
            modify_dir(tiles, x, y, new_dir).and_then(|pos| {
                results.insert(Light {
                    pos,
                    direction: new_dir,
                });
                Some(())
            });
        }
        '\\' => {
            let new_dir = match light.direction {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            };
            modify_dir(tiles, x, y, new_dir).and_then(|pos| {
                results.insert(Light {
                    pos,
                    direction: new_dir,
                });
                Some(())
            });
        }
        '|' => {
            match light.direction {
                Direction::Up | Direction::Down => {
                    modify_dir(tiles, x, y, light.direction).and_then(|pos| {
                        results.insert(Light {
                            pos,
                            direction: light.direction,
                        });
                        Some(())
                    });
                }
                Direction::Left | Direction::Right => {
                    modify_dir(tiles, x, y, Direction::Up).and_then(|pos| {
                        results.insert(Light {
                            pos,
                            direction: Direction::Up,
                        });
                        Some(())
                    });
                    modify_dir(tiles, x, y, Direction::Down).and_then(|pos| {
                        results.insert(Light {
                            pos,
                            direction: Direction::Down,
                        });
                        Some(())
                    });
                }
            };
        }
        '-' => match light.direction {
            Direction::Right | Direction::Left => {
                modify_dir(tiles, x, y, light.direction).and_then(|pos| {
                    results.insert(Light {
                        pos,
                        direction: light.direction,
                    });
                    Some(())
                });
            }
            Direction::Up | Direction::Down => {
                modify_dir(tiles, x, y, Direction::Left).and_then(|pos| {
                    results.insert(Light {
                        pos,
                        direction: Direction::Left,
                    });
                    Some(())
                });
                modify_dir(tiles, x, y, Direction::Right).and_then(|pos| {
                    results.insert(Light {
                        pos,
                        direction: Direction::Right,
                    });
                    Some(())
                });
            }
        },
        _ => panic!(),
    }

    results
}

fn get_energized(tiles: &Tiles, starting_light: Light) -> HashSet<(usize, usize)> {
    let mut occupied_tiles = HashSet::new();
    occupied_tiles.insert(starting_light.pos);

    let mut visited = HashSet::new();

    let mut to_visit = VecDeque::new();
    to_visit.push_back(starting_light);

    while !to_visit.is_empty() {
        let light = to_visit.pop_front().unwrap();

        if visited.contains(&light) {
            continue;
        }
        visited.insert(light);

        let new_lights = follow_light_beam_once(tiles, light);

        for new_light in &new_lights {
            to_visit.push_back(*new_light);
        }

        occupied_tiles.extend(new_lights.iter().map(|l| l.pos));
    }

    occupied_tiles
}

pub fn pt1() {
    let tiles: Tiles = INPUT.parse().unwrap();

    let starter_light = Light {
        pos: (0, 0),
        direction: Direction::Right,
    };
    let result: usize = get_energized(&tiles, starter_light).into_iter().count();

    println!("{}", result);
}

pub fn pt2() {
    let tiles: Tiles = INPUT.parse().unwrap();

    let mut energized: Vec<usize> = Vec::new();
    for x in 0..tiles.width() {
        energized.push(
            get_energized(
                &tiles,
                Light {
                    pos: (x, 0),
                    direction: Direction::Down,
                },
            )
            .into_iter()
            .count(),
        );

        energized.push(
            get_energized(
                &tiles,
                Light {
                    pos: (x, tiles.height() - 1),
                    direction: Direction::Up,
                },
            )
            .into_iter()
            .count(),
        );
    }

    for y in 0..tiles.height() {
        energized.push(
            get_energized(
                &tiles,
                Light {
                    pos: (0, y),
                    direction: Direction::Right,
                },
            )
            .into_iter()
            .count(),
        );

        energized.push(
            get_energized(
                &tiles,
                Light {
                    pos: (tiles.width() - 1, y),
                    direction: Direction::Left,
                },
            )
            .into_iter()
            .count(),
        );
    }

    let result: usize = energized.into_iter().max().unwrap();

    println!("{}", result);
}
