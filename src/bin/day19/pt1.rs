use super::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl FromStr for Part {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start_matches('{').trim_end_matches('}');
        let mut attr_split = s.split(',');

        let mut parse_next = || {
            attr_split
                .next()
                .unwrap()
                .split('=')
                .last()
                .unwrap()
                .parse::<u64>()
                .unwrap()
        };
        let x = parse_next();
        let m = parse_next();
        let a = parse_next();
        let s = parse_next();

        Ok(Part { x, m, a, s })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn evaluate<'a>(&'a self, part: &Part) -> &'a RuleResult {
        for rule in &self.rules {
            let eval = rule.evaluate(part);
            if eval.is_some() {
                return eval.unwrap();
            }
        }
        unreachable!()
    }
}

impl FromStr for Workflow {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_end_matches('}');
        let mut part_iter = s.split('{');

        let name = part_iter.next().unwrap().to_string();

        let rule_str = part_iter.next().unwrap();
        let rules = rule_str.split(',').map(|r| r.parse().unwrap()).collect();

        Ok(Workflow { name, rules })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum RuleResult {
    GoTo { name: String },
    Accepted,
    Rejected,
}

use RuleResult::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Condition {
    attr: char,
    oper: char,
    val: u64,
}

impl FromStr for Condition {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let oper = if s.contains('<') { '<' } else { '>' };
        let mut part_iter = s.split(oper);
        let attr = part_iter.next().unwrap().chars().next().unwrap();
        let val = part_iter.next().unwrap().parse().unwrap();

        Ok(Condition { oper, attr, val })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Rule {
    condition: Option<Condition>,
    result: RuleResult,
}

impl Rule {
    fn evaluate(&self, part: &Part) -> Option<&RuleResult> {
        if self.condition.is_none() {
            return Some(&self.result);
        } else {
            let condition = self.condition.unwrap();
            let inp_val = match condition.attr {
                'x' => part.x,
                'm' => part.m,
                'a' => part.a,
                's' => part.s,
                _ => panic!(),
            };
            let matches = match condition.oper {
                '>' => inp_val > condition.val,
                '<' => inp_val < condition.val,
                _ => panic!(),
            };
            matches.then_some(&self.result)
        }
    }
}

impl FromStr for Rule {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_result = |s| match s {
            "R" => Rejected,
            "A" => Accepted,
            _ => GoTo {
                name: s.to_string(),
            },
        };
        if s.contains(':') {
            let mut part_iter = s.split(':');

            let condition = Some(part_iter.next().unwrap().parse().unwrap());
            let result = parse_result(part_iter.next().unwrap());

            Ok(Rule { condition, result })
        } else {
            let condition = None;
            let result = parse_result(s);

            Ok(Rule { condition, result })
        }
    }
}

fn parse_inp(input: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let mut overall_iter = input.split("\n\n");

    let workflow_part = overall_iter.next().unwrap();
    let part_part = overall_iter.next().unwrap();

    let workflows = workflow_part
        .lines()
        .map(|i| {
            let workflow: Workflow = i.parse().unwrap();
            (workflow.name.clone(), workflow)
        })
        .collect();

    let parts = part_part.lines().map(|l| l.parse().unwrap()).collect();

    (workflows, parts)
}

const INPUT: &str = include_str!("input.dat");
#[allow(dead_code)]
const SAMPLE: &str = include_str!("sample.dat");

fn find_accepted(workflows: &HashMap<String, Workflow>, parts: &[Part]) -> Vec<Part> {
    let mut accepted = Vec::with_capacity(parts.len());

    for part in parts {
        let mut workflow_result = workflows["in"].evaluate(part);

        loop {
            match workflow_result {
                Accepted => {
                    accepted.push(part.clone());
                    break;
                }
                Rejected => {
                    break;
                }
                GoTo { name } => workflow_result = workflows[name].evaluate(part),
            }
        }
    }

    accepted
}

pub fn pt1() {
    let (workflows, parts) = parse_inp(INPUT);

    let accepted = find_accepted(&workflows, &parts);

    let result: u64 = accepted.iter().map(|p| p.x + p.m + p.a + p.s).sum();

    println!("{}", result);
}
