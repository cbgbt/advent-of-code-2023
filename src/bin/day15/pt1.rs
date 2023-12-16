use super::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::Infallible;
use std::str::FromStr;

const INPUT: &str = include_str!("input.dat");
#[allow(dead_code)]
const SAMPLE: &str = include_str!("sample.dat");

lazy_static! {
    static ref EXAMPLE: Regex = Regex::new(r"example").unwrap();
}

fn hasher(input: &str) -> u64 {
    let mut val: u64 = 0;
    input
        .to_ascii_lowercase()
        .chars()
        .map(|c| c as u8)
        .for_each(|ascii| {
            val += ascii as u64;
            val *= 17;
            val = val % 256;
        });
    val
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Lens {
    label: &'static str,
    focal_len: u8,
}

pub fn pt1() {
    let input = INPUT.trim();
    let result: u64 = input.split(',').map(hasher).sum();
    println!("{}", result);
}

fn print_lenses(state: &Vec<Vec<Lens>>) {
    state.iter().enumerate().for_each(|(ndx, lens_box)| {
        if !lens_box.is_empty() {
            print!("Box {}: ", ndx);
            for lens in lens_box {
                print!("[{} {}]  ", lens.label, lens.focal_len);
            }
            println!();
        }
    });
    println!();
}

pub fn pt2() {
    let mut state: Vec<Vec<Lens>> = Vec::new();
    (0..256).for_each(|_| state.push(Vec::new()));

    let input = INPUT.trim();
    input.split(',').for_each(|command| {
        if command.contains('=') {
            let mut command_splitter = command.split('=');
            let (label, focal_len) = (
                command_splitter.next().unwrap(),
                command_splitter.next().unwrap().parse().unwrap(),
            );

            let box_num = hasher(label);

            let lens_box = &mut state[box_num as usize];
            let lens_pos = lens_box.iter().position(|l| l.label == label);
            if let Some(lens_pos) = lens_pos {
                lens_box[lens_pos].focal_len = focal_len;
            } else {
                lens_box.push(Lens { label, focal_len })
            }
        } else {
            let label = command.trim_end_matches('-');
            let box_num = hasher(label);

            let lens_box = &mut state[box_num as usize];
            let lens_pos = lens_box.iter().position(|l| l.label == label);
            if let Some(lens_pos) = lens_pos {
                lens_box.remove(lens_pos);
            }
        }
    });

    let result: usize = state
        .iter()
        .enumerate()
        .filter(|(_, lens_box)| !lens_box.is_empty())
        .flat_map(|(lens_box_ndx, lens_box)| {
            lens_box.iter().enumerate().map(move |(lens_ndx, lens)| {
                (lens_box_ndx + 1) * (lens_ndx + 1) * lens.focal_len as usize
            })
        })
        .sum();

    println!("{}", result);
}
