use super::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::convert::Infallible;
use std::str::FromStr;

const INPUT: &str = include_str!("input.dat");
#[allow(dead_code)]
const SAMPLE: &str = include_str!("sample.dat");

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum PulseKind {
    High,
    Low,
}
use PulseKind::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Pulse {
    kind: PulseKind,
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
    conjunction: BTreeMap<String, PulseKind>,
    flip_flop: bool,
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

        let conjunction = BTreeMap::new();
        let flip_flop = false;

        Ok(Module {
            name,
            kind,
            connects_to,
            conjunction,
            flip_flop,
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Coordinator {
    modules: HashMap<String, Module>,
    low_pulses: u64,
    high_pulses: u64,
    done: bool,
}

impl Coordinator {
    fn push_button(&mut self) {
        let mut signals: VecDeque<Pulse> = [Pulse {
            kind: Low,
            from: "button".to_string(),
            to: "broadcaster".to_string(),
        }]
        .into_iter()
        .collect();

        while let Some(next_signal) = signals.pop_front() {
            if next_signal.kind == Low {
                self.low_pulses += 1;
            } else {
                self.high_pulses += 1;
            }

            if next_signal.kind == Low && next_signal.to == "rx" {
                self.done = true;
                return;
            }

            if !self.modules.contains_key(&next_signal.to) {
                continue;
            }

            let dest_mod = self.modules.get_mut(&next_signal.to).unwrap();
            let signal = match dest_mod.kind {
                FlipFlop => {
                    if next_signal.kind == Low {
                        dest_mod.flip_flop = !dest_mod.flip_flop;
                        if dest_mod.flip_flop {
                            Some(High)
                        } else {
                            Some(Low)
                        }
                    } else {
                        None
                    }
                }
                Conjunction => {
                    dest_mod
                        .conjunction
                        .insert(next_signal.from.to_string(), next_signal.kind);
                    if dest_mod.conjunction.values().all(|&k| k == High) {
                        Some(Low)
                    } else {
                        Some(High)
                    }
                }
                Broadcaster => Some(next_signal.kind),
            };
            if let Some(signal) = signal {
                for name in &dest_mod.connects_to {
                    signals.push_back(Pulse {
                        kind: signal,
                        from: dest_mod.name.clone(),
                        to: name.clone(),
                    });
                }
            }
        }
    }
}

impl FromStr for Coordinator {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let modules: Vec<Module> = s.lines().map(|line| line.parse().unwrap()).collect();
        let mut module_map: HashMap<String, Module> = modules
            .clone()
            .into_iter()
            .map(|m| (m.name.clone(), m))
            .collect();

        for module in modules.iter() {
            if module.kind == Conjunction {
                for possible_connections in modules.iter() {
                    if possible_connections.connects_to.contains(&module.name) {
                        let actual_mod = module_map.get_mut(&module.name).unwrap();
                        actual_mod
                            .conjunction
                            .insert(possible_connections.name.clone(), Low);
                    }
                }
            }
        }

        let modules = module_map;

        Ok(Coordinator {
            modules,
            low_pulses: 0,
            high_pulses: 0,
            done: false,
        })
    }
}

pub fn pt1() {
    let mut coordinator: Coordinator = INPUT.parse().unwrap();
    (0..1000).for_each(|_| coordinator.push_button());
    let result = coordinator.low_pulses * coordinator.high_pulses;
    println!("{}", result);
}
