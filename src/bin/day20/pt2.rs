use num::Integer;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::convert::Infallible;
use std::str::FromStr;

const INPUT: &str = include_str!("input.dat");
#[allow(dead_code)]
const SAMPLE: &str = include_str!("sample.dat");

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Pulse {
    from: String,
    to: String,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum ModuleKind {
    Broadcaster,
    FlipFlop,
    Conjunction,
}
use ModuleKind::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Module {
    name: String,
    kind: ModuleKind,
    connects_to: BTreeSet<String>,
}

impl FromStr for Module {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mod_parts = s.split("->");

        let mod_id = mod_parts.next().unwrap();
        let (kind, name) = if mod_id.starts_with("broadcaster") {
            (Broadcaster, "broadcaster".to_string())
        } else if mod_id.starts_with('%') {
            let name = mod_id.trim_start_matches('%').trim().to_string();
            (FlipFlop, name)
        } else if mod_id.starts_with('&') {
            let name = mod_id.trim_start_matches('&').trim().to_string();
            (Conjunction, name)
        } else {
            unreachable!();
        };

        let connects_to = mod_parts
            .next()
            .unwrap()
            .split(',')
            .map(|names| names.trim().to_string())
            .collect();

        Ok(Module {
            name,
            kind,
            connects_to,
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Coordinator {
    modules: HashMap<String, Module>,
}

impl FromStr for Coordinator {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let modules = s
            .lines()
            .map(|line| {
                let module: Module = line.parse().unwrap();
                (module.name.clone(), module)
            })
            .collect();

        Ok(Coordinator { modules })
    }
}

fn pointer_tree<'a>(
    coordinator: &'a Coordinator,
    mut visited: HashSet<String>,
    from: &str,
    target: &str,
) -> Vec<VecDeque<Module>> {
    let module = coordinator.modules.get(from).unwrap().clone();
    let mut trees = Vec::new();
    visited.insert(from.to_string());

    for connection in module.connects_to.iter() {
        if visited.contains(connection) {
            continue;
        }
        if connection == target {
            trees.push(VecDeque::from([module.clone()]));
        } else {
            let subtrees = pointer_tree(coordinator, visited.clone(), connection, target);
            for mut subtree in subtrees.into_iter() {
                if subtree.iter().any(|m| m.name == from) {
                    continue;
                }
                subtree.push_front(module.clone());
                trees.push(subtree);
            }
        }
    }

    trees
}

fn _print_pointer_tree(tree: &VecDeque<Module>) {
    let mod_names = tree
        .iter()
        .map(|m| {
            let indicator = match m.kind {
                FlipFlop => "%",
                Conjunction => "&",
                Broadcaster => "",
            };
            format!("{}{}", indicator, m.name)
        })
        .collect::<Vec<_>>();
    println!("{}", mod_names.join(" -> "));
}

fn lcm<I: Iterator<Item = u64>>(inputs: I) -> u64 {
    inputs.fold(1, |acc, x| acc.lcm(&x))
}

fn time_to_low(
    modules: &HashMap<String, Module>,
    pointers: &HashMap<String, HashSet<String>>,
    target: &str,
) -> u64 {
    let target = &modules[target];

    match target.kind {
        Conjunction => {
            let submods = &pointers[target.name.as_str()];
            let high_times = submods
                .iter()
                .map(|submod| time_to_high(modules, pointers, submod));

            lcm(high_times)
        }
        FlipFlop => {
            let pointer = &pointers[target.name.as_str()]
                .iter()
                .cloned()
                .next()
                .unwrap();
            2 * time_to_low(modules, pointers, pointer.as_str())
        }
        Broadcaster => 1,
    }
}

fn time_to_high(
    modules: &HashMap<String, Module>,
    pointers: &HashMap<String, HashSet<String>>,
    target: &str,
) -> u64 {
    let target = &modules[target];

    match target.kind {
        Conjunction => {
            let submods = &pointers[target.name.as_str()];
            submods
                .iter()
                .map(|submod| time_to_high(modules, pointers, submod))
                .sum()
        }
        FlipFlop => {
            let pointer = &pointers[target.name.as_str()]
                .iter()
                .cloned()
                .next()
                .unwrap();
            time_to_low(modules, pointers, pointer.as_str())
        }
        Broadcaster => panic!(),
    }
}

pub fn pt2() {
    let coordinator: Coordinator = INPUT.parse().unwrap();

    let pointers = pointer_tree(&coordinator, HashSet::new(), "broadcaster", "rx");
    // pointers.iter().for_each(_print_pointer_tree);

    let new_target = pointers[0].iter().last().unwrap().name.clone();

    let mut inv_pointers: HashMap<String, HashSet<String>> = HashMap::new();
    for pointer_list in pointers.iter() {
        (1..pointer_list.len()).rev().for_each(|i| {
            let from = &pointer_list[i];
            let to = &pointer_list[i - 1];
            let set = inv_pointers.entry(from.name.clone()).or_default();
            set.insert(to.name.clone());
        });
    }

    let result = time_to_low(&coordinator.modules, &inv_pointers, &new_target);

    println!("{}", result);
}
