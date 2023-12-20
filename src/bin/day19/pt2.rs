use std::collections::HashMap;
use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
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

impl Condition {
    fn invert(self) -> Condition {
        let (val, oper) = match self.oper {
            '>' => (u64::min(self.val + 1, 4001), '<'),
            '<' => (u64::max(self.val - 1, 0), '>'),
            _ => panic!(),
        };
        Condition {
            attr: self.attr,
            val,
            oper,
        }
    }
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

fn parse_inp(input: &str) -> HashMap<String, Workflow> {
    let mut overall_iter = input.split("\n\n");

    let workflow_part = overall_iter.next().unwrap();

    let workflows = workflow_part
        .lines()
        .map(|i| {
            let workflow: Workflow = i.parse().unwrap();
            (workflow.name.clone(), workflow)
        })
        .collect();

    workflows
}

fn all_paths(
    mut curr_rules: Vec<Condition>,
    workflow: &Workflow,
    workflows: &HashMap<String, Workflow>,
) -> Vec<Vec<Condition>> {
    let mut results = vec![];

    for rule in workflow.rules.iter() {
        let mut poss_rules = curr_rules.clone();
        match rule.condition {
            Some(condition) => {
                poss_rules.push(condition);
                curr_rules.push(condition.invert());
                match &rule.result {
                    Accepted => results.push(poss_rules),
                    Rejected => continue,
                    GoTo { name } => {
                        results.append(&mut all_paths(poss_rules, &workflows[name], workflows))
                    }
                }
            }
            None => match &rule.result {
                Accepted => results.push(poss_rules),
                Rejected => break,
                GoTo { name } => {
                    results.append(&mut all_paths(poss_rules, &workflows[name], workflows))
                }
            },
        }
    }

    results
}

#[derive(Debug, Copy, Clone)]
struct MinMax {
    x: (u32, u32),
    m: (u32, u32),
    a: (u32, u32),
    s: (u32, u32),
}

impl MinMax {
    fn new() -> MinMax {
        let x = (1, 4001);
        let m = (1, 4001);
        let a = (1, 4001);
        let s = (1, 4001);

        MinMax { x, m, a, s }
    }

    fn constrain(&mut self, condition: Condition) {
        let range = match condition.attr {
            'x' => &mut self.x,
            'm' => &mut self.m,
            'a' => &mut self.a,
            's' => &mut self.s,
            _ => panic!(),
        };

        match condition.oper {
            '<' => {
                range.1 = u32::min(range.1, condition.val as u32);
            }
            '>' => {
                range.0 = u32::max(range.0, (condition.val + 1) as u32);
            }
            _ => panic!(),
        }
    }

    fn cardinality(&self) -> u64 {
        (self.x.1 - self.x.0) as u64
            * (self.m.1 - self.m.0) as u64
            * (self.a.1 - self.a.0) as u64
            * (self.s.1 - self.s.0) as u64
    }
}

const INPUT: &str = include_str!("input.dat");
#[allow(dead_code)]
const SAMPLE: &str = include_str!("sample.dat");

pub fn pt2() {
    let workflows = parse_inp(INPUT);

    let in_wkflow = &workflows["in"];
    let all_paths = all_paths(vec![], in_wkflow, &workflows);

    let result: u64 = all_paths
        .iter()
        .map(|condition_list| {
            let mut min_max = MinMax::new();
            condition_list
                .iter()
                .for_each(|condition| min_max.constrain(*condition));
            min_max.cardinality()
        })
        .sum();

    println!("{}", result);
}
