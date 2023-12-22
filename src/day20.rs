use itertools::Itertools;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use regex::Regex;
use std::collections::{HashMap, VecDeque};

use crate::helpers::math::lcm;

type IntType = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    None,
}
#[derive(Debug, Clone)]
struct Module {
    module_type: ModuleType,
    callers: Vec<String>,
    destinations: Vec<String>,
}

#[derive(Debug, Clone)]
struct ModuleContext {
    modules: HashMap<String, Module>,
    flop_map: HashMap<String, bool>,
    conjunct_map: HashMap<String, HashMap<String, bool>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PulseType {
    Low,
    High,
}
#[derive(Debug)]
struct Pulse {
    from: String,
    energy: PulseType,
    to: String,
}

fn parse_file(file: &str) -> ModuleContext {
    const IDENT_RX: &str = r"^([%&]?)(\w+) -> .*$";
    const DESTINATION_RX: &str = r" (\w+)";

    static RE_IDENT: Lazy<Regex> = Lazy::new(|| Regex::new(IDENT_RX).unwrap());
    static RE_DEST: Lazy<Regex> = Lazy::new(|| Regex::new(DESTINATION_RX).unwrap());

    let mut hashmap = HashMap::new();

    file.lines().for_each(|line| {
        let (_, [ty, name]) = RE_IDENT.captures(line).unwrap().extract();
        let destinations: Vec<String> = RE_DEST
            .captures_iter(line)
            .map(|c| c.extract())
            .map(|(_, [dest])| dest.to_string())
            .collect();

        hashmap.insert(
            name.to_string(),
            Module {
                module_type: match ty {
                    "%" => ModuleType::FlipFlop,
                    "&" => ModuleType::Conjunction,
                    _ => ModuleType::None,
                },
                callers: Vec::new(),
                destinations,
            },
        );
    });

    let key_and_values = hashmap
        .iter()
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect_vec();

    let mut flop_map = HashMap::new();
    let mut conjunction_map = HashMap::new();

    for (key, value) in key_and_values.iter() {
        for dest in value.destinations.iter() {
            hashmap.entry(dest.to_string()).and_modify(|module| {
                module.callers.push(key.to_string());
            });
        }
    }

    let key_and_values = hashmap
        .iter()
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect_vec();

    for (key, value) in key_and_values.iter() {
        match value.module_type {
            ModuleType::FlipFlop => {
                flop_map.insert(key.to_string(), false);
            }
            ModuleType::Conjunction => {
                conjunction_map.insert(key.to_string(), HashMap::new());
                for c in value.callers.iter() {
                    conjunction_map
                        .get_mut(key)
                        .unwrap()
                        .insert(c.to_string(), false);
                }
            }
            ModuleType::None => {}
        }
    }

    hashmap.insert(
        String::from("button"),
        Module {
            module_type: ModuleType::None,
            callers: Vec::new(),
            destinations: vec![String::from("broadcaster")],
        },
    );

    ModuleContext {
        modules: hashmap,
        flop_map,
        conjunct_map: conjunction_map,
    }
}

fn inc_pulse_count((low, high): &mut (IntType, IntType), p_type: PulseType) {
    match p_type {
        PulseType::Low => {
            *low += 1;
        }
        PulseType::High => {
            *high += 1;
        }
    }
}

fn push_button(
    module_context: &mut ModuleContext,
    pulse_counts: &mut (IntType, IntType),
    mod_sends_high: &str,
) -> bool {
    let mut stack = VecDeque::new();

    let mut sends_high = false;

    // push low onto broadcaster
    stack.push_back(Pulse {
        energy: PulseType::Low,
        from: String::from("button"),
        to: String::from("broadcaster"),
    });

    // let mut pulse_counts = (1, 0);
    inc_pulse_count(pulse_counts, PulseType::Low);

    // a single round
    while let Some(pulse) = stack.pop_front() {
        if pulse.from == mod_sends_high && pulse.energy == PulseType::High {
            sends_high = true;
        }
        if let Some(m) = &module_context.modules.get(&pulse.to) {
            match m.module_type {
                ModuleType::FlipFlop => {
                    match pulse.energy {
                        PulseType::High => {} // Do nothing
                        PulseType::Low => {
                            let p_type = match module_context.flop_map[&pulse.to] {
                                true => {
                                    *module_context.flop_map.get_mut(&pulse.to).unwrap() = false;
                                    PulseType::Low
                                }
                                false => {
                                    *module_context.flop_map.get_mut(&pulse.to).unwrap() = true;
                                    PulseType::High
                                }
                            };

                            for d in m.destinations.iter() {
                                inc_pulse_count(pulse_counts, p_type);
                                stack.push_back(Pulse {
                                    energy: p_type,
                                    from: pulse.to.to_string(),
                                    to: d.to_string(),
                                })
                            }
                        }
                    }
                }
                ModuleType::Conjunction => {
                    *module_context
                        .conjunct_map
                        .get_mut(&pulse.to)
                        .unwrap()
                        .get_mut(&pulse.from)
                        .unwrap() = match pulse.energy {
                        PulseType::High => true,
                        PulseType::Low => false,
                    };

                    let p_type = if module_context.conjunct_map[&pulse.to]
                        .values()
                        .all(|&x| x == true)
                    {
                        PulseType::Low
                    } else {
                        PulseType::High
                    };
                    for d in m.destinations.iter() {
                        inc_pulse_count(pulse_counts, p_type);
                        stack.push_back(Pulse {
                            energy: p_type,
                            from: pulse.to.to_string(),
                            to: d.to_string(),
                        })
                    }
                }
                ModuleType::None => {
                    for d in m.destinations.iter() {
                        inc_pulse_count(pulse_counts, pulse.energy);
                        stack.push_back(Pulse {
                            energy: pulse.energy,
                            from: pulse.to.to_string(),
                            to: d.to_string(),
                        })
                    }
                }
            }
        }
    }
    sends_high
}

pub fn solve_part_1(file: &str) -> Option<IntType> {
    let mut module_context = parse_file(file);
    let mut pulse_counts = (0, 0);
    for _ in 0..1000 {
        push_button(&mut module_context, &mut pulse_counts, "");
    }
    let (low, high) = pulse_counts;
    Some(low * high)
}

pub fn solve_part_2(file: &str) -> Option<IntType> {
    let init_module_context = parse_file(file);

    let (_, caller_module) = init_module_context
        .modules
        .iter()
        .find(|(_, v)| v.destinations.contains(&String::from("rx")))
        .unwrap();

    assert_eq!(caller_module.module_type, ModuleType::Conjunction);

    // if all are high, then we win
    let key_modules = caller_module
        .callers
        .iter()
        .map(|caller| caller.clone())
        .collect_vec();

    let first_highs: Vec<IntType> = key_modules
        .par_iter()
        .map(|to_check| {
            let mut module_context = init_module_context.clone();
            let mut button_presses = 1;
            while push_button(&mut module_context, &mut (0, 0), &to_check) == false {
                button_presses += 1;
            }
            button_presses
        })
        .collect();

    Some(first_highs.iter().fold(1, |z, &u| lcm(z, u)))
}

const DAY: u8 = 20;

#[allow(unused)]
pub fn main(file: &str) {
    println!("Solving Day {}", DAY);
    println!("  part 1: {:?}", solve_part_1(&file));
    println!("  part 2: {:?}", solve_part_2(&file));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs::{self, InputType};

    #[test]
    fn solves_first_problem() {
        let content = inputs::get_file_part(DAY, InputType::Sample, 1);
        assert_eq!(solve_part_1(&content), Some(32000000));
        let content = inputs::get_file_part(DAY, InputType::Sample, 2);
        assert_eq!(solve_part_1(&content), Some(11687500));
    }

    #[test]
    fn solves_second_problem() {
        // probably shouldn't do this with my input
        let content = inputs::get_file(DAY, InputType::Input);
        assert_eq!(solve_part_2(&content), Some(262775362119547))
    }
}
