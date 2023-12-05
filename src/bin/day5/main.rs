use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;
use std::convert::Infallible;
use std::str::FromStr;

const INPUT: &str = include_str!("input.dat");
const SAMPLE: &str = include_str!("sample.dat");

lazy_static! {
    static ref EXAMPLE: Regex = Regex::new(r"example").unwrap();
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<SeedRange>,
    seed_to_soil: Vec<Map>,
    soil_to_fertilizer: Vec<Map>,
    fertilizer_to_water: Vec<Map>,
    water_to_light: Vec<Map>,
    light_to_temperature: Vec<Map>,
    temperature_to_humidity: Vec<Map>,
    humidity_to_location: Vec<Map>,
}

impl Almanac {
    fn consume_maps(lines: &mut std::str::Lines) -> Vec<Map> {
        let mut maps = Vec::new();

        loop {
            if let Some(line) = lines.next() {
                if line.is_empty() {
                    break;
                }

                maps.push(line.parse().unwrap());
            } else {
                break;
            }
        }

        maps
    }

    fn skip_two_lines(lines: &mut std::str::Lines) {
        lines.next();
        lines.next();
    }
}

#[derive(Debug, Copy, Clone)]
struct SeedRange {
    start: u64,
    len: u64,
}

impl SeedRange {
    fn iter(&self) -> impl Iterator<Item = u64> {
        SeedRangeIter {
            curr: self.start,
            range: self.clone(),
        }
    }
}

struct SeedRangeIter {
    curr: u64,
    range: SeedRange,
}

impl Iterator for SeedRangeIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.range.start + self.range.len {
            return None;
        }

        let ret = self.curr;
        self.curr += 1;

        Some(ret)
    }
}

impl FromStr for Almanac {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let mut seeds: Vec<SeedRange> = Vec::new();
        let mut seeds_numbers = lines
            .next()
            .unwrap()
            .split(':')
            .last()
            .unwrap()
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.parse::<u64>().unwrap());

        while let Some(start) = seeds_numbers.next() {
            let len = seeds_numbers.next().unwrap();
            seeds.push(SeedRange { start, len });
        }

        Almanac::skip_two_lines(&mut lines);
        let mut seed_to_soil = Almanac::consume_maps(&mut lines);
        seed_to_soil.sort_by_key(|m| m.dst_range_start);

        lines.next();
        let mut soil_to_fertilizer = Almanac::consume_maps(&mut lines);
        soil_to_fertilizer.sort_by_key(|m| m.dst_range_start);

        lines.next();
        let mut fertilizer_to_water = Almanac::consume_maps(&mut lines);
        fertilizer_to_water.sort_by_key(|m| m.dst_range_start);

        lines.next();
        let mut water_to_light = Almanac::consume_maps(&mut lines);
        water_to_light.sort_by_key(|m| m.dst_range_start);

        lines.next();
        let mut light_to_temperature = Almanac::consume_maps(&mut lines);
        light_to_temperature.sort_by_key(|m| m.dst_range_start);

        lines.next();
        let mut temperature_to_humidity = Almanac::consume_maps(&mut lines);
        temperature_to_humidity.sort_by_key(|m| m.dst_range_start);

        lines.next();
        let mut humidity_to_location = Almanac::consume_maps(&mut lines);
        humidity_to_location.sort_by_key(|m| m.dst_range_start);

        Ok(Almanac {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        })
    }
}

#[derive(Debug)]
struct Map {
    dst_range_start: u64,
    src_range_start: u64,
    range_len: u64,
}

impl Map {
    fn map_inp(&self, inp: u64) -> Option<u64> {
        if inp < self.src_range_start || inp >= self.src_range_start + self.range_len {
            return None;
        }

        Some(self.dst_range_start + (inp - self.src_range_start))
    }
}

fn find_mapping(inp: u64, maps: &Vec<Map>) -> u64 {
    maps.iter().find_map(|map| map.map_inp(inp)).unwrap_or(inp)
}

impl FromStr for Map {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace();
        let dst_range_start = parts.next().unwrap().parse().unwrap();
        let src_range_start = parts.next().unwrap().parse().unwrap();
        let range_len = parts.next().unwrap().parse().unwrap();

        Ok(Self {
            dst_range_start,
            src_range_start,
            range_len,
        })
    }
}

// mod pt1 {
//     use super::*;
//
//     pub fn pt1() {
//         let almanac: Almanac = INPUT.parse().unwrap();
//
//         let result = almanac
//             .seeds
//             .par_iter()
//             .map(|seed| {
//                 let seed = seed.clone();
//                 let soil = find_mapping(seed, &almanac.seed_to_soil);
//                 let fertilizer = find_mapping(soil, &almanac.soil_to_fertilizer);
//                 let water = find_mapping(fertilizer, &almanac.fertilizer_to_water);
//                 let light = find_mapping(water, &almanac.water_to_light);
//                 let temperature = find_mapping(light, &almanac.light_to_temperature);
//                 let humidity = find_mapping(temperature, &almanac.temperature_to_humidity);
//                 find_mapping(humidity, &almanac.humidity_to_location)
//             })
//             .min()
//             .unwrap();
//
//         println!("{}", result);
//     }
// }

mod pt2 {
    use super::*;

    pub fn pt2() {
        let almanac: Almanac = INPUT.parse().unwrap();

        fn find_min_in_range(almanac: &Almanac, sr: SeedRange) -> u64 {
            sr.iter()
                .map(|seed| {
                    let soil = find_mapping(seed, &almanac.seed_to_soil);
                    let fertilizer = find_mapping(soil, &almanac.soil_to_fertilizer);
                    let water = find_mapping(fertilizer, &almanac.fertilizer_to_water);
                    let light = find_mapping(water, &almanac.water_to_light);
                    let temperature = find_mapping(light, &almanac.light_to_temperature);
                    let humidity = find_mapping(temperature, &almanac.temperature_to_humidity);
                    find_mapping(humidity, &almanac.humidity_to_location)
                })
                .min()
                .unwrap()
        }

        let result: u64 = almanac
            .seeds
            .par_iter()
            .map(|sr| find_min_in_range(&almanac, sr.clone()))
            .min()
            .unwrap();

        println!("{}", result);
    }
}

fn main() {
    // println!("Part 1:");
    // pt1::pt1();
    println!("Part 2:");
    pt2::pt2();
}
