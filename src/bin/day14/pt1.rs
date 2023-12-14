use std::collections::{BTreeSet, HashMap};
use std::rc::Rc;

const INPUT: &str = include_str!("input.dat");
#[allow(dead_code)]
const SAMPLE: &str = include_str!("sample.dat");

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Map {
    rocks: PosMap,
    obstacles: PosMap,
    width: usize,
    height: usize,
}

impl Map {
    fn get(&self, x: usize, y: usize) -> char {
        if x >= self.width || y >= self.height {
            panic!()
        } else if self.rocks.contains(&(x, y)) {
            'O'
        } else if self.obstacles.contains(&(x, y)) {
            '#'
        } else {
            '.'
        }
    }

    fn move_rock(&mut self, orig: (usize, usize), new: (usize, usize)) {
        self.rocks.remove(&orig);
        self.rocks.insert(new);
    }
}

impl ToString for Map {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                s.push(self.get(x, y));
            }
            s.push('\n');
        }
        s
    }
}

type PosMap = BTreeSet<(usize, usize)>;

fn load_map(input: &str) -> Map {
    let char_map: Vec<Vec<char>> = input.trim().lines().map(|l| l.chars().collect()).collect();
    let mut rocks = BTreeSet::new();
    let mut obstacles = BTreeSet::new();
    let width = char_map[0].len();
    let height = char_map.len();
    for x in 0..width {
        for y in 0..height {
            if char_map[y][x] == '#' {
                obstacles.insert((x, y));
            } else if char_map[y][x] == 'O' {
                rocks.insert((x, y));
            }
        }
    }

    Map {
        rocks,
        obstacles,
        width,
        height,
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn tilt(
    map: &Rc<Map>,
    dir: Direction,
    memoized: &mut HashMap<(Rc<Map>, Direction), Rc<Map>>,
) -> (Rc<Map>, bool) {
    if memoized.contains_key(&(map.clone(), dir)) {
        return (memoized[&(map.clone(), dir)].clone(), true);
    };

    let mut nmap = (**map).clone();

    let x_range: Box<dyn Iterator<Item = usize>> = match dir {
        Direction::East => Box::new((0..map.width).rev()),
        _ => Box::new(0..map.width),
    };

    for x in x_range {
        let y_range: Box<dyn Iterator<Item = usize>> = match dir {
            Direction::South => Box::new((0..map.height).rev()),
            _ => Box::new(0..map.height),
        };
        for y in y_range {
            if nmap.get(x, y) == 'O' {
                let rock_loc = (x, y);
                let new = move_rock_dir(&nmap, rock_loc, dir);
                nmap.move_rock(rock_loc, new);
            }
        }
    }

    let nmap = Rc::new(nmap);
    memoized.insert((map.clone(), dir), nmap.clone());
    (nmap, false)
}

fn move_rock_dir_once(
    map: &Map,
    rock_loc: (usize, usize),
    dir: Direction,
) -> Option<(usize, usize)> {
    let (x, y) = rock_loc;

    match (y, dir) {
        (0, Direction::North) => return None,
        (y, Direction::South) if y == map.height - 1 => return None,
        _ => (),
    }

    match (x, dir) {
        (0, Direction::West) => return None,
        (x, Direction::East) if x == map.width - 1 => return None,
        _ => (),
    }

    let new_loc = match dir {
        Direction::North => (rock_loc.0, rock_loc.1 - 1),
        Direction::South => (rock_loc.0, rock_loc.1 + 1),
        Direction::East => (rock_loc.0 + 1, rock_loc.1),
        Direction::West => (rock_loc.0 - 1, rock_loc.1),
    };

    let (new_x, new_y) = new_loc;
    let new_char = map.get(new_x, new_y);
    (new_char == '.').then_some(new_loc)
}

fn move_rock_dir(map: &Map, mut rock_loc: (usize, usize), dir: Direction) -> (usize, usize) {
    while let Some(new_loc) = move_rock_dir_once(map, rock_loc, dir) {
        rock_loc = new_loc;
    }
    rock_loc
}

fn score_map(map: &Map) -> u64 {
    (0..map.height)
        .flat_map(move |y| (0..map.width).map(move |x| (x, y)))
        .filter(|&(x, y)| map.get(x, y) == 'O')
        .map(|(x, y)| (map.height - y) as u64)
        .sum()
}

pub fn pt1() {
    let map = Rc::new(load_map(INPUT));
    let (map, _) = tilt(&map, Direction::North, &mut HashMap::new());
    let result = score_map(&map);
    println!("{}", result);
}

pub fn pt2() {
    let mut map = Rc::new(load_map(INPUT));
    let mut memoized = HashMap::new();

    let mut fin;

    let mut loop_point = None;
    let mut loop_len = 0;
    let mut so_far = 0;

    let mut loops: Vec<Rc<Map>> = Vec::new();

    for ndx in 0..1000000000 {
        so_far = ndx;
        (map, _) = tilt(&map, Direction::North, &mut memoized);
        (map, _) = tilt(&map, Direction::West, &mut memoized);
        (map, _) = tilt(&map, Direction::South, &mut memoized);
        (map, fin) = tilt(&map, Direction::East, &mut memoized);

        if fin {
            match &loop_point {
                None => loop_point = Some(map.clone()),
                Some(loop_map) => {
                    loop_len += 1;
                    if &map == loop_map {
                        break;
                    }
                }
            }
            loops.push(map.clone());
        }
    }
    let remaining_loops = 1000000000 - (so_far + 1);
    let map = loops[remaining_loops % loop_len].clone();
    let result = score_map(&map);
    println!("{}", result);
}
